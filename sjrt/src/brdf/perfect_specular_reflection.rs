use crate::IBidirectionalReflectanceDistributionFunction;

pub struct PerfectSpecularReflection {}

impl IBidirectionalReflectanceDistributionFunction for PerfectSpecularReflection {
    fn new() -> Self {
        Self{}
    }

    fn calculate(
        &self,
        normal: &crate::Vector3f,
        in_direction: &crate::Vector3f,
        out_direction: &crate::Vector3f,
    ) -> f32 {
        let reflect_direction = *in_direction + 2.0 * (-in_direction.dot(normal)) * (*normal);
        let result = out_direction.dot(&reflect_direction);
        (result - 0.99).max(0.0).ceil()
    }
}
