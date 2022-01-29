use crate::Vector3f;

pub trait IBidirectionalReflectanceDistributionFunction {
    fn new() -> Self;

    fn calculate(
        &self,
        normal: &Vector3f,
        in_direction: &Vector3f,
        out_direction: &Vector3f,
    ) -> f32;
}
