pub struct TrainSample {
    pub size: u64,
    pub states: Vec<f32>,
    pub values: Vec<f32>,
    pub predictions: Vec<f32>,
}

impl TrainSample {
    pub fn new() -> TrainSample {
        TrainSample {
            size: 0,
            states: Vec::new(),
            values: Vec::new(),
            predictions: Vec::new(),
        }
    }

    pub fn add(&mut self, state: &Vec<f32>, value: f32, predictions: &Vec<f32>) {
        self.states.extend(state);
        self.values.push(value);
        self.predictions.extend(predictions);
        self.size += 1;
    }
}
