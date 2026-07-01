//! Global configuration for the XOR neural network.

#[derive(Debug, Clone)]
pub struct Config {
    pub input_size: usize,
    pub hidden_size: usize,
    pub epochs: usize,
    pub learning_rate: f32,
    pub dataset_size: usize,
    pub seed: u64,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            input_size: 2,
            hidden_size: 8,
            epochs: 100,
            learning_rate: 0.1,
            dataset_size: 50_000,
            seed: 42,
        }
    }
}