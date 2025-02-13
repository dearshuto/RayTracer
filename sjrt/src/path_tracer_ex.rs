use crate::{
    traits::IRandomEngine, util::HitParams, EntryParams, HitAction, IRayTracingPipeline, RayParams,
};

pub trait IHitParams {
    fn normal(&self) -> nalgebra::Vector3<f32>;

    fn position(&self) -> nalgebra::Vector3<f32>;

    fn emission(&self) -> nalgebra::Vector3<f32>;

    fn albedo(&self) -> nalgebra::Vector3<f32>;
}

pub trait IKernel {
    type RondomEngine: IRandomEngine<f32>;

    fn random_engine(&self) -> Self::RondomEngine;
}

#[derive(Clone)]
pub struct DefaultKernel;
impl IKernel for DefaultKernel {
    type RondomEngine = crate::util::RandomEngine;

    fn random_engine(&self) -> Self::RondomEngine {
        crate::util::RandomEngine::new()
    }
}

#[derive(sjrt_macro::Immutable)]
pub struct Payload<T>
where
    T: IKernel,
{
    current_depth: u32,
    current_sampling: u32,

    latest_hit_position: nalgebra::Vector3<f32>,
    latest_hit_normal: nalgebra::Vector3<f32>,

    kernel: T,

    // (emission, albedo)
    hit_history: Vec<(nalgebra::Vector3<f32>, nalgebra::Vector3<f32>)>,
}

pub struct PathTracerEx<T, TKernel>
where
    TKernel: IKernel,
{
    depth: u32,
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
    THitParams: IHitParams,
    TKernel: IKernel,
{
    pub fn new(kernel: TKernel) -> Self {
        Self {
            depth: 0, // TODO
            kernel,
            _marker: std::marker::PhantomData,
        }
    }
}

impl<T: IHitParams, TKernel> IRayTracingPipeline for PathTracerEx<T, TKernel>
where
    TKernel: IKernel + Clone,
{
    type PayloadType = Payload<TKernel>;
    type HitParams = T;

    fn entry(&self, _entry_params: &EntryParams) -> Self::PayloadType {
        Payload {
            current_depth: 0,
            current_sampling: 0,
            latest_hit_normal: nalgebra::Vector3::zeros(),
            latest_hit_position: nalgebra::Vector3::zeros(),
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
        impl Iterator<Item = crate::RayParams<Self::PayloadType>>,
    > {
        if false {
            return HitAction::RayGenerate([].into_iter());
        }

        // 反射回数である深度を増やしつつヒット情報を保持してレイの生成に進む
        let normal = hit_params.normal();
        let position = hit_params.position();
        let new_depth = payload.current_depth + 1;

        let albedo = hit_params.albedo();
        let emission = hit_params.emission();

        let mut new_payload = payload
            .with_current_depth(new_depth)
            .with_latest_hit_position(position)
            .with_latest_hit_normal(normal);

        new_payload
            .hit_history
            .push((hit_params.emission(), hit_params.albedo()));

        HitAction::Payload(new_payload)
    }

    fn react_hit_miss(&self, payload: Self::PayloadType) -> Self::PayloadType {
        // ミスしたらトレースを完了させたいので反射回数を発散させる
        let next_depth = u32::MAX;

        // どこにもヒットしなかったら背景色を返す
        if payload.current_depth == 0 {
            let mut new_values = payload.values.clone();
            new_values.push((
                nalgebra::Vector3::zeros(),
                nalgebra::Vector3::new(0.1, 0.2, 0.3),
            ));

            return payload
                .with_current_depth(next_depth)
                .with_values(new_values);
        }

        // 何回か反射してからミスしたら色は更新しない
        payload.with_current_depth(next_depth)
    }

    fn trace(
        &self,
        ray_params: RayParams<Self::PayloadType>,
    ) -> crate::TraceAction<Self::PayloadType> {
        let payload = ray_params.payload;

        // 反射回数が規定回数を超えていたら終了
        if self.depth < payload.current_depth {
            return crate::TraceAction::Finish(payload);
        }

        // 最初にヒットしたポイントの情報から次にレイを飛ばす方向を決める
        // とりあえず適当に乱数を生成して法線の向きに飛ばす
        let normal = payload.latest_hit_normal;
        let mut random_engine = payload.kernel.random_engine();
        let ratio_x = random_engine.generate_range(0.0..1.0);
        let ratio_y = random_engine.generate_range(0.0..1.0);
        let ratio_z = random_engine.generate_range(0.0..1.0);
        let new_to = 500.0
            * nalgebra::Vector3::new(normal.x * ratio_x, normal.y * ratio_y, normal.z * ratio_z)
                .normalize();

        let ray_params = RayParams {
            from: payload.latest_hit_position,
            to: new_to,
            payload,
        };
        crate::TraceAction::Next(ray_params)
    }

    fn write(&self, payload: Self::PayloadType) -> crate::executor::Color {
        let normal = payload.latest_hit_normal;
        crate::executor::Color::R32G32B32A32_Unorm([normal.x, normal.y, normal.z, 1.0])
    }
}
