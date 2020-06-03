use std::error;
use tensorflow as tf;

pub struct Fields {
    pub state_in: tf::Operation,
    pub allowed_plies_in: tf::Operation,
    pub actions_in: tf::Operation,
    pub rewards_in: tf::Operation,
    pub filepath_in: tf::Operation,

    pub init_op: tf::Operation,
    pub update_batch_op: tf::Operation,
    pub save_op: tf::Operation,
    pub restore_op: tf::Operation,

    pub argmax_action_out: tf::Operation,
    pub stochastic_action_out: tf::Operation,
    pub probabilities_out: tf::Operation,
    pub policy_loss_out: tf::Operation,
    pub reg_losses_out: tf::Operation,
    pub total_loss_out: tf::Operation,
}

impl Fields {
    pub fn new(graph: &tf::Graph) -> Result<Fields, Box<dyn error::Error>> {
        let result = Fields {
            state_in: graph.operation_by_name_required("state_in")?,
            allowed_plies_in: graph.operation_by_name_required("allowed_plies_in")?,
            actions_in: graph.operation_by_name_required("actions_in")?,
            rewards_in: graph.operation_by_name_required("rewards_in")?,
            filepath_in: graph.operation_by_name_required("save/Const")?,

            init_op: graph.operation_by_name_required("init_op")?,
            update_batch_op: graph.operation_by_name_required("update_batch_op")?,
            save_op: graph.operation_by_name_required("save/control_dependency")?,
            restore_op: graph.operation_by_name_required("save/restore_all")?,

            argmax_action_out: graph.operation_by_name_required("argmax_action_out")?,
            stochastic_action_out: graph
                .operation_by_name_required("stochastic_action_out/Multinomial")?,
            probabilities_out: graph.operation_by_name_required("probabilities_out")?,
            policy_loss_out: graph.operation_by_name_required("policy_loss_out")?,
            reg_losses_out: graph.operation_by_name_required("reg_losses_out")?,
            total_loss_out: graph.operation_by_name_required("total_loss_out")?,
        };
        Ok(result)
    }
}
