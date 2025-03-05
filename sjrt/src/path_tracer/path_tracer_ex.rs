use std::ops::{Add, Div, Mul};

use crate::{
    EntryParams, IConstract, IInnerProduct, IRayTracingPipeline, RayParams,
    traits::{IComponentMul, INormalized, IRandomEngine},
};

use num::Zero;

use super::DefaultKernel;

pub trait IPathTracerPlugin {
    type Point;
    type Payload;

    fn entry(&self, entry_params: &EntryParams<Self::Point>) -> Self::Payload;
}

pub trait IHitParams<TId, TPoint, TColor> {
    fn id(&self) -> TId;

    fn normal(&self) -> TPoint;

    fn position(&self) -> TPoint;

    fn emission(&self) -> TColor;

    fn albedo(&self) -> TColor;
}

pub trait IKernel {
    type Plugin: IPathTracerPlugin<Point = Self::Point>;
    type MaterialId: Copy;
    type ReflectionEstimationContext;
    type RondomEngine: IRandomEngine<f32>;
    type Point: Clone
        + IConstract<f32>
        + INormalized
        + IInnerProduct<f32>
        + Mul<f32, Output = Self::Point>
        + Add<Self::Point, Output = Self::Point>;
    type Color: Clone
        + Into<crate::Color>
        + num::Zero
        + Add<Self::Color, Output = Self::Color>
        + Div<f32, Output = Self::Color>
        + IComponentMul;
    type HitParams: IHitParams<Self::MaterialId, Self::Point, Self::Color>;

    fn new_plugin(&self) -> Self::Plugin;

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
pub struct Payload<T>
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
    hit_history: Vec<(T::Color, T::Color)>,

    plugin_payload: <<T as IKernel>::Plugin as IPathTracerPlugin>::Payload,
}

pub struct PathTracerEx<TKernel>
where
    TKernel: IKernel,
{
    depth: u32,
    sampling_count: u32,
    kernel: TKernel,
    plugin: TKernel::Plugin,
}

impl Default for PathTracerEx<DefaultKernel> {
    fn default() -> Self {
        let kernel = DefaultKernel {};
        Self::new(kernel)
    }
}

impl<TKernel> PathTracerEx<TKernel>
where
    TKernel: IKernel,
{
    pub fn new(kernel: TKernel) -> Self {
        Self {
            depth: 1,
            sampling_count: 1,
            plugin: kernel.new_plugin(),
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

impl<TKernel> IRayTracingPipeline for PathTracerEx<TKernel>
where
    TKernel: IKernel + Clone,
{
    type PayloadType = Payload<TKernel>;
    type HitParams = TKernel::HitParams;
    type Point = TKernel::Point;
    type Color = TKernel::Color;

    fn entry(&self, entry_params: &EntryParams<TKernel::Point>) -> Self::PayloadType {
        Payload::<TKernel> {
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
        payload: Self::PayloadType,
        hit_params: &Self::HitParams,
        _func: impl Fn(&Self::Point, &Self::Point) -> Option<Self::HitParams>, // 現状は未使用だがプラグインに渡す予定
    ) -> Self::PayloadType {
        // 反射回数である深度を増やしつつヒット情報を保持してレイの生成に進む
        let normal = hit_params.normal();
        let position = hit_params.position();
        let new_depth = payload.current_depth + 1;

        let mut new_payload = payload
            .with_latest_hit_material_id(Some(hit_params.id()))
            .with_current_depth(new_depth)
            .with_latest_hit_position(position)
            .with_latest_hit_normal(normal);

        // ヒットした点の情報を履歴として保持
        new_payload
            .hit_history
            .push((hit_params.emission(), hit_params.albedo()));

        new_payload
    }

    fn react_hit_miss(&self, payload: Self::PayloadType) -> Self::PayloadType {
        // ミスしたらトレースを完了させたいので反射回数を発散させる
        let next_depth = u32::MAX;

        // どこにもヒットしなかったので背景色を返す
        let mut new_payload = payload.with_current_depth(next_depth);

        new_payload
            .hit_history
            .push((Self::Color::zero(), Self::Color::zero()));

        new_payload
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
            while let Some((emission, albedo)) = payload.hit_history.pop() {
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
