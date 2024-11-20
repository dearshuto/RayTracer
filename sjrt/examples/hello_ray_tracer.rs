use clap::Parser;
use sjrt::{
    scene::{primitive::SphereData, Material},
    Vector3f,
};

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    #[clap(short = 's', long = "sampling-count", default_value_t = 16)]
    sampling_count: u16,
}

pub fn main() {
    let args = Args::parse();
    let scene_data = sjrt::scene::Scene {
        sky: sjrt::scene::Sky {
            lower_color: sjrt::Vector3f::zero(),
            upper_color: sjrt::Vector3f::new(0.2, 0.2, 0.6),
        },
        primitives: vec![
            sjrt::scene::primitive::Primitive::Sphere(SphereData { radius: 0.5f32 }),
            sjrt::scene::primitive::Primitive::Sphere(SphereData { radius: 0.5f32 }),
        ],
        transforms: vec![
            sjrt::scene::Transform::new_with_translation(&Vector3f::new(0.0, 0.0, 0.0)),
            sjrt::scene::Transform::new_with_translation(&Vector3f::new(0.0, 1.0, 0.0)),
        ],
        materials: vec![
            Material {
                albedo: Vector3f::new(0.1, 0.2, 1.0),
                emission: Vector3f::new(0.1, 0.1, 0.1),
            },
            Material {
                albedo: Vector3f::new(1.0, 0.2, 0.3),
                emission: Vector3f::new(0.7, 0.7, 0.7),
            },
        ],
    };
    let scene = sjrt::util::RapierScene::new_from_scene(&scene_data);
    let renderer = sjrt::PathTracer::new(args.sampling_count, 4, false);
    let mut buffer = sjrt::util::ImageBuffer::new(480, 480);
    let system = sjrt::System::new();
    system.execute(&scene, &mut buffer, &renderer);

    buffer.save("example.png");
}
