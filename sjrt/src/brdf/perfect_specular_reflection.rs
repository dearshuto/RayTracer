use std::ops::{Add, Mul, Sub};

use crate::{traits::IVector3, IBidirectionalReflectanceDistributionFunction};

#[derive(Default)]
pub struct PerfectSpecularReflection {}

impl PerfectSpecularReflection {
    pub fn new() -> Self {
        Self {}
    }
}

impl<TFloat, TVector> IBidirectionalReflectanceDistributionFunction<TFloat, TVector>
    for PerfectSpecularReflection
where
    TFloat: num::Float
        + Sub<f32, Output = TFloat>
        + Mul<TVector, Output = TVector>
        + Mul<f32, Output = TFloat>,

    TVector: IVector3<TFloat> + Add<Output = TVector> + Mul<f32, Output = TVector> + Copy + Clone,
{
    fn calculate(
        &self,
        normal: &TVector,
        in_direction: &TVector,
        out_direction: &TVector,
    ) -> TFloat {
        let reflect_direction = in_direction.mul(2.0f32) + (-in_direction.dot(normal)) * (*normal);
        let result = out_direction.dot(&reflect_direction);
        (result - 0.99).max(TFloat::zero()).ceil()
    }
}
