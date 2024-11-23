use crate::traits::IVector3;

mod default_sampling_estimation;
mod nee;
pub use default_sampling_estimation::DefaultSamplingEstimation;
pub use nee::{IRelatedLightEnumerator, NextEventEstimation};

pub struct SamplingResult<TFloat, TVector3>
where
    TFloat: num::Float,
    TVector3: IVector3<TFloat>,
{
    pub direction: TVector3,
    pub weight: TFloat,
}
