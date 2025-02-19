use clap::Parser;
use sjrt::scene::{primitive::SphereData, Material};

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    #[clap(short = 's', long = "sampling-count", default_value_t = 16)]
    sampling_count: u16,
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
    let renderer = sjrt::PathTracerEx::default()
        .with_depth(4)
        .with_sampling_count(64);
    let mut buffer = sjrt::util::ImageBuffer::new(640, 480);
    let rays = sjrt::Camera::builder()
        .with_position(&nalgebra::Vector3::new(0.0, 0.0, -10.0))
        .with_look_at(&nalgebra::Vector3::new(0.0, 0.0, 0.0))
        .with_field_of_view(std::f32::consts::PI / 6.0)
        .build()
        .calculate_ray_direction();

    sjrt::Executor::default().execute(&mut buffer, rays.into_iter(), scene, renderer);

    buffer.save("example.png");
}
