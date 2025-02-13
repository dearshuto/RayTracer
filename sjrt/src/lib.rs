mod executor;
mod path_tracer_ex;
mod sampling_algorithm;
mod system;
mod traits;

pub use executor::{
    Color, EntryParams, ExecuteParams, Executor, HitAction, IColorBuffer, IRayTracingPipeline,
    ISceneStructure, RayParams, TraceAction,
};
pub use path_tracer_ex::{IHitParams, IKernel, PathTracerEx};
pub use sampling_algorithm::{DefaultSamplingEstimation, NextEventEstimation};
pub use system::{ParallelizeSystem, System};
pub use traits::EnumerateLightResult;
pub use traits::IBuffer;
pub use traits::IRenderer;
pub use traits::IScene;

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

mod property;
pub use property::Property;

mod ray;
pub use ray::Ray;

mod vector;
pub use vector::Vector3f;

pub mod scene;

pub mod util;

mod types;
pub use types::Colors;
