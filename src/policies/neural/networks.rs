use super::fieldsets;
use super::samples;
use std::error;
use std::fs;
use std::io::Read;
use std::path;
use tensorflow as tf;

const MODEL_FILENAME: &str = "model.pb";

pub struct Network {
    state_dimensions: Vec<u64>,
    session: tf::Session,
    fields: fieldsets::FieldSet,
    ply_count: u64,
}

impl Network {
    pub fn new<P: AsRef<path::Path>>(
        path: P,
        state_in_dimensions: &[usize],
        ply_count: usize,
    ) -> Result<Network, Box<dyn error::Error>> {
        let mut graph = tf::Graph::new();
        let mut proto = Vec::new();
        let model_file = path.as_ref().join(MODEL_FILENAME);
        fs::File::open(model_file)?.read_to_end(&mut proto)?;
        graph.import_graph_def(&proto, &tf::ImportGraphDefOptions::new())?;
        let session = tf::Session::new(&tf::SessionOptions::new(), &graph)?;
        let fields = fieldsets::FieldSet::new(&graph)?;
        let mut state_dimensions = vec![1];
        state_dimensions.extend(state_in_dimensions.iter().map(|x| *x as u64));
        let network = Network {
            session,
            fields,
            state_dimensions,
            ply_count: ply_count as u64,
        };
        Ok(network)
    }

    pub fn initialize(&self) -> Result<(), Box<dyn error::Error>> {
        let mut run_args = tf::SessionRunArgs::new();
        run_args.add_target(&self.fields.init_op);
        self.session.run(&mut run_args)?;
        Ok(())
    }

    pub fn predict(
        &self,
        state: &Vec<f32>,
        allowed_plies: &Vec<f32>,
    ) -> Result<(f32, Vec<f32>), Box<dyn error::Error>> {
        let state_value = tf::Tensor::new(&self.state_dimensions[..]).with_values(&state)?;
        let allowed_plies_value =
            tf::Tensor::new(&[1, self.ply_count][..]).with_values(&allowed_plies)?;
        let training_value = tf::Tensor::new(&[][..]).with_values(&[false])?;
        let mut run_args = tf::SessionRunArgs::new();
        run_args.add_feed(&self.fields.state_in, 0, &state_value);
        run_args.add_feed(&self.fields.allowed_plies_in, 0, &allowed_plies_value);
        run_args.add_feed(&self.fields.is_training_in, 0, &training_value);
        let probs_fetch = run_args.request_fetch(&self.fields.probs_out, 0);
        let value_fetch = run_args.request_fetch(&self.fields.value_out, 0);
        self.session.run(&mut run_args)?;
        let value_value = run_args.fetch::<f32>(value_fetch)?[0];
        let probs_value = run_args.fetch::<f32>(probs_fetch)?;
        let mut probabilities = Vec::new();
        for i in 0..probs_value.dims()[1] {
            probabilities.push(probs_value[i as usize]);
        }
        Ok((value_value, probabilities))
    }

    pub fn save<P: AsRef<path::Path>>(&self, project_folder: P) -> Result<(), Box<dyn error::Error>> {
        let variables_folder = project_folder.as_ref().join(VARIABLES_FOLDER);
        let variables_folder = String::from(variables_folder.to_str().unwrap());
        let filepath_tensor = tf::Tensor::from(variables_folder);
        let mut run_args = tf::SessionRunArgs::new();
        run_args.add_target(&self.fields.save_op);
        run_args.add_feed(&self.fields.filepath_in, 0, &filepath_tensor);
        self.session.run(&mut run_args)?;
        Ok(())
    }
}
