// TODO: 名前空間の修正
pub mod test {
    tonic::include_proto!("sjrt");
}

pub use test::ImageView;

mod system;
mod sampling_algorithm;
mod traits;
pub use traits::EnumerateLightResult;
pub use traits::IBuffer;
pub use traits::IRenderer;
pub use traits::IScene;
pub use sampling_algorithm::{DefaultSamplingEstimation, NextEventEstimation};
pub use system::{System, ParallelizeSystem};

mod bidirectional_reflectance_distribution_function;
pub use bidirectional_reflectance_distribution_function::IBidirectionalReflectanceDistributionFunction;

pub mod brdf;

mod camera;
pub use camera::Camera;

mod material_info;
pub use material_info::Brdf;
pub use material_info::MaterialInfo;

mod path_tracer;
pub use path_tracer::PathTracer;

pub mod image;

pub mod net;

mod property;
pub use property::Property;

mod ray;
pub use ray::Ray;

mod vector;
pub use vector::Vector3f;

mod rapier_scene;
pub use rapier_scene::RapierScene;

pub mod scene;
