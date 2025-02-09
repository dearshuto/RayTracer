use image::GenericImage;
use sjrt::Vector3f;

struct Image(image::DynamicImage);

impl sjrt::IColorBuffer for &mut Image {
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

fn main() {
    let scene_data = sjrt::scene::Scene {
        sky: sjrt::scene::Sky {
            lower_color: sjrt::Vector3f::zero(),
            upper_color: sjrt::Vector3f::new(0.2, 0.2, 0.6),
        },
        primitives: vec![sjrt::scene::primitive::Primitive::Sphere(
            sjrt::scene::primitive::SphereData { radius: 5.0f32 },
        )],
        transforms: vec![sjrt::scene::Transform::new_with_translation(
            &Vector3f::new(0.0, 0.0, 10.0),
        )],
        materials: vec![sjrt::scene::Material {
            albedo: Vector3f::new(0.1, 0.2, 1.0),
            emission: Vector3f::new(0.1, 0.1, 0.1),
        }],
    };
    let scene = sjrt::util::RapierScene::new_from_scene(&scene_data);
    let pipeline = sjrt::PathTracerEx::default();
    let mut buffer = Image(image::DynamicImage::new_rgba8(640, 480));
    sjrt::Executor::default().execute(&mut buffer, scene, pipeline);

    buffer.0.save("executor.png").unwrap();
}
