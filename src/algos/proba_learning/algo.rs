use tensorflow as tf;
use tensorflow::ops as tf_ops;
use tensorflow::train as tf_train;
use tensorflow::train::Optimizer;

pub struct ProbabilityLearning {
    session: tf::Session,
    probabilities: tf::Operation,
    chosen_action: tf::Operation,
    action_holder: tf::Operation,
    reward_holder: tf::Operation,
    minimize: tf::Operation,
    loss: tf::Operation,
}

impl ProbabilityLearning {
    pub fn new(action_count: u64) -> Result<ProbabilityLearning, tf::Status> {
        let mut scope = tf::Scope::new_root_scope();
        let (weights, probabilities, chosen_action) = build_model(&scope, action_count)?;
        let (minimizer_vars, action_holder, reward_holder, loss, minimize) =
            setup_training(&scope, weights.clone())?;
        let options = tf::SessionOptions::new();
        let graph = scope.graph_mut();
        let session = tf::Session::new(&options, &graph)?;
        initialize(&session, &weights, &minimizer_vars)?;
        Ok(ProbabilityLearning {
            session,
            probabilities,
            chosen_action,
            action_holder,
            reward_holder,
            minimize,
            loss,
        })
    }

    pub fn play(&self) -> Result<i64, tf::Status> {
        let mut run_args = tf::SessionRunArgs::new();
        let action_fetch = run_args.request_fetch(&self.chosen_action, 0);
        self.session.run(&mut run_args)?;
        let action = run_args.fetch::<i64>(action_fetch)?[0];
        Ok(action)
    }

    pub fn learn(&self, action: i64, reward: f64) -> Result<f64, tf::Status> {
        let action_value = tf::Tensor::new(&[1][..]).with_values(&[action as i32])?;
        let reward_value = tf::Tensor::new(&[1][..]).with_values(&[reward])?;
        let mut run_args = tf::SessionRunArgs::new();
        run_args.add_target(&self.minimize);
        run_args.add_feed(&self.action_holder, 0, &action_value);
        run_args.add_feed(&self.reward_holder, 0, &reward_value);
        let loss_fetch = run_args.request_fetch(&self.loss, 0);
        self.session.run(&mut run_args)?;
        let loss: f64 = run_args.fetch(loss_fetch)?[0];
        Ok(loss)
    }

    pub fn get_probabilities(&self) -> Result<Vec<f64>, tf::Status> {
        let mut result = Vec::new();
        let mut run_args = tf::SessionRunArgs::new();
        let probabilities_fetch = run_args.request_fetch(&self.probabilities, 0);
        self.session.run(&mut run_args)?;
        let probabilities_value = run_args.fetch::<f64>(probabilities_fetch)?;
        for i in 0..4 {
            result.push(probabilities_value[i]);
        }
        Ok(result)
    }
}

fn initialize(
    session: &tf::Session,
    weights: &tf::Variable,
    minimizer_vars: &Vec<tf::Variable>,
) -> Result<(), tf::Status> {
    let mut run_args = tf::SessionRunArgs::new();
    run_args.add_target(weights.initializer());
    for var in minimizer_vars {
        run_args.add_target(var.initializer());
    }
    session.run(&mut run_args)?;
    Ok(())
}

fn setup_training(
    scope: &tf::Scope,
    weights: tf::Variable,
) -> Result<
    (
        Vec<tf::Variable>,
        tf::Operation,
        tf::Operation,
        tf::Operation,
        tf::Operation,
    ),
    tf::Status,
> {
    let mut scope = scope.new_sub_scope("training");
    let reward_holder = tf_ops::Placeholder::new()
        .dtype(tf::DataType::Double)
        .shape(tf::Shape::from(&[1u64][..]))
        .build(&mut scope.with_op_name("reward_holder"))?;
    let action_holder = tf_ops::Placeholder::new()
        .dtype(tf::DataType::Int32)
        .shape(tf::Shape::from(&[1u64][..]))
        .build(&mut scope.with_op_name("action_holder"))?;
    let slice_size = tf_ops::constant(&[1][..], &mut scope)?;
    let responsible_weight = tf_ops::slice(
        tf_ops::squeeze(weights.output().clone(), &mut scope)?.into(),
        action_holder.clone().into(),
        slice_size.into(),
        &mut scope,
    )?;
    let log = tf_ops::log(responsible_weight.into(), &mut scope)?;
    let neg = tf_ops::neg(log.into(), &mut scope)?;
    let loss = tf_ops::mul(
        neg.into(),
        reward_holder.clone().into(),
        &mut scope.with_op_name("loss"),
    )?;
    let learning_rate = tf_ops::constant(0.001f64, &mut scope)?;
    let optimizer = tf_train::GradientDescentOptimizer::new(learning_rate);
    let (minimizer_vars, minimize) = optimizer.minimize(
        &mut scope.with_op_name("minimize"),
        loss.clone().into(),
        tf_train::MinimizeOptions::default().with_variables(&[weights]),
    )?;
    Ok((minimizer_vars, action_holder, reward_holder, loss, minimize))
}

fn build_model(
    scope: &tf::Scope,
    choice_count: u64,
) -> Result<(tf::Variable, tf::Operation, tf::Operation), tf::Status> {
    let mut scope = scope.new_sub_scope("model");
    let mut initial_value = tf::Tensor::new(&[1, choice_count][..]);
    for index in 0..choice_count {
        initial_value.set(&[0, index], 1.0);
    }
    let weights = tf::Variable::builder()
        .initial_value(tf_ops::constant(initial_value, &mut scope)?)
        .shape(tf::Shape::from(&[1, choice_count as u64][..]))
        .data_type(tf::DataType::Double)
        .build(&mut scope.with_op_name("weights"))?;

    let probabilities = tf_ops::softmax(
        weights.output().clone(),
        &mut scope.with_op_name("probabilities"),
    )?;

    let chosen_action = tf_ops::squeeze(
        tf_ops::multinomial(
            tf_ops::log_softmax(weights.output().clone(), &mut scope)?.into(),
            tf_ops::constant(1, &mut scope)?.into(),
            &mut scope,
        )?
        .into(),
        &mut scope.with_op_name("chosen_action"),
    )?;

    Ok((weights, probabilities, chosen_action))
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand;
    use rand::Rng;
    use std::error;

    const REWARDS: [[f64; 4]; 4] = [
        [0.0, -1.0, 1.0, -1.0],
        [1.0, 0.0, -1.0, 1.0],
        [-1.0, 1.0, 0.0, -1.0],
        [1.0, -1.0, 1.0, 0.0],
    ];

    #[test]
    fn test_build_graph() -> Result<(), Box<dyn error::Error>> {
        let agent = ProbabilityLearning::new(4)?;
        let mut rng = rand::thread_rng();
        let e = 0.1;
        let mut get_action = || {
            if rng.gen_range(0.0, 1.0) < e {
                Ok(rng.gen_range(0, 4))
            } else {
                agent.play()
            }
        };
        let action1 = get_action()?;
        let action2 = get_action()?;
        let reward = REWARDS[action1 as usize][action2 as usize];
        agent.learn(action1, reward)?;
        agent.learn(action2, -reward)?;
        Ok(())
    }
}
