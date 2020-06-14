use std::error;
use tensorflow as tf;

pub struct FieldSet {
    pub state_in: tf::Operation,
    pub is_training_in: tf::Operation,
    pub allowed_plies_in: tf::Operation,

    pub init_op: tf::Operation,

    pub probs_out: tf::Operation,
    pub value_out: tf::Operation,
}

impl FieldSet {
    pub fn new(graph: &tf::Graph) -> Result<FieldSet, Box<dyn error::Error>> {
        let result = FieldSet {
            state_in: graph.operation_by_name_required("state_in")?,
            is_training_in: graph.operation_by_name_required("is_training_in")?,
            allowed_plies_in: graph.operation_by_name_required("allowed_plies_in")?,

            init_op: graph.operation_by_name_required("init_op")?,

            probs_out: graph.operation_by_name_required("probs_out")?,
            value_out: graph.operation_by_name_required("value_out")?,
        };
        Ok(result)
    }
}
