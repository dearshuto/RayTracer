use std::sync::Arc;

use image::{DynamicImage, GenericImage};
use sjrt::scene::{Material, primitive::SphereData};

struct Image(image::DynamicImage);

impl sjrt::IColorBuffer for &mut Image {
    type Color = sjrt::Color;

    fn write(&mut self, x: u32, y: u32, color: sjrt::Color) {
        let data = match color {
            sjrt::Color::R8G8B8A8_Uint(data) => data,
            sjrt::Color::R32G32B32A32_Unorm(data) => [
                (data[0] * 255.0).clamp(0.0, 255.0) as u8,
                (data[1] * 255.0).clamp(0.0, 255.0) as u8,
                (data[2] * 255.0).clamp(0.0, 255.0) as u8,
                (data[3] * 255.0).clamp(0.0, 255.0) as u8,
            ],
        };

        self.0.put_pixel(x, y, image::Rgba::from(data));
    }
}

#[tokio::main]
async fn main() {
    let scene_data = sjrt::scene::Scene {
        sky: sjrt::scene::Sky {
            lower_color: nalgebra::Vector3::zeros(),
            upper_color: nalgebra::Vector3::new(0.2, 0.2, 0.6),
        },
        primitives: vec![
            sjrt::scene::primitive::Primitive::Sphere(SphereData { radius: 0.5f32 }),
            sjrt::scene::primitive::Primitive::Sphere(SphereData { radius: 0.5f32 }),
            sjrt::scene::primitive::Primitive::Sphere(SphereData { radius: 10.5f32 }),
            sjrt::scene::primitive::Primitive::Sphere(SphereData { radius: 5.5f32 }),
        ],
        transforms: vec![
            sjrt::scene::Transform::new_with_translation(&nalgebra::Vector3::new(0.0, 0.0, 0.0)),
            sjrt::scene::Transform::new_with_translation(&nalgebra::Vector3::new(0.0, 1.0, 0.0)),
            sjrt::scene::Transform::new_with_translation(&nalgebra::Vector3::new(3.0, 1.5, 55.0)),
            sjrt::scene::Transform::new_with_translation(&nalgebra::Vector3::new(-7.0, -1.0, 10.0)),
        ],
        materials: vec![
            Material {
                albedo: nalgebra::Vector3::new(0.1, 0.2, 1.0),
                emission: nalgebra::Vector3::new(0.1, 0.1, 0.1),
            },
            Material {
                albedo: nalgebra::Vector3::new(1.0, 0.2, 0.3),
                emission: nalgebra::Vector3::new(0.7, 0.7, 0.7),
            },
            Material {
                albedo: sjrt::Colors::white(),
                emission: sjrt::Colors::white(),
            },
            Material {
                albedo: sjrt::Colors::white(),
                emission: sjrt::Colors::white(),
            },
        ],
    };
    let scene = Arc::new(sjrt::util::RapierScene::new_from_scene(&scene_data));
    let mut buffer = Image(DynamicImage::new_rgba8(640, 480));

    let ray_tracing_pipeline = sjrt::NormalTracer::default();
    let rays = sjrt::Camera::builder()
        .with_position(&nalgebra::Vector3::new(0.0, 7.0, 20.0))
        .with_look_at(&nalgebra::Vector3::new(0.0, 5.0, 0.0))
        .with_field_of_view(std::f32::consts::PI / 4.0)
        .build()
        .calculate_ray_direction(640, 480);
    sjrt::Executor::default()
        .execute_async(&mut buffer, rays.into_iter(), scene, ray_tracing_pipeline)
        .await;

    buffer.0.save("parallel.png").unwrap();
}
