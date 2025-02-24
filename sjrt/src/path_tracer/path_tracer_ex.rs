use std::ops::{Add, Div, Mul};

use crate::{
    EntryParams, HitAction, IConstract, IInnerProduct, IRayTracingPipeline, RayParams,
    traits::{IComponentMul, INormalized, IRandomEngine},
    util::HitParams,
};

use num::Zero;

use super::DefaultKernel;

pub trait IHitParams<TPoint, TColor> {
    fn normal(&self) -> TPoint;

    fn position(&self) -> TPoint;

    fn emission(&self) -> TColor;

    fn albedo(&self) -> TColor;
}

pub trait IKernel {
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

    fn random_engine(&self) -> Self::RondomEngine;

    fn new_point(&self, x: f32, y: f32, z: f32) -> Self::Point;
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

    latest_hit_position: T::Point,
    latest_hit_normal: T::Point,

    kernel: T,

    // (emission, albedo)
    hit_history: Vec<(T::Color, T::Color)>,
}

pub struct PathTracerEx<T, TKernel>
where
    TKernel: IKernel,
{
    depth: u32,
    sampling_count: u32,
    kernel: TKernel,
    _marker: std::marker::PhantomData<T>,
}

impl Default for PathTracerEx<HitParams, DefaultKernel> {
    fn default() -> Self {
        let kernel = DefaultKernel {};
        Self::new(kernel)
    }
}

impl<THitParams, TKernel> PathTracerEx<THitParams, TKernel>
where
    THitParams: IHitParams<TKernel::Point, TKernel::Color>,
    TKernel: IKernel,
{
    pub fn new(kernel: TKernel) -> Self {
        Self {
            depth: 1,
            sampling_count: 1,
            kernel,
            _marker: std::marker::PhantomData,
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

impl<T, TKernel> IRayTracingPipeline for PathTracerEx<T, TKernel>
where
    T: IHitParams<TKernel::Point, TKernel::Color>,
    TKernel: IKernel + Clone,
{
    type PayloadType = Payload<TKernel>;
    type HitParams = T;
    type Point = TKernel::Point;
    type Color = TKernel::Color;

    fn entry(&self, entry_params: &EntryParams<TKernel::Point>) -> Self::PayloadType {
        Payload {
            from: entry_params.from.clone(),
            to: entry_params.to.clone(),
            value: Self::Color::zero(),
            current_depth: 0,
            current_sampling: 0,
            latest_hit_normal: self.kernel.new_point(0.0, 0.0, 0.0),
            latest_hit_position: self.kernel.new_point(0.0, 0.0, 0.0),
            kernel: self.kernel.clone(),
            hit_history: Vec::default(),
        }
    }

    fn react_closest_hit(
        &self,
        payload: Self::PayloadType,
        hit_params: &Self::HitParams,
    ) -> crate::HitAction<
        Self::PayloadType,
        impl Iterator<Item = crate::RayParams<Self::PayloadType, Self::Point>>,
        Self::Point,
    > {
        if false {
            return HitAction::RayGenerate([].into_iter());
        }

        // 反射回数である深度を増やしつつヒット情報を保持してレイの生成に進む
        let normal = hit_params.normal();
        let position = hit_params.position();
        let new_depth = payload.current_depth + 1;

        let mut new_payload = payload
            .with_current_depth(new_depth)
            .with_latest_hit_position(position)
            .with_latest_hit_normal(normal);

        // ヒットした点の情報を履歴として保持
        new_payload
            .hit_history
            .push((hit_params.emission(), hit_params.albedo()));

        HitAction::Payload(new_payload)
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

        // 最初にヒットしたポイントの情報から次にレイを飛ばす方向を決める
        // とりあえず適当に乱数を生成して法線の向きに飛ばす
        let normal = payload.latest_hit_normal.clone();
        let mut random_engine = payload.kernel.random_engine();
        let new_direction = loop {
            let ratio_x = random_engine.generate_range(-1.0..1.0);
            let ratio_y = random_engine.generate_range(-1.0..1.0);
            let ratio_z = random_engine.generate_range(-1.0..1.0);
            let new_normal = self
                .kernel
                .new_point(ratio_x, ratio_y, ratio_z)
                .normalized();
            if new_normal.dot(&normal) <= 0.0 {
                continue;
            }

            break new_normal;
        };

        let from = payload.latest_hit_position.clone() + new_direction.clone() * 0.001;
        let to = new_direction * 500.0 + from.clone();
        let ray_params = RayParams { from, to, payload };
        crate::TraceAction::Next(ray_params)
    }

    fn write(&self, payload: Self::PayloadType) -> Self::Color {
        payload.value
    }
}
