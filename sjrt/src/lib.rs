mod executor;
mod normal_tracer;
mod path_tracer_ex;
mod sampling_algorithm;
mod system;
mod traits;

pub use executor::{
    Color, EntryParams, ExecuteParams, Executor, HitAction, IColorBuffer, IRayTracingPipeline,
    ISceneStructure, RayParams, TraceAction,
};
pub use normal_tracer::NormalTracer;
pub use path_tracer_ex::{DefaultKernel, IHitParams, IKernel, PathTracerEx};
// パストレーサーのデフォルト実装を提供しておく
// TODO: PathTracerEx はカスタマイズ用として使用するので命名を変更する
pub type PathTracer = PathTracerEx<util::HitParams, DefaultKernel>;
pub use sampling_algorithm::{DefaultSamplingEstimation, NextEventEstimation};
pub use system::ParallelizeSystem;
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

mod property;
pub use property::Property;

pub mod scene;

pub mod util;

mod types;
pub use types::Colors;
