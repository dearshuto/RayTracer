use std::ops::{Add, Mul, Sub};

use crate::sampling_algorithm::SamplingResult;
use crate::traits::{IVector3, IVectorComponent3};
use crate::{IScene, Vector3f};
use rand::prelude::*;

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
            + Mul<TVector3, Output = TVector3>
            + num::traits::Inv<Output = TFloat>,
        TVector3: IVector3<TFloat>
            + IVectorComponent3<TFloat>
            + Add<TVector3, Output = TVector3>
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

        let mut rng = rand::thread_rng();
        let x: TFloat = ::core::convert::From::from(rng.gen_range(-1.0..1.0));
        let y: TFloat = ::core::convert::From::from(rng.gen_range(-1.0..1.0));
        let z: TFloat = ::core::convert::From::from(rng.gen_range(-1.0..1.0));
        let random_direction = TVector3::new(x, y, z).normalize();

        let result = if TFloat::zero() < random_direction.dot(normal) {
            random_direction
        } else {
            let two: TFloat = ::core::convert::From::from(2.0f32);
            random_direction + two * (-random_direction.dot(normal)) * *normal
        };
        let random_direction_result = SamplingResult {
            weight: ::core::convert::From::from(1.0),
            direction: result,
        };
        direction_candidate.push(random_direction_result);
        direction_candidate
    }

    pub fn estimate<TScene: IScene>(
        &self,
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

        let mut rng = rand::thread_rng();
        let x: f32 = rng.gen_range(-1.0..1.0);
        let y: f32 = rng.gen_range(-1.0..1.0);
        let z: f32 = rng.gen_range(-1.0..1.0);
        let random_direction = Vector3f::new(x, y, z).normalize();

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
