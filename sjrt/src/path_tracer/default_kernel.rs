use crate::util::HitParams;

use crate::IUniHemisphereUniformDistribution;

use super::IKernel;

#[derive(Clone)]
pub struct DefaultKernel;

impl IKernel for DefaultKernel {
    // TODO
    type MaterialId = u32;
    type ReflectionEstimationContext = crate::util::UnitHemisphereUniformDistribution;

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
        crate::util::UnitHemisphereUniformDistribution::new()
    }

    fn estimate_next_reflection(
        &self,
        #[allow(unused)] id: Self::MaterialId,
        context: &mut Self::ReflectionEstimationContext,
        #[allow(unused)] in_direction: &Self::Point,
        normal: &Self::Point,
    ) -> Self::Point {
        // 反射する点のワールド空間と半球の方向を一致させるための回転を算出
        // Z 軸が反射点の法線と一致するようにして計算している
        let angle = normal.dot(&nalgebra::Vector3::z()).acos();
        let axisangle = nalgebra::Vector3::z() * angle;
        let rotation = nalgebra::UnitQuaternion::new(axisangle);

        // ローカル座標で半球面上の点をサンプリング
        let direction = context.sample();

        // サンプリングした頂点をワールド空間に変換
        let world_direction = rotation * direction;

        world_direction
    }
}
