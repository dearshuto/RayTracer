use rand::Rng;

use crate::traits::IRandomEngine;

pub struct RandomEngine {
    rng: rand::rngs::ThreadRng,
}

impl RandomEngine {
    pub fn new() -> Self {
        let rng = rand::thread_rng();
        Self { rng }
    }
}

impl IRandomEngine<f32> for RandomEngine {
    fn generate_range(&mut self, range: std::ops::Range<f32>) -> f32 {
        self.rng.gen_range(range)
    }
}
