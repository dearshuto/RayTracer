use std::marker::PhantomData;
use std::ops::{Add, Mul, Sub};

use crate::sampling_algorithm::SamplingResult;
use crate::traits::{IRandomEngine, IVector3, IVectorComponent3};
use crate::{IScene, Vector3f};

use super::detail::RandomEngineAdapter;

pub trait IRelatedLightEnumerator<TFloat, TVector3>
where
    TFloat: num::Float,
    TVector3: IVector3<TFloat>,
{
    fn enumerate(&self, position: &TVector3) -> impl Iterator<Item = TVector3>;
}

pub struct NextEventEstimation {
    sampling_count: u16,
}

impl NextEventEstimation {
    pub fn new(sampling_count: u16) -> Self {
        Self { sampling_count }
    }

    pub fn estimate_with<TFloat, TVector3, TEnumerator>(
        &self,
        position: &TVector3,
        normal: &TVector3,
        scene: &TEnumerator,
    ) -> Vec<SamplingResult<TFloat, TVector3>>
    where
        TFloat: num::Float
            + From<f32>
            + Into<f32>
            + Mul<TVector3, Output = TVector3>
            + num::traits::Inv<Output = TFloat>,
        TVector3: IVector3<TFloat>
            + IVectorComponent3<TFloat>
            + Add<TVector3, Output = TVector3>
            + Sub<TVector3, Output = TVector3>
            + Mul<TFloat, Output = TVector3>
            + Copy,
        TEnumerator: IRelatedLightEnumerator<TFloat, TVector3>,
    {
        let random_engine = RandomEngineAdapter::new();
        let mut internal = NextEventEstimationInternal::<TFloat, RandomEngineAdapter>::new(
            self.sampling_count,
            random_engine,
        );
        internal.estimate_with(position, normal, scene)
    }

    pub fn estimate<TScene: IScene>(
        &self,
        position: &Vector3f,
        normal: &Vector3f,
        scene: &TScene,
    ) -> Vec<SamplingResult<f32, Vector3f>> {
        let random_engine = RandomEngineAdapter::new();
        let mut internal = NextEventEstimationInternal::<f32, RandomEngineAdapter>::new(
            self.sampling_count,
            random_engine,
        );
        internal.estimate(position, normal, scene)
    }
}

struct NextEventEstimationInternal<TFloat, TRandomEngine>
where
    TFloat: num::Float,
    TRandomEngine: IRandomEngine<TFloat>,
{
    sampling_count: u16,
    random_engine: TRandomEngine,
    _marker: PhantomData<TFloat>,
}

impl<TFloat, TRandomEngine> NextEventEstimationInternal<TFloat, TRandomEngine>
where
    TFloat: num::Float + From<f32> + Into<f32> + num::traits::Inv<Output = TFloat>,
    TRandomEngine: IRandomEngine<TFloat>,
{
    pub fn new(sampling_count: u16, random_engine: TRandomEngine) -> Self {
        Self {
            sampling_count,
            random_engine,
            _marker: PhantomData,
        }
    }

    pub fn estimate_with<TVector3, TEnumerator>(
        &mut self,
        position: &TVector3,
        normal: &TVector3,
        scene: &TEnumerator,
    ) -> Vec<SamplingResult<TFloat, TVector3>>
    where
        TVector3: IVector3<TFloat>
            + IVectorComponent3<TFloat>
            + Add<TVector3, Output = TVector3>
            + Mul<TFloat, Output = TVector3>
            + Sub<TVector3, Output = TVector3>
            + Copy,
        TEnumerator: IRelatedLightEnumerator<TFloat, TVector3>,
    {
        let lights = scene.enumerate(position);
        let sampling_count: TFloat = ::core::convert::From::from(self.sampling_count as f32);
        let weight = sampling_count.inv();
        let mut direction_candidate = lights
            .into_iter()
            .map(|light_center| SamplingResult {
                direction: (light_center - *position).normalize(),
                weight,
            })
            .collect::<Vec<_>>();

        let range = From::<f32>::from(-1.0)..From::<f32>::from(1.0);
        let x = self.random_engine.generate_range(range.clone());
        let y = self.random_engine.generate_range(range.clone());
        let z = self.random_engine.generate_range(range);
        let random_direction = TVector3::new(x, y, z).normalize();

        let result = if TFloat::zero() < random_direction.dot(normal) {
            random_direction
        } else {
            let two: TFloat = ::core::convert::From::from(2.0f32);
            random_direction + *normal * two * (-random_direction.dot(normal))
        };
        let random_direction_result = SamplingResult {
            weight: ::core::convert::From::from(1.0),
            direction: result,
        };
        direction_candidate.push(random_direction_result);
        direction_candidate
    }

    pub fn estimate<TScene: IScene>(
        &mut self,
        position: &Vector3f,
        normal: &Vector3f,
        scene: &TScene,
    ) -> Vec<SamplingResult<f32, Vector3f>> {
        let result = scene.enumerate_related_lights(position);
        let mut direction_candidate = result
            .centers
            .iter()
            .map(|light_center| SamplingResult {
                direction: (*light_center - *position).normalize(),
                weight: 1.0 / self.sampling_count as f32,
            })
            .collect::<Vec<_>>();

        let range = From::<f32>::from(-1.0)..From::<f32>::from(1.0);
        let x = self.random_engine.generate_range(range.clone());
        let y = self.random_engine.generate_range(range.clone());
        let z = self.random_engine.generate_range(range);
        let random_direction = Vector3f::new(x.into(), y.into(), z.into()).normalize();

        let result = if 0.0 < random_direction.dot(normal) {
            random_direction
        } else {
            random_direction + 2.0 * (-random_direction.dot(normal)) * *normal
        };
        let random_direction_result = SamplingResult {
            weight: 1.0,
            direction: result,
        };
        direction_candidate.push(random_direction_result);

        direction_candidate
    }
}
