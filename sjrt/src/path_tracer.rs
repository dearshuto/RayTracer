use rand::Rng;

use crate::{IBidirectionalReflectanceDistributionFunction, IRenderer, IScene, Vector3f};

pub struct PathTracer {
    _sampling_count: u16,
}

impl PathTracer {
    pub fn new(sampling_count: u16) -> Self {
        Self {
            _sampling_count: sampling_count,
        }
    }

    pub fn cast_ray<TScene: IScene>(
        &self,
        scene: &TScene,
        position: &Vector3f,
        direction: &Vector3f,
        depth: u32,
    ) -> (f32, f32, f32) {
        if 50 < depth {
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

                let brdf = crate::brdf::Lambert::new();
                let value = brdf.calculate(&material_info.normal, &direction, &new_direction);
                let new_position = *mat_position + 0.1 * new_direction;
                let result = self.cast_ray(
                    scene,
                    &new_position,
                    &new_direction,
                    depth + 1,
                );
                (result.0 * value, result.1 * value, result.2 * value)
            }
        } else {
            (0.0, 0.0, 0.0)
        }
    }
}

impl IRenderer for PathTracer {
    fn render<TScene: IScene, TBuffer: crate::IBuffer>(
        &self,
        scene: &TScene,
        buffer: &mut TBuffer,
    ) {
        let lower_left = Vector3f::new(-5.0, -5.0, 0.0);
        let stride_width = 10.0 / (buffer.get_height() as f32);
        let stride_height = 10.0 / (buffer.get_height() as f32);

        for y in 0..buffer.get_height() {
            for x in 0..buffer.get_height() {
                let sampling_count = self._sampling_count;
                let mut red = 0.0;
                let mut blue = 0.0;
                let mut green = 0.0;

                let camera_position = Vector3f::new(0.0, 2.0, 5.0);
                let local_target = lower_left
                    + Vector3f::new(
                        (x as f32) * stride_width,
                        y as f32 * stride_height,
                        0.0,
                    );
                let direction = local_target - camera_position;
                for _i in 0..sampling_count {
                    let color = self.cast_ray(
                        scene,
                        &camera_position,
                        &direction,
                        0, // depth
                    );
                    red += color.0;
                    green += color.1;
                    blue += color.2;
                }
                let red_result = red / (sampling_count as f32);
                let green_result = green / (sampling_count as f32);
                let blue_result = blue / (sampling_count as f32);
                buffer.set_color(
                    x,
                    buffer.get_width() - y - 1,
                    (255.0 * red_result) as u8,
                    (255.0 * green_result) as u8,
                    (255.0 * blue_result) as u8,
                );
            }
        }
    }
}

