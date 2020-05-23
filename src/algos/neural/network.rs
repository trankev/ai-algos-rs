use std::error;
use std::fs;
use std::io::Read;
use std::path;
use tensorflow as tf;

pub struct Network {
    graph: tf::Graph,
    session: tf::Session,
    state_size: u64,
    action_count: u64,
}

impl Network {
    pub fn from_file<P: AsRef<path::Path>>(
        filename: P,
        state_size: u64,
        action_count: u64,
    ) -> Result<Network, Box<dyn error::Error>> {
        let mut graph = tf::Graph::new();
        let mut proto = Vec::new();
        fs::File::open(filename)?.read_to_end(&mut proto)?;
        graph.import_graph_def(&proto, &tf::ImportGraphDefOptions::new())?;
        let session = tf::Session::new(&tf::SessionOptions::new(), &graph)?;
        Ok(Network {
            graph,
            session,
            state_size,
            action_count,
        })
    }

    pub fn initialize(&self) -> Result<(), Box<dyn error::Error>> {
        let mut run_args = tf::SessionRunArgs::new();
        let init = self.graph.operation_by_name_required("init")?;
        run_args.add_target(&init);
        self.session.run(&mut run_args)?;
        Ok(())
    }

    pub fn play(
        &self,
        state: &Vec<f32>,
        allowed_plies: &Vec<f32>,
    ) -> Result<i32, Box<dyn error::Error>> {
        let state_value = tf::Tensor::new(&[1, self.state_size][..]).with_values(&state)?;
        let state_in = self.graph.operation_by_name_required("state_in")?;
        let allowed_plies_value =
            tf::Tensor::new(&[1, self.action_count][..]).with_values(&allowed_plies)?;
        let allowed_plies_in = self.graph.operation_by_name_required("allowed_plies")?;
        let chosen_action = self.graph.operation_by_name_required("chosen_action")?;

        let mut run_args = tf::SessionRunArgs::new();
        run_args.add_feed(&state_in, 0, &state_value);
        run_args.add_feed(&allowed_plies_in, 0, &allowed_plies_value);
        let action_fetch = run_args.request_fetch(&chosen_action, 0);
        self.session.run(&mut run_args)?;
        let action = run_args.fetch::<i64>(action_fetch)?[0] as i32;
        Ok(action)
    }

    pub fn get_probabilities(
        &self,
        state: &Vec<f32>,
        allowed_plies: &Vec<f32>,
    ) -> Result<Vec<f32>, Box<dyn error::Error>> {
        let state_value = tf::Tensor::new(&[1, self.state_size][..]).with_values(&state)?;
        let state_in = self.graph.operation_by_name_required("state_in")?;
        let allowed_plies_value =
            tf::Tensor::new(&[1, self.action_count][..]).with_values(&allowed_plies)?;
        let allowed_plies_in = self.graph.operation_by_name_required("allowed_plies")?;
        let probabilities = self.graph.operation_by_name_required("probabilities")?;

        let mut run_args = tf::SessionRunArgs::new();
        run_args.add_feed(&state_in, 0, &state_value);
        run_args.add_feed(&allowed_plies_in, 0, &allowed_plies_value);
        let probabilities_fetch = run_args.request_fetch(&probabilities, 0);
        self.session.run(&mut run_args)?;

        let mut result = Vec::new();
        let probabilities_value = run_args.fetch::<f32>(probabilities_fetch)?;
        for i in 0..probabilities_value.dims()[1] {
            result.push(probabilities_value[i as usize]);
        }
        Ok(result)
    }

    pub fn learn(&self, action: i32, reward: f32) -> Result<(), Box<dyn error::Error>> {
        let action_value = tf::Tensor::new(&[1][..]).with_values(&[action])?;
        let reward_value = tf::Tensor::new(&[1][..]).with_values(&[reward])?;
        let mut run_args = tf::SessionRunArgs::new();
        let minimize = self.graph.operation_by_name_required("minimize")?;
        run_args.add_target(&minimize);
        let action_holder = self.graph.operation_by_name_required("action_holder")?;
        let reward_holder = self.graph.operation_by_name_required("reward_holder")?;
        run_args.add_feed(&action_holder, 0, &action_value);
        run_args.add_feed(&reward_holder, 0, &reward_value);
        self.session.run(&mut run_args)?;
        Ok(())
    }

    pub fn save(&self, path: String) -> Result<(), Box<dyn error::Error>> {
        let op_filepath = self.graph.operation_by_name_required("save/Const")?;
        let op_save = self
            .graph
            .operation_by_name_required("save/control_dependency")?;
        let filepath_tensor = tf::Tensor::from(path);
        let mut run_args = tf::SessionRunArgs::new();
        run_args.add_target(&op_save);
        run_args.add_feed(&op_filepath, 0, &filepath_tensor);
        self.session.run(&mut run_args)?;
        Ok(())
    }

    pub fn load(&self, path: String) -> Result<(), Box<dyn error::Error>> {
        let op_filepath = self.graph.operation_by_name_required("save/Const")?;
        let op_load = self.graph.operation_by_name_required("save/restore_all")?;
        let filepath_tensor = tf::Tensor::from(path);
        let mut run_args = tf::SessionRunArgs::new();
        run_args.add_target(&op_load);
        run_args.add_feed(&op_filepath, 0, &filepath_tensor);
        self.session.run(&mut run_args)?;
        Ok(())
    }
}
