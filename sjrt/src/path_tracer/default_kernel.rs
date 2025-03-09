use crate::util::HitParams;

use crate::IUniHemisphereUniformDistribution;

use super::IKernel;

#[derive(Clone)]
pub struct DefaultKernel;

impl IKernel for DefaultKernel {
    // TODO
    type MaterialId = u32;
    type ReflectionEstimationContext = crate::util::UnitHemisphereUniformDistribution;

    type Point = nalgebra::Vector3<f32>;
    type Color = nalgebra::Vector3<f32>;
    type HitParams = HitParams;

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
        let axis = nalgebra::Vector3::z().cross(&normal);
        let axisangle = axis * angle;
        let rotation = nalgebra::UnitQuaternion::new(axisangle);

        // ローカル座標で半球面上の点をサンプリング
        let direction = context.sample();

        // サンプリングした頂点をワールド空間に変換
        let world_direction = rotation * direction;

        world_direction
    }
}

#[cfg(test)]
mod tests {

    use crate::IKernel;

    use super::DefaultKernel;

    /// X 軸方向の法線に対して半球サンプリングしたベクトルが意図した範囲に収まっているかをテスト
    #[test]
    fn test_direction_x() {
        let kernel = DefaultKernel {};
        let mut context = kernel.new_reflection_estimation_context();

        for _ in 0..3000 {
            let normal = nalgebra::Vector3::x();
            let direction =
                kernel.estimate_next_reflection(0, &mut context, &nalgebra::Vector3::x(), &normal);
            assert!(direction.x >= 0.0);
        }
    }

    /// Y 軸方向の法線に対して半球サンプリングしたベクトルが意図した範囲に収まっているかをテスト
    #[test]
    fn test_direction_y() {
        let kernel = DefaultKernel {};
        let mut context = kernel.new_reflection_estimation_context();

        for _ in 0..3000 {
            let normal = nalgebra::Vector3::y();
            let direction =
                kernel.estimate_next_reflection(0, &mut context, &nalgebra::Vector3::x(), &normal);
            assert!(direction.y >= 0.0);
        }
    }

    /// Z 軸方向の法線に対して半球サンプリングしたベクトルが意図した範囲に収まっているかをテスト
    #[test]
    fn test_direction_z() {
        let kernel = DefaultKernel {};
        let mut context = kernel.new_reflection_estimation_context();

        for _ in 0..3000 {
            let normal = nalgebra::Vector3::z();
            let direction =
                kernel.estimate_next_reflection(0, &mut context, &nalgebra::Vector3::x(), &normal);
            assert!(direction.z >= 0.0);
        }
    }
}
