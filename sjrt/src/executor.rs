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

pub struct EntryParams {
    pub x: u32,
    pub y: u32,
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

    fn entry(&self, entry_params: &EntryParams) -> Self::PayloadType;

    fn react_closest_hit(
        &self,
        payload: Self::PayloadType,
        hit_params: &Self::HitParams,
    ) -> HitAction<Self::PayloadType, impl Iterator<Item = RayParams<Self::PayloadType>>>;

    fn react_hit_miss(&self, payload: Self::PayloadType) -> Self::PayloadType;

    fn trace(&self, ray_params: RayParams<Self::PayloadType>) -> TraceAction<Self::PayloadType>;
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
        for y in 0..480 {
            for x in 0..640 {
                // 初期値生成
                let payload = ray_tracing_pipeline.entry(&EntryParams { x, y });

                // 初期レイ
                // TODO: 外部から注入できるようにする
                let mut ray_params = RayParams {
                    from: Vector3f::new(0.0, 0.0, -10.0),
                    to: Vector3f::new(-320.0 + x as f32, -240.0 + y as f32, 1000.0),
                    payload,
                };

                // レイを飛ばすループ
                let final_payload = loop {
                    // 衝突判定
                    let cast_result = scene.cast(&ray_params.from, &ray_params.to);

                    // 衝突の結果による値の更新
                    let new_payload = match cast_result {
                        // 衝突した
                        Some(cast_result) => {
                            match ray_tracing_pipeline
                                .react_closest_hit(ray_params.payload, &cast_result)
                            {
                                HitAction::RayGenerate(_rays) => {
                                    todo!()
                                }
                                HitAction::Payload(payload) => payload,
                            }
                        }
                        // 衝突しなかった
                        None => ray_tracing_pipeline.react_hit_miss(ray_params.payload),
                    };

                    // つぎのアクション選定
                    let trace_action = ray_tracing_pipeline.trace(RayParams {
                        from: ray_params.from,
                        to: ray_params.to,
                        payload: new_payload,
                    });
                    match trace_action {
                        TraceAction::Next(next_ray_params) => ray_params = next_ray_params,
                        TraceAction::Finish(payload) => break payload,
                    }
                };
                //  end of loop --------------------------------------

                // 出力して終了
                payload_buffer.write(final_payload);
            }
        }
    }
}
