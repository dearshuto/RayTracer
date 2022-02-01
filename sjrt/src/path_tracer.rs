use rand::Rng;

use crate::{IBidirectionalReflectanceDistributionFunction, IRenderer, IScene, Vector3f, brdf::{Lambert, PerfectSpecularReflection}};

pub struct PathTracer {
    _sampling_count: u16,
    _depth_max: u16,
}

impl PathTracer {
    pub fn new(sampling_count: u16, depth_max: u16) -> Self {
        Self {
            _sampling_count: sampling_count,
            _depth_max: depth_max,
        }
    }

    pub fn cast_ray<TScene: IScene>(
        &self,
        scene: &TScene,
        position: &Vector3f,
        direction: &Vector3f,
        depth: u32,
    ) -> (f32, f32, f32) {
        if 50 < self._depth_max {
            return (0.0, 0.0, 0.0);
        }

        let to = Vector3f::new(
            position.x + 100.0 * direction.x,
            position.y + 100.0 * direction.y,
            position.z + 100.0 * direction.z,
        );
        if let Some(material_info) = scene.cast_ray(position, &to) {
            let _mat_normal = &material_info.normal;
            let mat_position = &material_info.position;
            if 0.0 < material_info.property.emission {
                (
                    material_info.property.emission,
                    material_info.property.emission,
                    material_info.property.emission,
                )
            } else {
                let mut rng = rand::thread_rng();
                let x: f32 = rng.gen_range(-1.0..1.0);
                let y: f32 = rng.gen_range(-1.0..1.0);
                let z: f32 = rng.gen_range(-1.0..1.0);

                let random_direction = Vector3f::new(x, y, z).normalize();
                let dot = random_direction.dot(&material_info.normal);

                let new_direction = if dot < 0.0 {
                    let result =
                        random_direction + 2.0 * dot.abs() * material_info.normal;
                    result
                } else {
                    random_direction
                };

                // 鏡面反射か、拡散反射かを確立で切り替える
                let reflect_rate = rng.gen_range(0.0..1.0);
                let value = if material_info.property.metaric < reflect_rate {
                    Lambert::new().calculate(&material_info.normal, &direction, &new_direction)
                }
                else {
                    PerfectSpecularReflection::new().calculate(&material_info.normal, &direction, &new_direction)
                };

                let new_position = *mat_position + 0.1 * new_direction;
                let albedo = material_info.property.albedo;
                let result = self.cast_ray(scene, &new_position, &new_direction, depth + 1);
                (result.0 * value * albedo.x, result.1 * value * albedo.y, result.2 * value * albedo.z)
            }
        } else {
            (0.0, 0.0, 0.0)
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
        let sampling_count = self._sampling_count;
        let mut red = 0.0;
        let mut blue = 0.0;
        let mut green = 0.0;
        for _i in 0..sampling_count {
            let color = self.cast_ray(
                scene, &position, &direction, 0, // depth
            );
            red += color.0;
            green += color.1;
            blue += color.2;
        }
        let red_result = red / (sampling_count as f32);
        let green_result = green / (sampling_count as f32);
        let blue_result = blue / (sampling_count as f32);
        (red_result, green_result, blue_result)
    }
}
