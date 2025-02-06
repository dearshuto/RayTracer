use crate::{IScene, Vector3f};

pub enum HitAction<T, U>
where
    U: Iterator<Item = RayParams>,
{
    RayGenerate(U),
    Payload(T),
}

pub struct RayParams {
    pub from: Vector3f,
    pub to: Vector3f,
}

pub trait IReceiver {
    type PayloadType;

    fn receive(self, payload: Self::PayloadType);
}

pub trait IRayTracingPipeline {
    type RayId: Copy;
    type PayloadType;

    fn generate_rays(&self) -> impl Iterator<Item = (Self::RayId, RayParams)>;

    fn react_closest_hit(&self) -> HitAction<Self::PayloadType, impl Iterator<Item = RayParams>>;

    fn react_hit_miss(&self) -> Self::PayloadType;
}

pub trait IPayloadBuffer<TId, TPayload> {
    fn write(&mut self, id: TId, payload: TPayload);
}

pub struct ExecuteParams<TRayTracingPipeline, TScene>
where
    TRayTracingPipeline: IRayTracingPipeline,
    TScene: IScene,
{
    pub scene: TScene,
    pub ray_tracing_pipeline: TRayTracingPipeline,
}

#[derive(Debug, Default)]
pub struct Executor;

impl Executor {
    pub fn execute<TPayloadBuffer, TRayTracingPipeline, TScene>(
        &self,
        mut payload_buffer: TPayloadBuffer,
        scene: TScene,
        ray_tracing_pipeline: TRayTracingPipeline,
    ) where
        TPayloadBuffer:
            IPayloadBuffer<TRayTracingPipeline::RayId, TRayTracingPipeline::PayloadType>,
        TRayTracingPipeline: IRayTracingPipeline,
        TScene: IScene,
    {
        let rays = ray_tracing_pipeline.generate_rays();

        for (id, ray) in rays {
            let payload = Self::trace(&ray, &ray_tracing_pipeline, &scene);
            payload_buffer.write(id, payload);
        }
    }

    fn trace<TPipeline, TScene>(
        ray_params: &RayParams,
        ray_tracing_pipeline: &TPipeline,
        scene: &TScene,
    ) -> TPipeline::PayloadType
    where
        TPipeline: IRayTracingPipeline,
        TScene: IScene,
    {
        let cast_result = scene.cast_ray(&ray_params.from, &ray_params.to);

        match cast_result {
            Some(_cast_result) => match ray_tracing_pipeline.react_closest_hit() {
                HitAction::RayGenerate(_rays) => {
                    todo!()
                }
                HitAction::Payload(payload) => payload,
            },
            None => ray_tracing_pipeline.react_hit_miss(),
        }
    }
}
