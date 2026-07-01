#[derive(Debug, Clone)]
pub struct Random {
    state: u64,
}

impl Random {
    pub fn new(seed: u64) -> Self {
        Self { state: seed }
    }

    fn next_u32(&mut self) -> u32 {
        const MULTIPLIER: u64 = 6364136223846793005;
        const INCREMENT: u64 = 1;

        self.state = self
            .state
            .wrapping_mul(MULTIPLIER)
            .wrapping_add(INCREMENT);

        (self.state >> 32) as u32
    }

    pub fn next_f32(&mut self) -> f32 {
        self.next_u32() as f32 / u32::MAX as f32
    }

    pub fn next_weight(&mut self) -> f32 {
        self.next_f32() * 2.0 - 1.0
    }

    pub fn next_bool(&mut self) -> bool {
        self.next_f32() >= 0.5
    }


}

