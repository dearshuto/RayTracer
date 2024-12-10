use crate::{
    brdf::{Lambert, PerfectSpecularReflection},
    traits::IRandomEngine,
    DefaultSamplingEstimation, IBidirectionalReflectanceDistributionFunction, IRenderer, IScene,
    NextEventEstimation, Vector3f,
};
use rand::Rng;

pub struct PathTracer {
    sampling_count: u16,
    depth_max: u16,
    is_nee_enabled: bool,
}

impl PathTracer {
    pub fn new(sampling_count: u16, depth_max: u16, is_nee_enabled: bool) -> Self {
        Self {
            sampling_count,
            depth_max,
            is_nee_enabled,
        }
    }

    pub fn cast_ray<TScene: IScene>(
        &self,
        scene: &TScene,
        position: &Vector3f,
        direction: &Vector3f,
        depth: u32,
    ) -> (Vector3f, Option<Vector3f>) // (色、位置)
    {
        let mut random_engine = RandomEngine::new();
        self.cast_ray_with_random_engine(scene, position, direction, depth, &mut random_engine)
    }

    pub fn cast_ray_with_random_engine<TScene: IScene, TRandomEngine: IRandomEngine<f32>>(
        &self,
        scene: &TScene,
        position: &Vector3f,
        direction: &Vector3f,
        depth: u32,
        random_engine: &mut TRandomEngine,
    ) -> (Vector3f, Option<Vector3f>) // (色、位置)
    {
        if self.depth_max < depth as u16 {
            let sky_color = scene.find_background_color(position, direction);
            return (sky_color, None);
        }

        let normalized_direction = direction.normalize();
        let to = Vector3f::new(
            position.x + 100.0 * normalized_direction.x,
            position.y + 100.0 * normalized_direction.y,
            position.z + 100.0 * normalized_direction.z,
        );
        if let Some(material_info) = scene.cast_ray(position, &to) {
            let _mat_normal = &material_info.normal;
            let mat_position = &material_info.position;
            if 0.0 < material_info.property.emission {
                let emission = Vector3f::new(
                    material_info.property.emission,
                    material_info.property.emission,
                    material_info.property.emission,
                );
                (emission, Some(*mat_position))
            } else {
                let direction_candidates = if self.is_nee_enabled {
                    NextEventEstimation::new(self.sampling_count).estimate(
                        &material_info.position,
                        &material_info.normal,
                        scene,
                    )
                } else {
                    DefaultSamplingEstimation::new().estimate::<f32, Vector3f, Vector3f, TScene>(
                        &material_info.position,
                        &material_info.normal,
                        scene,
                    )
                };

                let (mut red, mut green, mut blue) = (0.0, 0.0, 0.0);
                for result in &direction_candidates {
                    let direction_candidate = result.direction;
                    if !direction_candidate.is_valid() {
                        continue;
                    }

                    let weight = result.weight;
                    let reflect_rate = random_engine.generate_range(0.0..1.0);

                    // 鏡面反射か、拡散反射かを確立で切り替える
                    let metaric = material_info.property.metaric;
                    let value = if reflect_rate < metaric {
                        let perfect_specular_reflection = PerfectSpecularReflection::new();
                        perfect_specular_reflection.calculate(
                            &material_info.normal,
                            &normalized_direction,
                            &direction_candidate,
                        )
                    } else {
                        Lambert::new().calculate(
                            &material_info.normal,
                            &normalized_direction,
                            &direction_candidate,
                        )
                    };

                    let new_position = *mat_position + 0.1 * direction_candidate;
                    let albedo = material_info.property.albedo;
                    let (result, hit_position_opt) =
                        self.cast_ray(scene, &new_position, &direction_candidate, depth + 1);

                    let distance = if let Some(hit_position) = hit_position_opt {
                        let diff = hit_position - *mat_position;
                        diff.norm()
                    } else {
                        1.0
                    };

                    let ratio = direction_candidates.len() as f32;
                    red += weight * (result.x * value * albedo.x / ratio) / (distance * distance);
                    green += weight * (result.y * value * albedo.y / ratio) / (distance * distance);
                    blue += weight * (result.z * value * albedo.z / ratio) / (distance * distance);
                }

                (
                    Vector3f::new(red, green, blue),
                    Some(material_info.position),
                )
            }
        } else {
            let sky_color = scene.find_background_color(position, direction);
            (sky_color, None)
        }
    }
}

impl IRenderer for PathTracer {
    fn render<TScene: IScene>(
        &self,
        scene: &TScene,
        position: &Vector3f,
        direction: &Vector3f,
    ) -> (f32, f32, f32) {
        let sampling_count = self.sampling_count;
        let mut red = 0.0;
        let mut blue = 0.0;
        let mut green = 0.0;
        for _i in 0..sampling_count {
            let (color, _) = self.cast_ray(
                scene, position, direction, 0, // depth
            );
            red += color.x;
            green += color.y;
            blue += color.z;
        }
        let red_result = red / (sampling_count as f32);
        let green_result = green / (sampling_count as f32);
        let blue_result = blue / (sampling_count as f32);
        (red_result, green_result, blue_result)
    }
}

struct RandomEngine {
    rng: rand::rngs::ThreadRng,
}

impl RandomEngine {
    pub fn new() -> Self {
        let rng = rand::thread_rng();
        Self { rng }
    }
}

impl IRandomEngine<f32> for RandomEngine {
    fn generate_range(&mut self, range: std::ops::Range<f32>) -> f32 {
        self.rng.gen_range(range)
    }
}
