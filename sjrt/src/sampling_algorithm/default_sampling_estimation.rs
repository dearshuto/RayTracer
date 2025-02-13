use std::marker::PhantomData;
use std::ops::{Add, Mul, Range};

use crate::sampling_algorithm::SamplingResult;
use crate::traits::{IInnerProduct, INormalized, IRandomEngine, IVectorComponent3};
use crate::IScene;
use rand::Rng;

#[derive(Default)]
pub struct DefaultSamplingEstimation;

impl DefaultSamplingEstimation {
    pub fn new() -> Self {
        Self {}
    }

    pub fn estimate<TFloat, TVector3, TScene>(
        &self,
        position: &TVector3,
        normal: &TVector3,
        scene: &TScene,
    ) -> Vec<SamplingResult<TFloat, TVector3>>
    where
        TFloat: num::Float
            + From<f32>
            + Into<f32>
            + PartialOrd<TFloat>
            + Mul<TVector3, Output = TVector3>,
        TVector3: IVectorComponent3<TFloat>
            + INormalized
            + IInnerProduct<TFloat>
            + Add<TVector3, Output = TVector3>
            + Copy,
        TScene: IScene,
    {
        let random_engine = RandomEngineAdapter::new();
        let mut internal = DefaultSamplingEstimationInternal::new(random_engine);
        internal.estimate(position, normal, scene)
    }
}

struct RandomEngineAdapter {
    rng: rand::rngs::ThreadRng,
}

impl RandomEngineAdapter {
    pub fn new() -> Self {
        let rng = rand::thread_rng();
        Self { rng }
    }
}

impl<TFloat> IRandomEngine<TFloat> for RandomEngineAdapter
where
    TFloat: num::Float + From<f32> + Into<f32>,
{
    fn generate_range(&mut self, range: Range<TFloat>) -> TFloat {
        let start: f32 = range.start.into();
        let end = range.end.into();
        let value = self.rng.gen_range(start..end);
        From::from(value)
    }
}

pub struct DefaultSamplingEstimationInternal<TFloat, TRandomEngine>
where
    TFloat: num::Float,
    TRandomEngine: IRandomEngine<TFloat>,
{
    random_engine: TRandomEngine,
    _marker: PhantomData<TFloat>,
}

impl<TFloat, TRandomEngine> DefaultSamplingEstimationInternal<TFloat, TRandomEngine>
where
    TFloat: num::Float,
    TRandomEngine: IRandomEngine<TFloat>,
{
    pub fn new(random_engine: TRandomEngine) -> Self {
        Self {
            random_engine,
            _marker: PhantomData,
        }
    }

    pub fn estimate<TVector3, TScene>(
        &mut self,
        _position: &TVector3,
        normal: &TVector3,
        _scene: &TScene,
    ) -> Vec<SamplingResult<TFloat, TVector3>>
    where
        TFloat: num::Float + From<f32> + PartialOrd<TFloat> + Mul<TVector3, Output = TVector3>,
        TVector3: IVectorComponent3<TFloat>
            + INormalized
            + IInnerProduct<TFloat>
            + Add<TVector3, Output = TVector3>
            + Copy,
        TScene: IScene,
    {
        let range = From::<f32>::from(-1.0)..From::<f32>::from(1.0);
        let x = self.random_engine.generate_range(range.clone());
        let y = self.random_engine.generate_range(range.clone());
        let z = self.random_engine.generate_range(range);
        let random_direction = TVector3::new(x, y, z).normalized();

        let result = if TFloat::zero() < random_direction.dot(normal) {
            random_direction
        } else {
            let two: TFloat = ::core::convert::From::<f32>::from(2.0f32);
            random_direction + two * (-random_direction.dot(normal)) * (*normal)
        };

        vec![SamplingResult::<TFloat, TVector3> {
            weight: ::core::convert::From::<f32>::from(1.0),
            direction: result,
        }]
    }
}

#[cfg(test)]
mod tests {
    use std::ops::Range;

    use super::*;

    struct RandomEngineMock;
    impl IRandomEngine<f32> for RandomEngineMock {
        fn generate_range(&mut self, _range: Range<f32>) -> f32 {
            0.0f32
        }
    }

    struct SceneMock;
    impl IScene for SceneMock {
        fn cast_ray(
            &self,
            _from: &nalgebra::Vector3<f32>,
            _to: &nalgebra::Vector3<f32>,
        ) -> Option<crate::MaterialInfo> {
            None
        }

        fn enumerate_related_lights(
            &self,
            _position: &nalgebra::Vector3<f32>,
        ) -> crate::EnumerateLightResult {
            crate::EnumerateLightResult {
                centers: Default::default(),
            }
        }

        fn find_background_color(
            &self,
            _position: &nalgebra::Vector3<f32>,
            _direction: &nalgebra::Vector3<f32>,
        ) -> nalgebra::Vector3<f32> {
            nalgebra::Vector3::zeros()
        }
    }

    #[test]
    fn new_f32() {
        let random_engine = RandomEngineMock {};
        let scene = SceneMock {};
        let position = nalgebra::Vector3::zeros();
        let normal = nalgebra::Vector3::new(0.0, 1.0, 0.0);
        let mut estimation = DefaultSamplingEstimationInternal::new(random_engine);
        let _ = estimation.estimate(&position, &normal, &scene);
    }
}
