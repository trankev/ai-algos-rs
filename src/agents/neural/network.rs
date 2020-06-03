use super::actions;
use super::fieldset;
use super::learning_metrics;
use super::replay_buffer;
use std::error;
use std::fs;
use std::io::Read;
use std::path;
use tensorflow as tf;

pub struct Network {
    session: tf::Session,
    state_size: u64,
    action_count: u64,
    action_choosing: actions::ActionChoosing,
    fields: fieldset::Fields,
}

impl Network {
    pub fn from_file<P: AsRef<path::Path>>(
        filename: P,
        state_size: u64,
        action_count: u64,
        action_choosing: actions::ActionChoosing,
    ) -> Result<Network, Box<dyn error::Error>> {
        let mut graph = tf::Graph::new();
        let mut proto = Vec::new();
        fs::File::open(filename)?.read_to_end(&mut proto)?;
        graph.import_graph_def(&proto, &tf::ImportGraphDefOptions::new())?;
        let session = tf::Session::new(&tf::SessionOptions::new(), &graph)?;
        let fields = fieldset::Fields::new(&graph)?;
        Ok(Network {
            session,
            state_size,
            action_count,
            action_choosing,
            fields,
        })
    }

    pub fn initialize(&self) -> Result<(), Box<dyn error::Error>> {
        let mut run_args = tf::SessionRunArgs::new();
        run_args.add_target(&self.fields.init_op);
        self.session.run(&mut run_args)?;
        Ok(())
    }

    pub fn play(
        &self,
        state: &Vec<f32>,
        allowed_plies: &Vec<f32>,
    ) -> Result<i32, Box<dyn error::Error>> {
        let state_value = tf::Tensor::new(&[1, self.state_size][..]).with_values(&state)?;
        let allowed_plies_value =
            tf::Tensor::new(&[1, self.action_count][..]).with_values(&allowed_plies)?;
        let chosen_action = match self.action_choosing {
            actions::ActionChoosing::Deterministic => &self.fields.argmax_action_out,
            actions::ActionChoosing::Stochastic => &self.fields.stochastic_action_out,
        };

        let mut run_args = tf::SessionRunArgs::new();
        run_args.add_feed(&self.fields.state_in, 0, &state_value);
        run_args.add_feed(&self.fields.allowed_plies_in, 0, &allowed_plies_value);
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
        let allowed_plies_value =
            tf::Tensor::new(&[1, self.action_count][..]).with_values(&allowed_plies)?;

        let mut run_args = tf::SessionRunArgs::new();
        run_args.add_feed(&self.fields.state_in, 0, &state_value);
        run_args.add_feed(&self.fields.allowed_plies_in, 0, &allowed_plies_value);
        let probabilities_fetch = run_args.request_fetch(&self.fields.probabilities_out, 0);
        self.session.run(&mut run_args)?;

        let mut result = Vec::new();
        let probabilities_value = run_args.fetch::<f32>(probabilities_fetch)?;
        for i in 0..probabilities_value.dims()[1] {
            result.push(probabilities_value[i as usize]);
        }
        Ok(result)
    }

    pub fn learn(
        &self,
        buffer: &replay_buffer::ReplayBuffer,
    ) -> Result<learning_metrics::LearningMetrics, Box<dyn error::Error>> {
        let action_value =
            tf::Tensor::new(&[buffer.plies.len() as u64][..]).with_values(&buffer.plies)?;
        let reward_value =
            tf::Tensor::new(&[buffer.rewards.len() as u64][..]).with_values(&buffer.rewards)?;
        let state_value = tf::Tensor::new(&[buffer.plies.len() as u64, self.state_size][..])
            .with_values(&buffer.states)?;
        let allowed_plies_value =
            tf::Tensor::new(&[buffer.plies.len() as u64, self.action_count][..])
                .with_values(&buffer.allowed_plies)?;
        let mut run_args = tf::SessionRunArgs::new();
        run_args.add_target(&self.fields.update_batch_op);
        run_args.add_feed(&self.fields.actions_in, 0, &action_value);
        run_args.add_feed(&self.fields.rewards_in, 0, &reward_value);
        run_args.add_feed(&self.fields.state_in, 0, &state_value);
        run_args.add_feed(&self.fields.allowed_plies_in, 0, &allowed_plies_value);
        let policy_loss_fetch = run_args.request_fetch(&self.fields.policy_loss_out, 0);
        let reg_losses_fetch = run_args.request_fetch(&self.fields.reg_losses_out, 0);
        let total_loss_fetch = run_args.request_fetch(&self.fields.total_loss_out, 0);
        self.session.run(&mut run_args)?;
        let metrics = learning_metrics::LearningMetrics {
            policy_loss: run_args.fetch::<f32>(policy_loss_fetch)?[0],
            reg_losses: run_args.fetch::<f32>(reg_losses_fetch)?[0],
            total_loss: run_args.fetch::<f32>(total_loss_fetch)?[0],
        };
        Ok(metrics)
    }

    pub fn save(&self, path: String) -> Result<(), Box<dyn error::Error>> {
        let filepath_tensor = tf::Tensor::from(path);
        let mut run_args = tf::SessionRunArgs::new();
        run_args.add_target(&self.fields.save_op);
        run_args.add_feed(&self.fields.filepath_in, 0, &filepath_tensor);
        self.session.run(&mut run_args)?;
        Ok(())
    }

    pub fn load(&self, path: String) -> Result<(), Box<dyn error::Error>> {
        let filepath_tensor = tf::Tensor::from(path);
        let mut run_args = tf::SessionRunArgs::new();
        run_args.add_target(&self.fields.restore_op);
        run_args.add_feed(&self.fields.filepath_in, 0, &filepath_tensor);
        self.session.run(&mut run_args)?;
        Ok(())
    }
}
