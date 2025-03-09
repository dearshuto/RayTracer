mod direction_tracer;
mod executor;
mod normal_tracer;
mod path_tracer;
mod sampling_algorithm;
mod system;
mod traits;

pub use executor::{
    Color, EntryParams, Executor, IColorBuffer, IRayTracingPipeline, ISceneStructure, RayParams,
    TraceAction,
};
pub use normal_tracer::NormalTracer;

// 旧実装と命名が衝突しないように Ex の接尾語をつけておく
// 将来的に Ex を取り除いて本実装とする
pub use path_tracer::NextEventEstimation as NextEventEstimationEx;

pub use direction_tracer::DirectionTracer;
pub use path_tracer::{IHitParams, IKernel, PathTracer, PathTracerEx};
pub use sampling_algorithm::{DefaultSamplingEstimation, NextEventEstimation};
pub use system::ParallelizeSystem;
pub use traits::EnumerateLightResult;
pub use traits::IBuffer;
pub use traits::IRenderer;
pub use traits::{
    IConstract, IInnerProduct, INorm, IOuterProduct, IScene, IUniHemisphereUniformDistribution,
};

mod bidirectional_reflectance_distribution_function;
pub use bidirectional_reflectance_distribution_function::IBidirectionalReflectanceDistributionFunction;

pub mod brdf;

mod camera;
pub use camera::Camera;

mod material_info;
pub use material_info::Brdf;
pub use material_info::MaterialInfo;

mod property;
pub use property::Property;

pub mod scene;

pub mod util;

mod types;
pub use types::Colors;
