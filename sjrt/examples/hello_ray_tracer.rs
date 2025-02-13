use clap::Parser;
use image::GenericImage;
use sjrt::scene::{primitive::SphereData, Material};

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    #[clap(short = 's', long = "sampling-count", default_value_t = 16)]
    sampling_count: u16,
}

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

pub fn main() {
    let _args = Args::parse();
    let scene_data = sjrt::scene::Scene {
        sky: sjrt::scene::Sky {
            lower_color: nalgebra::Vector3::zeros(),
            upper_color: nalgebra::Vector3::new(0.2, 0.2, 0.6),
        },
        primitives: vec![
            sjrt::scene::primitive::Primitive::Sphere(SphereData { radius: 0.5f32 }),
            sjrt::scene::primitive::Primitive::Sphere(SphereData { radius: 0.5f32 }),
        ],
        transforms: vec![
            sjrt::scene::Transform::new_with_translation(&nalgebra::Vector3::new(0.0, 0.0, 0.0)),
            sjrt::scene::Transform::new_with_translation(&nalgebra::Vector3::new(0.0, 1.0, 0.0)),
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
        ],
    };
    let scene = sjrt::util::RapierScene::new_from_scene(&scene_data);
    let renderer = sjrt::PathTracerEx::default();
    let mut buffer = Image(image::DynamicImage::new_rgba8(640, 480));

    sjrt::Executor::default().execute(&mut buffer, scene, renderer);

    buffer.0.save("example.png").unwrap();
}
