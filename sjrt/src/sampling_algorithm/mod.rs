mod default_sampling_estimation;
mod nee;
pub use default_sampling_estimation::DefaultSamplingEstimation;
pub use nee::{IRelatedLightEnumerator, NextEventEstimation};

pub struct SamplingResult<TFloat, TVector3>
where
    TFloat: num::Float,
{
    pub direction: TVector3,
    pub weight: TFloat,
}
