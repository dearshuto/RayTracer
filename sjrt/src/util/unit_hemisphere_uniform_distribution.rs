use rand_distr::Distribution;

use crate::IUniHemisphereUniformDistribution;

pub struct UnitHemisphereUniformDistribution {
    rng: rand::rngs::ThreadRng,
}

impl UnitHemisphereUniformDistribution {
    pub fn new() -> Self {
        Self { rng: rand::rng() }
    }
}

impl IUniHemisphereUniformDistribution for UnitHemisphereUniformDistribution {
    type FieldType = f32;
    type Vector = nalgebra::Vector3<f32>;

    fn sample(&mut self) -> Self::Vector {
        let direction: [f32; 3] = rand_distr::UnitSphere.sample(&mut self.rng);
        nalgebra::Vector3::new(direction[0], direction[1], direction[2].abs())
    }
}

#[cfg(test)]
mod tests {
    use crate::IUniHemisphereUniformDistribution;

    use super::UnitHemisphereUniformDistribution;

    #[test]
    fn test_range_z() {
        let mut engine = UnitHemisphereUniformDistribution::new();
        for _ in 0..3000 {
            let direction = engine.sample();

            // z 方向正の領域にしか存在しない
            assert!(direction.z >= 0.0);
        }
    }

    #[test]
    fn norm() {
        let mut engine = UnitHemisphereUniformDistribution::new();
        for _ in 0..3000 {
            let direction = engine.sample();

            // 大きさが 1.0 である
            // f32::EPSILON ほどの精度はなくてもよいとする
            assert!((1.0 - direction.norm()) < 0.0001);
        }
    }
}
