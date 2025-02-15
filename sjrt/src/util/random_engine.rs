use std::sync::{Arc, Mutex};

use crate::traits::IRandomEngine;

#[derive(Clone)]
pub struct RandomEngine;

impl RandomEngine {
    pub fn new() -> Self {
        Self {}
    }
}

impl IRandomEngine<f32> for RandomEngine {
    fn generate_range(&mut self, range: std::ops::Range<f32>) -> f32 {
        // [0, 1)
        let random = rand::random::<f32>();
        let length = range.end - range.start;
        length * random - range.end
    }
}

impl<T> IRandomEngine<f32> for Arc<Mutex<T>>
where
    T: IRandomEngine<f32>,
{
    fn generate_range(&mut self, range: std::ops::Range<f32>) -> f32 {
        // [0, 1)
        let random = rand::random::<f32>();
        let length = range.end - range.start;
        length * random - range.end
    }
}
