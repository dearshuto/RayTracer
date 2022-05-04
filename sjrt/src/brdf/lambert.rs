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
        debug_assert!((normal.norm() - 1.0).abs() < 0.01);
        debug_assert!((out_direction.norm() - 1.0).abs() < 0.01);

        let dot_value = normal.dot(&out_direction).max(0.0);
        let result = dot_value / std::f32::consts::PI;
        result
    }
}
