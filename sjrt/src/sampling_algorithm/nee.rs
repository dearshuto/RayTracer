use crate::sampling_algorithm::SamplingResult;
use crate::{IScene, Vector3f};
use rand::prelude::*;

pub struct NextEventEstimation {
    sampling_count: u16,
}

impl NextEventEstimation {
    pub fn new(sampling_count: u16) -> Self {
        Self { sampling_count }
    }

    pub fn estimate<TScene: IScene>(
        &self,
        position: &Vector3f,
        normal: &Vector3f,
        scene: &TScene,
    ) -> Vec<SamplingResult> {
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
