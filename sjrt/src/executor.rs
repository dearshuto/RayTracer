use crate::Vector3f;

pub enum TraceAction<T> {
    Next(RayParams<T>),
    Finish(T),
}

pub enum HitAction<T, U>
where
    U: Iterator<Item = RayParams<T>>,
{
    RayGenerate(U),
    Payload(T),
}

pub struct RayParams<T> {
    pub from: Vector3f,
    pub to: Vector3f,
    pub payload: T,
}

pub trait ISceneStructure<T> {
    fn cast(&mut self, from: &Vector3f, to: &Vector3f) -> Option<T>;
}

pub trait IRayTracingPipeline {
    type PayloadType;
    type HitParams;

    fn entry(&self) -> impl Iterator<Item = Self::PayloadType>;

    fn trace(&self, payload: Self::PayloadType) -> TraceAction<Self::PayloadType>;

    fn react_closest_hit(
        &self,
        payload: Self::PayloadType,
        hit_params: &Self::HitParams,
    ) -> HitAction<Self::PayloadType, impl Iterator<Item = RayParams<Self::PayloadType>>>;

    fn react_hit_miss(&self, payload: Self::PayloadType) -> Self::PayloadType;
}

pub trait IPayloadBuffer<TPayload> {
    fn write(&mut self, payload: TPayload);
}

pub struct ExecuteParams<TRayTracingPipeline, TScene>
where
    TRayTracingPipeline: IRayTracingPipeline,
    TScene: ISceneStructure<TRayTracingPipeline::HitParams>,
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
        mut scene: TScene,
        ray_tracing_pipeline: TRayTracingPipeline,
    ) where
        TPayloadBuffer: IPayloadBuffer<TRayTracingPipeline::PayloadType>,
        TRayTracingPipeline: IRayTracingPipeline,
        TScene: ISceneStructure<TRayTracingPipeline::HitParams>,
    {
        for mut payload in ray_tracing_pipeline.entry() {
            let final_payload = loop {
                match ray_tracing_pipeline.trace(payload) {
                    // トレースが続くかぎりループを回す
                    TraceAction::Next(ray_params) => {
                        let next_payload = ray_params.payload;

                        // 衝突判定
                        let cast_result = scene.cast(&ray_params.from, &ray_params.to);

                        // 衝突の結果によって分岐しつつ次のループへ
                        let new_payload = match cast_result {
                            // 衝突した
                            Some(cast_result) => {
                                match ray_tracing_pipeline
                                    .react_closest_hit(next_payload, &cast_result)
                                {
                                    HitAction::RayGenerate(_rays) => {
                                        todo!()
                                    }
                                    HitAction::Payload(payload) => payload,
                                }
                            }
                            // 衝突しなかった
                            None => ray_tracing_pipeline.react_hit_miss(next_payload),
                        };
                        payload = new_payload;
                    }
                    // トレースの終了。ここでループを抜ける
                    TraceAction::Finish(payload) => break payload,
                }
            };

            payload_buffer.write(final_payload);
        }
    }
}
