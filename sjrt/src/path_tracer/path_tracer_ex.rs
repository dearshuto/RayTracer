use std::ops::{Add, Div, Mul, Sub};

use crate::{
    EntryParams, IConstract, IInnerProduct, IRayTracingPipeline, RayParams,
    traits::{IComponentMul, INormalized, IRandomEngine},
};

use num::Zero;

use super::DefaultKernel;

pub struct SamplingData<TColor> {
    pub emission: TColor,
    pub albedo: TColor,
}

pub trait IPathTracerPlugin {
    type MaterialId;
    type Point;
    type Color;
    type HitParams: IHitParams<Self::MaterialId, Self::Point, Self::Color>;
    type Payload;

    fn entry(&self, entry_params: &EntryParams<Self::Point>) -> Self::Payload;

    fn react_closest_hit(
        &self,
        depth: u32,
        payload: &Self::Payload,
        hit_params: &Self::HitParams,
        func: impl Fn(&Self::Point, &Self::Point) -> Option<Self::HitParams>,
    ) -> SamplingData<Self::Color>;

    fn react_hit_miss(&self, payload: &Self::Payload) -> SamplingData<Self::Color>;
}

pub trait IHitParams<TId, TPoint, TColor> {
    fn id(&self) -> TId;

    fn normal(&self) -> TPoint;

    fn position(&self) -> TPoint;

    fn emission(&self) -> TColor;

    fn albedo(&self) -> TColor;
}

pub trait IKernel {
    type MaterialId: Copy;
    type ReflectionEstimationContext;
    type RondomEngine: IRandomEngine<f32>;
    type Point: Clone
        + IConstract<f32>
        + INormalized
        + IInnerProduct<f32>
        + Mul<f32, Output = Self::Point>
        + Add<Self::Point, Output = Self::Point>
        // Point に引き算を定義しているがあまり直感的ではない気がする
        // 例えば Vector を関連型に追加して、IKernel に 2 つの Point を渡したら、
        // Vector を生成するようなインターフェースにしてもいいかも
        + Sub<Self::Point, Output = Self::Point>;
    type Color: Clone
        + Into<crate::Color>
        + num::Zero
        + Add<Self::Color, Output = Self::Color>
        + Mul<f32, Output = Self::Color>
        + Div<f32, Output = Self::Color>
        + IComponentMul;
    type HitParams: IHitParams<Self::MaterialId, Self::Point, Self::Color>;

    fn random_engine(&self) -> Self::RondomEngine;

    fn new_point(&self, x: f32, y: f32, z: f32) -> Self::Point;

    fn new_reflection_estimation_context(&self) -> Self::ReflectionEstimationContext;

    fn estimate_next_reflection(
        &self,
        id: Self::MaterialId,
        context: &mut Self::ReflectionEstimationContext,
        in_direction: &Self::Point,
        normal: &Self::Point,
    ) -> Self::Point;
}

#[derive(sjrt_macro::Immutable)]
pub struct Payload<T, U>
where
    T: IKernel,
{
    // 最初にレイを飛ばしたときの始点と終点
    from: T::Point,
    to: T::Point,
    // 蓄積した色
    value: T::Color,

    current_depth: u32,
    current_sampling: u32,

    latest_hit_material_id: Option<T::MaterialId>,
    latest_hit_position: T::Point,
    latest_hit_normal: T::Point,

    next_reflection_context: T::ReflectionEstimationContext,

    kernel: T,

    // (emission, albedo)
    hit_history: Vec<SamplingData<T::Color>>,

    plugin_payload: U,
}

pub struct PathTracerEx<TKernel, TPlugin>
where
    TKernel: IKernel,
    TPlugin: IPathTracerPlugin<Point = TKernel::Point, Color = TKernel::Color>,
{
    depth: u32,
    sampling_count: u32,
    kernel: TKernel,
    plugin: TPlugin,
}

impl Default for PathTracerEx<DefaultKernel, SimplePlugin<DefaultKernel>> {
    fn default() -> Self {
        let kernel = DefaultKernel {};
        let plugin = SimplePlugin::new();
        Self::new(kernel, plugin)
    }
}

impl<TPlugin> PathTracerEx<DefaultKernel, TPlugin>
where
    TPlugin: IPathTracerPlugin<
            MaterialId = <DefaultKernel as IKernel>::MaterialId,
            Point = <DefaultKernel as IKernel>::Point,
            Color = <DefaultKernel as IKernel>::Color,
            HitParams = <DefaultKernel as IKernel>::HitParams,
        >,
{
    pub fn default_with(plugin: TPlugin) -> Self {
        let kernel = DefaultKernel {};
        Self::new(kernel, plugin)
    }
}

impl<TKernel, TPlugin> PathTracerEx<TKernel, TPlugin>
where
    TKernel: IKernel,
    TPlugin: IPathTracerPlugin<Point = TKernel::Point, Color = TKernel::Color>,
{
    pub fn new(kernel: TKernel, plugin: TPlugin) -> Self {
        Self {
            depth: 1,
            sampling_count: 1,
            plugin,
            kernel,
        }
    }

    pub fn with_depth(mut self, depth: u32) -> Self {
        self.depth = depth;
        self
    }

    pub fn with_sampling_count(mut self, count: u32) -> Self {
        self.sampling_count = count;
        self
    }
}

impl<TKernel, TPlugin> IRayTracingPipeline for PathTracerEx<TKernel, TPlugin>
where
    TKernel: IKernel + Clone,
    TPlugin: IPathTracerPlugin<
            Point = TKernel::Point,
            Color = TKernel::Color,
            HitParams = TKernel::HitParams,
        >,
{
    type PayloadType = Payload<TKernel, TPlugin::Payload>;
    type HitParams = TKernel::HitParams;
    type Point = TKernel::Point;
    type Color = TKernel::Color;

    fn entry(&self, entry_params: &EntryParams<TKernel::Point>) -> Self::PayloadType {
        Payload {
            from: entry_params.from.clone(),
            to: entry_params.to.clone(),
            value: Self::Color::zero(),
            current_depth: 0,
            current_sampling: 0,
            latest_hit_material_id: None,
            latest_hit_normal: self.kernel.new_point(0.0, 0.0, 0.0),
            latest_hit_position: self.kernel.new_point(0.0, 0.0, 0.0),
            kernel: self.kernel.clone(),
            next_reflection_context: self.kernel.new_reflection_estimation_context(),
            hit_history: Vec::default(),
            plugin_payload: self.plugin.entry(entry_params),
        }
    }

    fn react_closest_hit(
        &self,
        mut payload: Self::PayloadType,
        hit_params: &Self::HitParams,
        func: impl Fn(&Self::Point, &Self::Point) -> Option<Self::HitParams>, // 現状は未使用だがプラグインに渡す予定
    ) -> Self::PayloadType {
        let sampling_data = self.plugin.react_closest_hit(
            payload.current_depth,
            &payload.plugin_payload,
            hit_params,
            func,
        );
        payload.hit_history.push(sampling_data);

        // 反射回数である深度を増やしつつヒット情報を保持してレイの生成に進む
        let normal = hit_params.normal();
        let position = hit_params.position();
        let new_depth = payload.current_depth + 1;

        payload
            .with_latest_hit_material_id(Some(hit_params.id()))
            .with_current_depth(new_depth)
            .with_latest_hit_position(position)
            .with_latest_hit_normal(normal)
    }

    fn react_hit_miss(&self, mut payload: Self::PayloadType) -> Self::PayloadType {
        // プラグイン呼び出し
        let sampling_data = self.plugin.react_hit_miss(&payload.plugin_payload);
        payload.hit_history.push(sampling_data);

        // ミスしたらトレースを完了させたいので反射回数を発散させる
        let next_depth = u32::MAX;
        payload.with_current_depth(next_depth)
    }

    fn trace(
        &self,
        ray_params: RayParams<Self::PayloadType, Self::Point>,
    ) -> crate::TraceAction<Self::PayloadType, Self::Point> {
        let mut payload = ray_params.payload;

        // 反射回数が規定回数を超えていたら...
        if self.depth < payload.current_depth {
            // 指定の回数のサンプリングが完了していたら終了
            if self.sampling_count <= payload.current_sampling {
                return crate::TraceAction::Finish(payload);
            }

            // 今回のサンプリングの結果を保持
            let mut color = Self::Color::zero();
            while let Some(sampling_data) = payload.hit_history.pop() {
                let emission = sampling_data.emission;
                let albedo = sampling_data.albedo;
                color = albedo.multiply(&color);
                color = color + emission;
            }

            // 前回のサンプリング結果との平均をとっていく
            let current_color = color / self.sampling_count as f32;
            let new_color = current_color + payload.value.clone();

            // 今回のサンプリングで保持していた情報を削除して、
            // 開始点に巻き戻してレイのトレースを続ける
            let new_sampling_count = payload.current_sampling + 1;
            return crate::TraceAction::Next(RayParams {
                from: payload.from.clone(),
                to: payload.to.clone(),
                payload: payload
                    .with_value(new_color)
                    .with_current_depth(0)
                    .with_current_sampling(new_sampling_count),
            });
        }

        let material_id = payload.latest_hit_material_id.unwrap();
        let new_direction = self.kernel.estimate_next_reflection(
            material_id,
            &mut payload.next_reflection_context,
            &payload.from,
            &payload.to,
        );

        let from = payload.latest_hit_position.clone() + new_direction.clone() * 0.001;
        let to = new_direction * 500.0 + from.clone();
        let ray_params = RayParams { from, to, payload };
        crate::TraceAction::Next(ray_params)
    }

    fn write(&self, payload: Self::PayloadType) -> Self::Color {
        payload.value
    }
}

pub struct SimplePlugin<T> {
    _marker: std::marker::PhantomData<T>,
}

impl<TKernel> SimplePlugin<TKernel> {
    pub fn new() -> Self {
        Self {
            _marker: std::marker::PhantomData,
        }
    }
}

impl<TKernel> IPathTracerPlugin for SimplePlugin<TKernel>
where
    TKernel: IKernel + Clone,
{
    type MaterialId = TKernel::MaterialId;
    type Point = TKernel::Point;
    type Color = TKernel::Color;
    type HitParams = TKernel::HitParams;
    type Payload = ();

    fn entry(&self, _entry_params: &EntryParams<Self::Point>) -> Self::Payload {
        ()
    }

    fn react_closest_hit(
        &self,
        _depth: u32,
        _payload: &Self::Payload,
        hit_params: &Self::HitParams,
        _func: impl Fn(&Self::Point, &Self::Point) -> Option<Self::HitParams>,
    ) -> SamplingData<Self::Color> {
        // ヒットした点の情報をシンプルに返す
        SamplingData {
            emission: hit_params.emission(),
            albedo: hit_params.albedo(),
        }
    }

    fn react_hit_miss(&self, _payload: &Self::Payload) -> SamplingData<Self::Color> {
        // どこにもヒットしなかったので背景色を返す
        SamplingData {
            emission: Self::Color::zero(),
            albedo: Self::Color::zero(),
        }
    }
}
