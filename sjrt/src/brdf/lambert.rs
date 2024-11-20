use std::ops::Div;

use crate::{traits::IVector3, IBidirectionalReflectanceDistributionFunction};

#[derive(Default)]
pub struct Lambert {}

impl Lambert {
    pub fn new() -> Self {
        Self {}
    }
}

impl<TFloat, TVector> IBidirectionalReflectanceDistributionFunction<TFloat, TVector> for Lambert
where
    TFloat: num::Float + Div<f32, Output = TFloat>,
    TVector: IVector3<TFloat>,
{
    fn calculate(
        &self,
        normal: &TVector,
        _in_direction: &TVector,
        out_direction: &TVector,
    ) -> TFloat {
        // TODO: 復旧
        // debug_assert!((normal.norm() - 1.0).abs() < 0.01);
        // debug_assert!((out_direction.norm() - 1.0).abs() < 0.01);

        let dot_value = normal.dot(out_direction).max(TFloat::zero());
        dot_value / std::f32::consts::PI
    }
}
