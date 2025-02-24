use super::IKernel;

#[derive(Clone)]
pub struct DefaultKernel;

impl IKernel for DefaultKernel {
    type RondomEngine = crate::util::RandomEngine;
    type Point = nalgebra::Vector3<f32>;
    type Color = nalgebra::Vector3<f32>;

    fn random_engine(&self) -> Self::RondomEngine {
        crate::util::RandomEngine::new()
    }

    fn new_point(&self, x: f32, y: f32, z: f32) -> Self::Point {
        nalgebra::Vector3::new(x, y, z)
    }
}
