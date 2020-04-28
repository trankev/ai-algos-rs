use tensorflow as tf;
use tensorflow::ops as tf_ops;
use tensorflow::train as tf_train;
use tensorflow::train::Optimizer;

pub struct NeuralNetwork {
    session: tf::Session,
    input: tf::Operation,
    probabilities: tf::Operation,
    chosen_action: tf::Operation,
    action_holder: tf::Operation,
    reward_holder: tf::Operation,
    minimize: tf::Operation,
    loss: tf::Operation,
}

type InputType = tf::Tensor<f64>;

impl NeuralNetwork {
    pub fn new(dimensions: &[u64]) -> Result<NeuralNetwork, tf::Status> {
        let mut scope = tf::Scope::new_root_scope();
        let (variables, input, output_layer, chosen_action) = build_model(&scope, dimensions)?;
        let (minimizer_vars, action_holder, reward_holder, loss, minimize) =
            setup_training(&scope, &output_layer, &variables)?;
        let options = tf::SessionOptions::new();
        let graph = scope.graph_mut();
        let session = tf::Session::new(&options, &graph)?;
        initialize(&session, &variables, &minimizer_vars)?;
        Ok(NeuralNetwork {
            session,
            input,
            probabilities: output_layer,
            chosen_action,
            action_holder,
            reward_holder,
            minimize,
            loss,
        })
    }

    pub fn build_input(&self, state: &[usize]) -> InputType {
        let mut input_tensor = tf::Tensor::new(&[1, 19]);
        for index in state {
            input_tensor[*index] = 1.0;
        }
        input_tensor
    }

    pub fn play(&self, input_tensor: &InputType) -> Result<i64, tf::Status> {
        let mut run_args = tf::SessionRunArgs::new();
        let action_fetch = run_args.request_fetch(&self.chosen_action, 0);
        run_args.add_feed(&self.input, 0, &input_tensor);
        self.session.run(&mut run_args)?;
        let action = run_args.fetch::<i64>(action_fetch)?[0];
        Ok(action)
    }

    pub fn learn(
        &self,
        input_tensor: &InputType,
        action: i64,
        reward: f64,
    ) -> Result<f64, tf::Status> {
        let action_value = tf::Tensor::new(&[1][..]).with_values(&[action as i32])?;
        let reward_value = tf::Tensor::new(&[1][..]).with_values(&[reward])?;
        let mut run_args = tf::SessionRunArgs::new();
        run_args.add_target(&self.minimize);
        run_args.add_feed(&self.input, 0, &input_tensor);
        run_args.add_feed(&self.action_holder, 0, &action_value);
        run_args.add_feed(&self.reward_holder, 0, &reward_value);
        let loss_fetch = run_args.request_fetch(&self.loss, 0);
        self.session.run(&mut run_args)?;
        let loss: f64 = run_args.fetch(loss_fetch)?[0];
        Ok(loss)
    }

    pub fn get_probabilities(&self, input_tensor: &InputType) -> Result<Vec<f64>, tf::Status> {
        let mut result = Vec::new();
        let mut run_args = tf::SessionRunArgs::new();
        let probabilities_fetch = run_args.request_fetch(&self.probabilities, 0);
        run_args.add_feed(&self.input, 0, &input_tensor);
        self.session.run(&mut run_args)?;
        let probabilities_value = run_args.fetch::<f64>(probabilities_fetch)?;
        let size = probabilities_value.dims()[1] as usize;
        for i in 0..size {
            result.push(probabilities_value[i]);
        }
        Ok(result)
    }
}

fn initialize(
    session: &tf::Session,
    model_variables: &Vec<tf::Variable>,
    minimizer_variables: &Vec<tf::Variable>,
) -> Result<(), tf::Status> {
    let mut run_args = tf::SessionRunArgs::new();
    for var in model_variables {
        run_args.add_target(var.initializer());
    }
    for var in minimizer_variables {
        run_args.add_target(var.initializer());
    }
    session.run(&mut run_args)?;
    Ok(())
}

fn setup_training(
    scope: &tf::Scope,
    output_layer: &tf::Operation,
    variables: &Vec<tf::Variable>,
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
        tf_ops::squeeze(output_layer.clone().into(), &mut scope)?.into(),
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
        tf_train::MinimizeOptions::default().with_variables(&variables),
    )?;
    Ok((minimizer_vars, action_holder, reward_holder, loss, minimize))
}

fn build_model(
    scope: &tf::Scope,
    dimensions: &[u64],
) -> Result<
    (
        Vec<tf::Variable>,
        tf::Operation,
        tf::Operation,
        tf::Operation,
    ),
    tf::Status,
> {
    let mut scope = scope.new_sub_scope("model");
    let input = tf_ops::Placeholder::new()
        .dtype(tf::DataType::Double)
        .shape(tf::Shape::from(&[1u64, dimensions[0]][..]))
        .build(&mut scope.with_op_name("input"))?;
    let mut variables = Vec::new();
    let layer = input.clone();
    for index in 0..(dimensions.len() - 2) {
        let (vars, _layer) = build_layer(
            &scope,
            layer.clone(),
            dimensions[index],
            dimensions[index + 1],
            &|x, scope| Ok(tf_ops::tanh(x, scope)?),
        )?;
        variables.extend(vars);
    }
    let last_index = dimensions.len() - 2;
    let (vars, layer) = build_layer(
        &scope,
        layer.clone(),
        dimensions[last_index],
        dimensions[last_index + 1],
        &|x, scope| Ok(tf_ops::softmax(x, scope)?),
    )?;
    variables.extend(vars);
    let chosen_action = tf_ops::arg_max(
        layer.clone().into(),
        tf_ops::constant(1, &mut scope)?.into(),
        &mut scope.with_op_name("chosen_action"),
    )?;
    Ok((variables, input, layer, chosen_action))
}

fn build_layer(
    scope: &tf::Scope,
    input: tf::Operation,
    input_size: u64,
    output_size: u64,
    activation: &dyn Fn(tf::Output, &mut tf::Scope) -> Result<tf::Operation, tf::Status>,
) -> Result<(Vec<tf::Variable>, tf::Operation), tf::Status> {
    let mut scope = scope.new_sub_scope("layer");
    let weight_shape = tf_ops::constant(&[input_size as i64, output_size as i64][..], &mut scope)?;
    let weights = tf::Variable::builder()
        .initial_value(
            tf_ops::RandomStandardNormal::new()
                .dtype(tf::DataType::Double)
                .build(weight_shape.into(), &mut scope)?,
        )
        .data_type(tf::DataType::Double)
        .shape(tf::Shape::from(&[input_size, output_size][..]))
        .build(&mut scope)?;
    let biases = tf::Variable::builder()
        .const_initial_value(tf::Tensor::<f64>::new(&[output_size]))
        .build(&mut scope)?;
    Ok((
        vec![weights.clone(), biases.clone()],
        activation(
            tf_ops::add(
                tf_ops::mat_mul(input.into(), weights.output().clone(), &mut scope)?.into(),
                biases.output().clone(),
                &mut scope,
            )?
            .into(),
            &mut scope,
        )?,
    ))
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::error;

    #[test]
    fn test_build_graph() -> Result<(), Box<dyn error::Error>> {
        let nn = NeuralNetwork::new(&[19, 9])?;
        let input = nn.build_input(&[0]);
        let action = nn.play(&input)?;
        nn.learn(&input, action, 1.0)?;
        nn.get_probabilities(&input)?;
        Ok(())
    }
}
