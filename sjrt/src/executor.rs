use crate::Vector3f;

pub enum Color {
    #[allow(non_camel_case_types)]
    R8G8B8A8_Uint([u8; 4]),
    #[allow(non_camel_case_types)]
    R32G32B32A32_Unorm([f32; 4]),
}

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
    fn cast(&self, from: &Vector3f, to: &Vector3f) -> Option<T>;
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

    fn write(&self, payload: Self::PayloadType) -> Color;
}

pub trait IColorBuffer {
    fn write(&mut self, x: u32, y: u32, color: Color);
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
    pub fn execute<TColorBuffer, TRayTracingPipeline, TScene>(
        &self,
        mut color_buffer: TColorBuffer,
        scene: TScene,
        ray_tracing_pipeline: TRayTracingPipeline,
    ) where
        TColorBuffer: IColorBuffer,
        TRayTracingPipeline: IRayTracingPipeline,
        TScene: ISceneStructure<TRayTracingPipeline::HitParams>,
    {
        let camera = crate::Camera::builder()
            .with_position(&Vector3f::new(0.0, 0.0, -10.0))
            .with_look_at(&Vector3f::new(0.0, 0.0, 0.0))
            .with_field_of_view(std::f32::consts::PI / 6.0)
            .build();

        let rays = camera.calculate_ray_direction_range(640, 480, 0..640, 0..480);
        for ray in rays {
            let x = ray.x;
            let y = ray.y;
            let direction = ray.directions[0];

            // 初期値生成
            let payload = ray_tracing_pipeline.entry(&EntryParams { x, y });

            // 初期レイ
            // TODO: 外部から注入できるようにする
            let mut ray_params = RayParams {
                from: Vector3f::new(0.0, 0.0, -10.0),
                to: 1000.0 * direction,
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
            let color = ray_tracing_pipeline.write(final_payload);
            color_buffer.write(x, y, color);
        }
    }
}
