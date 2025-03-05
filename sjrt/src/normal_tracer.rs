use crate::{IHitParams, IRayTracingPipeline, TraceAction, util::HitParams};

#[derive(sjrt_macro::Immutable)]
pub struct Payload {
    normal: nalgebra::Vector3<f32>,
}

// レイを飛ばして法線を取得するトレーサー
#[derive(Default, Clone)]
pub struct NormalTracer;

impl IRayTracingPipeline for NormalTracer {
    type PayloadType = Payload;
    type HitParams = HitParams;
    type Point = nalgebra::Vector3<f32>;
    type Color = nalgebra::Vector3<f32>;

    fn entry(&self, _entry_params: &crate::EntryParams<Self::Point>) -> Self::PayloadType {
        Payload {
            normal: nalgebra::Vector3::zeros(),
        }
    }

    fn react_closest_hit(
        &self,
        payload: Self::PayloadType,
        hit_params: &Self::HitParams,
        _func: impl Fn(&Self::Point, &Self::Point) -> Option<Self::HitParams>,
    ) -> Self::PayloadType {
        payload.with_normal(hit_params.normal())
    }

    fn react_hit_miss(&self, payload: Self::PayloadType) -> Self::PayloadType {
        payload.with_normal(nalgebra::Vector3::zeros())
    }

    fn trace(
        &self,
        ray_params: crate::RayParams<Self::PayloadType, Self::Point>,
    ) -> crate::TraceAction<Self::PayloadType, Self::Point> {
        TraceAction::Finish(ray_params.payload)
    }

    fn write(&self, payload: Self::PayloadType) -> Self::Color {
        let x = payload.normal.x.clamp(0.0, 1.0);
        let y = payload.normal.y.clamp(0.0, 1.0);
        let z = payload.normal.z.clamp(0.0, 1.0);
        nalgebra::Vector3::new(x, y, z)
    }
}
