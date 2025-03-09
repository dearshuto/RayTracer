use crate::{IRayTracingPipeline, util::HitParams};

#[derive(Default, Clone)]
pub struct DirectionTracer;

#[derive(Default)]
pub struct DirectionTracerPayload {
    direction: nalgebra::Vector3<f32>,
}

impl IRayTracingPipeline for DirectionTracer {
    type PayloadType = DirectionTracerPayload;
    type HitParams = HitParams;
    type Point = nalgebra::Vector3<f32>;
    type Color = nalgebra::Vector3<f32>;

    fn entry(&self, entry_params: &crate::EntryParams<Self::Point>) -> Self::PayloadType {
        let direction = (entry_params.to - entry_params.from).normalize();
        DirectionTracerPayload { direction }
    }

    fn react_closest_hit<TSceneStructure>(
        &self,
        payload: Self::PayloadType,
        _hit_params: Self::HitParams,
        _scene_structure: TSceneStructure,
    ) -> Self::PayloadType
    where
        TSceneStructure: crate::ISceneStructure<Self::HitParams, Self::Point>,
    {
        payload
    }

    fn react_hit_miss(&self, payload: Self::PayloadType) -> Self::PayloadType {
        payload
    }

    fn trace(
        &self,
        ray_params: crate::RayParams<Self::PayloadType, Self::Point>,
    ) -> crate::TraceAction<Self::PayloadType, Self::Point> {
        crate::TraceAction::Finish(ray_params.payload)
    }

    fn write(&self, payload: Self::PayloadType) -> Self::Color {
        payload.direction
    }
}
