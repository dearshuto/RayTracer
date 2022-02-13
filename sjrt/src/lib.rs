
// TODO: 名前空間の修正
pub mod test {
    tonic::include_proto!("sjrt");
}

pub use test::ImageView;

mod traits;
pub use traits::IBuffer;
pub use traits::IRenderer;
pub use traits::IScene;
pub use traits::System;

mod bidirectional_reflectance_distribution_function;
pub use bidirectional_reflectance_distribution_function::IBidirectionalReflectanceDistributionFunction;

pub mod brdf;

mod camera;
pub use camera::Camera;

mod default_sampling_estimation;
pub use default_sampling_estimation::DefaultSamplingEstimation;

mod material_info;
pub use material_info::MaterialInfo;
pub use material_info::Brdf;

mod path_tracer;
pub use path_tracer::PathTracer;

pub mod image;

mod nee;
pub use nee::NextEventEstimation;

pub mod net;

mod property;
pub use property::Property;

mod ray;
pub use ray::Ray;

mod vector;
pub use vector::Vector3f;

mod parallelize_system;
pub use parallelize_system::ParallelizeSystem;

mod rapier_scene;
pub use rapier_scene::RapierScene;

pub mod scene;
