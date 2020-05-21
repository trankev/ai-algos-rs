use std::error;
use std::fs;
use std::io::Read;
use std::path;
use tensorflow as tf;

pub struct ProbabilityLearning {
    graph: tf::Graph,
    session: tf::Session,
}

impl ProbabilityLearning {
    pub fn from_file<P: AsRef<path::Path>>(
        filename: P,
    ) -> Result<ProbabilityLearning, Box<dyn error::Error>> {
        let mut graph = tf::Graph::new();
        let mut proto = Vec::new();
        fs::File::open(filename)?.read_to_end(&mut proto)?;
        graph.import_graph_def(&proto, &tf::ImportGraphDefOptions::new())?;
        let session = tf::Session::new(&tf::SessionOptions::new(), &graph)?;
        Ok(ProbabilityLearning { graph, session })
    }

    pub fn initialize(&self) -> Result<(), Box<dyn error::Error>> {
        let mut run_args = tf::SessionRunArgs::new();
        let init = self.graph.operation_by_name_required("init")?;
        run_args.add_target(&init);
        self.session.run(&mut run_args)?;
        Ok(())
    }

    pub fn play(&self) -> Result<i64, Box<dyn error::Error>> {
        let mut run_args = tf::SessionRunArgs::new();
        let chosen_action = self
            .graph
            .operation_by_name_required("chosen_action/Multinomial")?;
        let action_fetch = run_args.request_fetch(&chosen_action, 0);
        self.session.run(&mut run_args)?;
        let action = run_args.fetch::<i64>(action_fetch)?[0];
        Ok(action)
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

    pub fn get_probabilities(&self) -> Result<Vec<f32>, Box<dyn error::Error>> {
        let mut result = Vec::new();
        let mut run_args = tf::SessionRunArgs::new();
        let probabilities = self.graph.operation_by_name_required("probabilities")?;
        let probabilities_fetch = run_args.request_fetch(&probabilities, 0);
        self.session.run(&mut run_args)?;
        let probabilities_value = run_args.fetch::<f32>(probabilities_fetch)?;
        for i in 0..4 {
            result.push(probabilities_value[i]);
        }
        Ok(result)
    }
}
