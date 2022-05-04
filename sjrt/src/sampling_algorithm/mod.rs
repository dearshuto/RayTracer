use crate::Vector3f;

mod default_sampling_estimation;
mod nee;
pub use nee::NextEventEstimation;
pub use default_sampling_estimation::DefaultSamplingEstimation;

pub struct SamplingResult {
    pub direction: Vector3f,
    pub weight: f32,
}
