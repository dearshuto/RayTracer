use std::{
    ops::{Add, Mul},
    sync::Arc,
};

use crate::{IConstract, IInnerProduct, camera::RayInfo, traits::INormalized};

pub enum Color {
    #[allow(non_camel_case_types)]
    R8G8B8A8_Uint([u8; 4]),
    #[allow(non_camel_case_types)]
    R32G32B32A32_Unorm([f32; 4]),
}

pub enum TraceAction<T, TPoint> {
    Next(RayParams<T, TPoint>),
    Finish(T),
}

pub enum HitAction<T, U, TPoint>
where
    U: Iterator<Item = RayParams<T, TPoint>>,
{
    RayGenerate(U),
    Payload(T),
}

pub struct EntryParams<T> {
    pub x: u32,
    pub y: u32,
    pub from: T,
    pub to: T,
}

pub struct RayParams<T, TPoint> {
    pub from: TPoint,
    pub to: TPoint,
    pub payload: T,
}

pub trait ISceneStructure<THitData, TPoint> {
    fn cast(&self, from: &TPoint, to: &TPoint) -> Option<THitData>;
}

pub trait IRayTracingPipeline {
    type PayloadType;
    type HitParams;
    type Point: Clone
        + IConstract<f32>
        + Add<Self::Point, Output = Self::Point>
        + Mul<f32, Output = Self::Point>;
    type Color: Into<Color>;

    fn entry(&self, entry_params: &EntryParams<Self::Point>) -> Self::PayloadType;

    fn react_closest_hit(
        &self,
        payload: Self::PayloadType,
        hit_params: &Self::HitParams,
    ) -> HitAction<
        Self::PayloadType,
        impl Iterator<Item = RayParams<Self::PayloadType, Self::Point>>,
        Self::Point,
    >;

    fn react_hit_miss(&self, payload: Self::PayloadType) -> Self::PayloadType;

    fn trace(
        &self,
        ray_params: RayParams<Self::PayloadType, Self::Point>,
    ) -> TraceAction<Self::PayloadType, Self::Point>;

    fn write(&self, payload: Self::PayloadType) -> Self::Color;
}

pub trait IColorBuffer {
    type Color;

    fn write(&mut self, x: u32, y: u32, color: Self::Color);
}

#[derive(Debug, Default)]
pub struct Executor;

impl Executor {
    pub fn execute<TColorBuffer, TRayTracingPipeline, TScene>(
        &self,
        mut color_buffer: TColorBuffer,
        rays: impl Iterator<Item = RayInfo<TRayTracingPipeline::Point>>,
        scene: TScene,
        ray_tracing_pipeline: TRayTracingPipeline,
    ) where
        TColorBuffer: IColorBuffer<Color = Color>,
        TRayTracingPipeline: IRayTracingPipeline,
        TScene: ISceneStructure<TRayTracingPipeline::HitParams, TRayTracingPipeline::Point>,
    {
        for ray in rays {
            let scene_adapter: SceneAdapter<'_, TScene, TRayTracingPipeline, _> = SceneAdapter {
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
            color_buffer.write(x, y, color.into());
        }
    }

    pub async fn execute_async<TColorBuffer, TPayload, TRayTracingPipeline, TScene, TPoint>(
        &self,
        mut color_buffer: TColorBuffer,
        rays: impl Iterator<Item = RayInfo<TRayTracingPipeline::Point>>,
        scene: TScene,
        ray_tracing_pipeline: TRayTracingPipeline,
    ) where
        TColorBuffer: IColorBuffer<Color = Color>,
        TPayload: 'static + Send,
        TRayTracingPipeline: 'static
            + IRayTracingPipeline<PayloadType = TPayload, Point = TPoint>
            + Clone
            + Sync
            + Send,
        TScene: 'static
            + ISceneStructure<TRayTracingPipeline::HitParams, TRayTracingPipeline::Point>
            + Clone
            + Sync
            + Send,
        TPoint: 'static
            + Send
            + Sync
            + IConstract<f32>
            + INormalized
            + IInnerProduct<f32>
            + Mul<f32, Output = TPoint>
            + Add<TPoint, Output = TPoint>,
    {
        // 初期レイのメモ化
        let mut rays: Vec<_> = rays.collect();

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
                color_buffer.write(x, y, color.into());
            }
        }
    }

    fn execute_impl<TRayTracingPipeline, TScene>(
        ray: RayInfo<TRayTracingPipeline::Point>,
        scene: TScene,
        ray_tracing_pipeline: TRayTracingPipeline,
    ) -> TRayTracingPipeline::PayloadType
    where
        TRayTracingPipeline: IRayTracingPipeline,
        TScene: ISceneStructure<TRayTracingPipeline::HitParams, TRayTracingPipeline::Point>,
    {
        let x = ray.x;
        let y = ray.y;
        let direction = ray.directions[0].clone();

        // 初期レイ
        let from = TRayTracingPipeline::Point::new(0.0, 7.0, 20.0);
        let to = from.clone() + direction * 1000.0;

        // 初期値生成
        let payload = ray_tracing_pipeline.entry(&EntryParams {
            x,
            y,
            from: from.clone(),
            to: to.clone(),
        });

        // TODO: 外部から注入できるようにする
        let mut ray_params = RayParams { from, to, payload };

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

struct SceneAdapter<'a, TScene, TPipeline, TVector>
where
    TScene: ISceneStructure<TPipeline::HitParams, TVector>,
    TPipeline: IRayTracingPipeline,
{
    scene: &'a TScene,
    _marker: std::marker::PhantomData<(TPipeline, TVector)>,
}

impl<'a, TScene, TPipeline, TVectotr> ISceneStructure<TPipeline::HitParams, TVectotr>
    for SceneAdapter<'a, TScene, TPipeline, TVectotr>
where
    TScene: ISceneStructure<TPipeline::HitParams, TVectotr>,
    TPipeline: IRayTracingPipeline,
{
    fn cast(&self, from: &TVectotr, to: &TVectotr) -> Option<TPipeline::HitParams> {
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
    type Point = TRayTracingPipeline::Point;
    type Color = TRayTracingPipeline::Color;

    fn entry(&self, entry_params: &EntryParams<Self::Point>) -> Self::PayloadType {
        self.pipeline.entry(entry_params)
    }

    fn react_closest_hit(
        &self,
        payload: Self::PayloadType,
        hit_params: &Self::HitParams,
    ) -> HitAction<
        Self::PayloadType,
        impl Iterator<Item = RayParams<Self::PayloadType, Self::Point>>,
        Self::Point,
    > {
        self.pipeline.react_closest_hit(payload, hit_params)
    }

    fn react_hit_miss(&self, payload: Self::PayloadType) -> Self::PayloadType {
        self.pipeline.react_hit_miss(payload)
    }

    fn trace(
        &self,
        ray_params: RayParams<Self::PayloadType, Self::Point>,
    ) -> TraceAction<Self::PayloadType, Self::Point> {
        self.pipeline.trace(ray_params)
    }

    fn write(&self, payload: Self::PayloadType) -> Self::Color {
        self.pipeline.write(payload)
    }
}

// 任意の ISceneStructure を Arc でくるんだ型をパイプするための impl
impl<T, THitParams, TVector> ISceneStructure<THitParams, TVector> for Arc<T>
where
    T: ISceneStructure<THitParams, TVector>,
{
    fn cast(&self, from: &TVector, to: &TVector) -> Option<THitParams> {
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
    type Point = T::Point;
    type Color = T::Color;

    fn entry(&self, entry_params: &EntryParams<Self::Point>) -> Self::PayloadType {
        self.as_ref().entry(entry_params)
    }

    fn react_closest_hit(
        &self,
        payload: Self::PayloadType,
        hit_params: &Self::HitParams,
    ) -> HitAction<
        Self::PayloadType,
        impl Iterator<Item = RayParams<Self::PayloadType, Self::Point>>,
        Self::Point,
    > {
        self.as_ref().react_closest_hit(payload, hit_params)
    }

    fn react_hit_miss(&self, payload: Self::PayloadType) -> Self::PayloadType {
        self.as_ref().react_hit_miss(payload)
    }

    fn trace(
        &self,
        ray_params: RayParams<Self::PayloadType, Self::Point>,
    ) -> TraceAction<Self::PayloadType, Self::Point> {
        self.as_ref().trace(ray_params)
    }

    fn write(&self, payload: Self::PayloadType) -> Self::Color {
        self.as_ref().write(payload)
    }
}
