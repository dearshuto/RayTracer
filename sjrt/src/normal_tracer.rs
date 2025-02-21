use crate::{util::HitParams, Color, HitAction, IHitParams, IRayTracingPipeline, TraceAction};

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
    type Color = Color;

    fn entry(&self, _entry_params: &crate::EntryParams) -> Self::PayloadType {
        Payload {
            normal: nalgebra::Vector3::zeros(),
        }
    }

    fn react_closest_hit(
        &self,
        payload: Self::PayloadType,
        hit_params: &Self::HitParams,
    ) -> crate::HitAction<
        Self::PayloadType,
        impl Iterator<Item = crate::RayParams<Self::PayloadType>>,
    > {
        if false {
            return HitAction::RayGenerate([].into_iter());
        }

        HitAction::Payload(payload.with_normal(hit_params.normal()))
    }

    fn react_hit_miss(&self, payload: Self::PayloadType) -> Self::PayloadType {
        payload.with_normal(nalgebra::Vector3::zeros())
    }

    fn trace(
        &self,
        ray_params: crate::RayParams<Self::PayloadType>,
    ) -> crate::TraceAction<Self::PayloadType> {
        TraceAction::Finish(ray_params.payload)
    }

    fn write(&self, payload: Self::PayloadType) -> crate::Color {
        let x = payload.normal.x.clamp(0.0, 1.0);
        let y = payload.normal.y.clamp(0.0, 1.0);
        let z = payload.normal.z.clamp(0.0, 1.0);
        Color::R32G32B32A32_Unorm([x, y, z, 1.0])
    }
}
