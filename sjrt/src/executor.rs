use std::sync::Arc;

use crate::{camera::RayInfo, util::HitParams, Vector3f};

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
            let scene_adapter: SceneAdapter<'_, TScene, TRayTracingPipeline> = SceneAdapter {
                scene: &scene,
                _marker: std::marker::PhantomData::default(),
            };
            let pipeline_adapter = PipelineAdapter {
                pipeline: &ray_tracing_pipeline,
            };
            let x = ray.x;
            let y = ray.y;
            let final_payload = Self::execute_impl(ray, scene_adapter, pipeline_adapter);

            // 出力して終了
            let color = ray_tracing_pipeline.write(final_payload);
            color_buffer.write(x, y, color);
        }
    }

    pub async fn execute_async<TColorBuffer, TPayload, TRayTracingPipeline, TScene>(
        &self,
        mut color_buffer: TColorBuffer,
        scene: TScene,
        ray_tracing_pipeline: TRayTracingPipeline,
    ) where
        TColorBuffer: IColorBuffer,
        TPayload: 'static + Send,
        TRayTracingPipeline:
            'static + IRayTracingPipeline<PayloadType = TPayload> + Clone + Sync + Send,
        TScene: 'static + ISceneStructure<TRayTracingPipeline::HitParams> + Clone + Sync + Send,
    {
        // 初期レイの生成
        let camera = crate::Camera::builder()
            .with_position(&Vector3f::new(0.0, 0.0, -10.0))
            .with_look_at(&Vector3f::new(0.0, 0.0, 0.0))
            .with_field_of_view(std::f32::consts::PI / 6.0)
            .build();
        let mut rays = camera.calculate_ray_direction_range(640, 480, 0..640, 0..480);

        // 初期レイを分割してそれぞれにレンダリングタスクを割り当てていく
        let mut tasks = Vec::default();
        while !rays.is_empty() {
            // 適当に横一列 (640 決めうち) をひとつのタスクとしている
            // TODO: 外部から指定できるようにする
            let len = rays.len();
            let range = len.saturating_sub(640)..len;
            let chunks: Vec<_> = rays.drain(range).collect();

            // 必要なインスタンスはクローンしてタスクに所有権ごと渡す
            let scene_local = scene.clone();
            let ray_tracing_pipeline_local = ray_tracing_pipeline.clone();
            let task = tokio::spawn(async move {
                let payloads: Vec<_> = chunks
                    .into_iter()
                    .map(|ray_info| {
                        let x = ray_info.x;
                        let y = ray_info.y;
                        let payload = Self::execute_impl(
                            ray_info,
                            scene_local.clone(),
                            ray_tracing_pipeline_local.clone(),
                        );

                        (x, y, payload)
                    })
                    .collect();
                payloads
            });
            tasks.push(task);
        }

        // 完了待ち
        let payload_vecs = futures::future::join_all(tasks).await;

        // 結果を出力
        // MEMO: 出力処理自体も並列化したほうがよいかも
        for payload_vec in payload_vecs {
            for (x, y, payload) in payload_vec.unwrap() {
                let color = ray_tracing_pipeline.write(payload);
                color_buffer.write(x, y, color);
            }
        }
    }

    fn execute_impl<TRayTracingPipeline, TScene>(
        ray: RayInfo,
        scene: TScene,
        ray_tracing_pipeline: TRayTracingPipeline,
    ) -> TRayTracingPipeline::PayloadType
    where
        TRayTracingPipeline: IRayTracingPipeline,
        TScene: ISceneStructure<TRayTracingPipeline::HitParams>,
    {
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

        loop {
            // 衝突判定
            let cast_result = scene.cast(&ray_params.from, &ray_params.to);

            // 衝突の結果による値の更新
            let new_payload = match cast_result {
                // 衝突した
                Some(cast_result) => {
                    match ray_tracing_pipeline.react_closest_hit(ray_params.payload, &cast_result) {
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
        }
    }
}

struct SceneAdapter<'a, TScene, TPipeline>
where
    TScene: ISceneStructure<TPipeline::HitParams>,
    TPipeline: IRayTracingPipeline,
{
    scene: &'a TScene,
    _marker: std::marker::PhantomData<TPipeline>,
}

impl<'a, TScene, TPipeline> ISceneStructure<TPipeline::HitParams>
    for SceneAdapter<'a, TScene, TPipeline>
where
    TScene: ISceneStructure<TPipeline::HitParams>,
    TPipeline: IRayTracingPipeline,
{
    fn cast(&self, from: &Vector3f, to: &Vector3f) -> Option<TPipeline::HitParams> {
        self.scene.cast(from, to)
    }
}

struct PipelineAdapter<'a, TRayTracingPipeline>
where
    TRayTracingPipeline: IRayTracingPipeline,
{
    pipeline: &'a TRayTracingPipeline,
}

impl<'a, TRayTracingPipeline> IRayTracingPipeline for PipelineAdapter<'a, TRayTracingPipeline>
where
    TRayTracingPipeline: IRayTracingPipeline,
{
    type PayloadType = TRayTracingPipeline::PayloadType;
    type HitParams = TRayTracingPipeline::HitParams;

    fn entry(&self, entry_params: &EntryParams) -> Self::PayloadType {
        self.pipeline.entry(entry_params)
    }

    fn react_closest_hit(
        &self,
        payload: Self::PayloadType,
        hit_params: &Self::HitParams,
    ) -> HitAction<Self::PayloadType, impl Iterator<Item = RayParams<Self::PayloadType>>> {
        self.pipeline.react_closest_hit(payload, hit_params)
    }

    fn react_hit_miss(&self, payload: Self::PayloadType) -> Self::PayloadType {
        self.pipeline.react_hit_miss(payload)
    }

    fn trace(&self, ray_params: RayParams<Self::PayloadType>) -> TraceAction<Self::PayloadType> {
        self.pipeline.trace(ray_params)
    }

    fn write(&self, payload: Self::PayloadType) -> Color {
        self.pipeline.write(payload)
    }
}

// 任意の ISceneStructure を Arc でくるんだ型をパイプするための impl
impl<T> ISceneStructure<HitParams> for Arc<T>
where
    T: ISceneStructure<HitParams>,
{
    fn cast(&self, from: &Vector3f, to: &Vector3f) -> Option<HitParams> {
        self.as_ref().cast(from, to)
    }
}

// 任意の IRayTracingPipeline を Arc でくるんだ型をパイプするための impl
impl<T> IRayTracingPipeline for Arc<T>
where
    T: IRayTracingPipeline,
{
    type PayloadType = T::PayloadType;
    type HitParams = T::HitParams;

    fn entry(&self, entry_params: &EntryParams) -> Self::PayloadType {
        self.as_ref().entry(entry_params)
    }

    fn react_closest_hit(
        &self,
        payload: Self::PayloadType,
        hit_params: &Self::HitParams,
    ) -> HitAction<Self::PayloadType, impl Iterator<Item = RayParams<Self::PayloadType>>> {
        self.as_ref().react_closest_hit(payload, hit_params)
    }

    fn react_hit_miss(&self, payload: Self::PayloadType) -> Self::PayloadType {
        self.as_ref().react_hit_miss(payload)
    }

    fn trace(&self, ray_params: RayParams<Self::PayloadType>) -> TraceAction<Self::PayloadType> {
        self.as_ref().trace(ray_params)
    }

    fn write(&self, payload: Self::PayloadType) -> Color {
        self.as_ref().write(payload)
    }
}
