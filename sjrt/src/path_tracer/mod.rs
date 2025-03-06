mod default_kernel;
mod next_event_estimation;
mod path_tracer;
mod path_tracer_ex;

pub use default_kernel::DefaultKernel;
pub use next_event_estimation::NextEventEstimation;
pub use path_tracer::PathTracer;
pub use path_tracer_ex::{IHitParams, IKernel, PathTracerEx};

struct SamplingData<TColor> {
    pub emission: TColor,
    pub albedo: TColor,
}
