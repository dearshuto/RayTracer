use std::ops::{Add, Mul};

use crate::sampling_algorithm::SamplingResult;
use crate::traits::{IVector3, IVectorComponent3};
use crate::IScene;
use rand::Rng;

#[derive(Default)]
pub struct DefaultSamplingEstimation {}

impl DefaultSamplingEstimation {
    pub fn new() -> Self {
        Self {}
    }

    pub fn estimate<TFloat, TVector3, TVectorComponent3, TScene>(
        &self,
        _position: &TVector3,
        normal: &TVector3,
        _scene: &TScene,
    ) -> Vec<SamplingResult<TFloat, TVector3>>
    where
        TFloat: num::Float + From<f32> + PartialOrd<TFloat> + Mul<TVector3, Output = TVector3>,
        TVector3:
            IVector3<TFloat> + IVectorComponent3<TFloat> + Add<TVector3, Output = TVector3> + Copy,
        TVectorComponent3: IVectorComponent3<TFloat>,
        TScene: IScene,
    {
        let mut rng = rand::thread_rng();
        let x: TFloat = ::core::convert::From::<f32>::from(rng.gen_range(-1.0, 1.0));
        let y: TFloat = ::core::convert::From::<f32>::from(rng.gen_range(-1.0, 1.0));
        let z: TFloat = ::core::convert::From::<f32>::from(rng.gen_range(-1.0, 1.0));
        let random_direction = TVector3::new(x, y, z).normalize();

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
