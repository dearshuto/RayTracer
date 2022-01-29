use crate::{IBidirectionalReflectanceDistributionFunction, Vector3f};

pub struct Lambert {}

impl IBidirectionalReflectanceDistributionFunction for Lambert {
    fn new() -> Self {
        Self {}
    }

    fn calculate(
        &self,
        normal: &Vector3f,
        _in_direction: &Vector3f,
        out_direction: &Vector3f,
    ) -> f32 {
        let dot_value = normal.to_nalgebra().dot(&out_direction.to_nalgebra());
        let result = dot_value / std::f32::consts::PI;
        result
    }
}