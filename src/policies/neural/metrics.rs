#[derive(serde::Deserialize, serde::Serialize)]
pub struct Metrics {
    pub value_loss: f32,
    pub policy_loss: f32,
}
