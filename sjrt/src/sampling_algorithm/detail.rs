use std::ops::Range;

use rand::Rng;

use crate::traits::IRandomEngine;

pub struct RandomEngineAdapter {
    rng: rand::rngs::ThreadRng,
}

impl RandomEngineAdapter {
    pub fn new() -> Self {
        let rng = rand::thread_rng();
        Self { rng }
    }
}

impl<TFloat> IRandomEngine<TFloat> for RandomEngineAdapter
where
    TFloat: num::Float + From<f32> + Into<f32>,
{
    fn generate_range(&mut self, range: Range<TFloat>) -> TFloat {
        let start: f32 = range.start.into();
        let end = range.end.into();
        let value = self.rng.gen_range(start..end);
        From::from(value)
    }
}
