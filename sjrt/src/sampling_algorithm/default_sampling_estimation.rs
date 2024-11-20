use crate::sampling_algorithm::SamplingResult;
use crate::{IScene, Vector3f};
use rand::Rng;

#[derive(Default)]
pub struct DefaultSamplingEstimation {}

impl DefaultSamplingEstimation {
    pub fn new() -> Self {
        Self {}
    }

    pub fn estimate<TScene: IScene>(
        &self,
        _position: &Vector3f,
        normal: &Vector3f,
        _scene: &TScene,
    ) -> Vec<SamplingResult<f32, Vector3f>> {
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

        vec![SamplingResult::<f32, Vector3f> {
            weight: 1.0,
            direction: result,
        }]
    }
}
