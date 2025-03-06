use crate::{traits::IRandomEngine, util::HitParams};

use super::IKernel;

#[derive(Clone)]
pub struct DefaultKernel;

impl IKernel for DefaultKernel {
    // TODO
    type MaterialId = u32;
    type ReflectionEstimationContext = crate::util::RandomEngine;

    type RondomEngine = crate::util::RandomEngine;
    type Point = nalgebra::Vector3<f32>;
    type Color = nalgebra::Vector3<f32>;
    type HitParams = HitParams;

    fn random_engine(&self) -> Self::RondomEngine {
        crate::util::RandomEngine::new()
    }

    fn new_point(&self, x: f32, y: f32, z: f32) -> Self::Point {
        nalgebra::Vector3::new(x, y, z)
    }

    fn new_reflection_estimation_context(&self) -> Self::ReflectionEstimationContext {
        crate::util::RandomEngine::new()
    }

    fn estimate_next_reflection(
        &self,
        #[allow(unused)] id: Self::MaterialId,
        context: &mut Self::ReflectionEstimationContext,
        #[allow(unused)] in_direction: &Self::Point,
        normal: &Self::Point,
    ) -> Self::Point {
        loop {
            let ratio_x = context.generate_range(-1.0..1.0);
            let ratio_y = context.generate_range(-1.0..1.0);
            let ratio_z = context.generate_range(-1.0..1.0);
            let new_normal = nalgebra::Vector3::new(ratio_x, ratio_y, ratio_z).normalize();
            if new_normal.dot(&normal) <= 0.0 {
                continue;
            }

            break new_normal;
        }
    }
}
