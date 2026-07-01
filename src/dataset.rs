use crate::random::Random;

#[derive(Debug, Clone)]
pub struct Sample {
    pub input: Vec<f32>,
    pub target: Vec<f32>,
}

pub struct XorDatasetGenerator {
    rng: Random,
}

impl XorDatasetGenerator {
    pub fn new(seed: u64) -> Self {
        Self {
            rng: Random::new(seed),
        }
    }

    fn generate_sample(&mut self) -> Sample {
        let x1 = if self.rng.next_bool() { 1.0 } else { 0.0 };
        let x2 = if self.rng.next_bool() { 1.0 } else { 0.0 };

        let target = if x1 != x2 { 1.0 } else { 0.0 };

        Sample {
            input: vec![x1, x2],
            target: vec![target],
        }
    }

    pub fn generate(&mut self, size: usize) -> Vec<Sample> {
        let mut dataset = Vec::with_capacity(size);

        for _ in 0..size {
            dataset.push(self.generate_sample());
        }

        dataset
    }
}