use std::error;
use tensorflow as tf;

pub struct FieldSet {
    pub state_in: tf::Operation,
    pub is_training_in: tf::Operation,
    pub allowed_plies_in: tf::Operation,
    pub target_pis_in: tf::Operation,
    pub target_value_in: tf::Operation,
    pub filepath_in: tf::Operation,

    pub init_op: tf::Operation,
    pub train_op: tf::Operation,
    pub save_op: tf::Operation,
    pub load_op: tf::Operation,

    pub probs_out: tf::Operation,
    pub value_out: tf::Operation,
    pub pi_loss_out: tf::Operation,
    pub value_loss_out: tf::Operation,
    pub total_loss_out: tf::Operation,
}

impl FieldSet {
    pub fn new(graph: &tf::Graph) -> Result<FieldSet, Box<dyn error::Error>> {
        let result = FieldSet {
            state_in: graph.operation_by_name_required("state_in")?,
            is_training_in: graph.operation_by_name_required("is_training_in")?,
            allowed_plies_in: graph.operation_by_name_required("allowed_plies_in")?,
            target_pis_in: graph.operation_by_name_required("target_pis_in")?,
            target_value_in: graph.operation_by_name_required("target_value_in")?,
            filepath_in: graph.operation_by_name_required("save/Const")?,

            init_op: graph.operation_by_name_required("init_op")?,
            train_op: graph.operation_by_name_required("train_op")?,
            save_op: graph.operation_by_name_required("save/control_dependency")?,
            load_op: graph.operation_by_name_required("save/restore_all")?,

            probs_out: graph.operation_by_name_required("probs_out")?,
            value_out: graph.operation_by_name_required("value_out")?,
            pi_loss_out: graph.operation_by_name_required("softmax_cross_entropy_loss/value")?,
            value_loss_out: graph.operation_by_name_required("mean_squared_error/value")?,
            total_loss_out: graph.operation_by_name_required("total_loss_out")?,
        };
        Ok(result)
    }
}
