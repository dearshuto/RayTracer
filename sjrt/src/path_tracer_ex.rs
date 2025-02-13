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
    // 最初にレイを飛ばしたときの始点と終点
    from: nalgebra::Vector3<f32>,
    to: nalgebra::Vector3<f32>,

    // 蓄積した色
    value: nalgebra::Vector3<f32>,

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
    THitParams: IHitParams,
    TKernel: IKernel,
{
    pub fn new(kernel: TKernel) -> Self {
        Self {
            depth: 8,            // TODO
            sampling_count: 256, // TODO
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

    fn entry(&self, entry_params: &EntryParams) -> Self::PayloadType {
        Payload {
            from: entry_params.from,
            to: entry_params.to,
            value: nalgebra::Vector3::zeros(),
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

        new_payload.hit_history.push((
            nalgebra::Vector3::new(0.0, 0.0, 0.0),
            nalgebra::Vector3::zeros(),
        ));

        new_payload
    }

    fn trace(
        &self,
        ray_params: RayParams<Self::PayloadType>,
    ) -> crate::TraceAction<Self::PayloadType> {
        let mut payload = ray_params.payload;

        // 反射回数が規定回数を超えていたら...
        if self.depth < payload.current_depth {
            // 指定の回数のサンプリングが完了していたら終了
            if self.sampling_count <= payload.current_sampling {
                return crate::TraceAction::Finish(payload);
            }

            // 今回のサンプリングの結果を保持
            let mut color = nalgebra::Vector3::zeros();
            while let Some((emission, albedo)) = payload.hit_history.pop() {
                color = albedo.component_mul(&color);
                color += emission;
            }

            // 前回のサンプリング結果との平均をとっていく
            let current_color = color / self.sampling_count as f32;
            let new_color = payload.value + current_color;

            // 今回のサンプリングで保持していた情報を削除して、
            // 開始点に巻き戻してレイのトレースを続ける
            let new_sampling_count = payload.current_sampling + 1;
            return crate::TraceAction::Next(RayParams {
                from: payload.from,
                to: payload.to,
                payload: payload
                    .with_value(new_color)
                    .with_current_depth(0)
                    .with_current_sampling(new_sampling_count),
            });
        }

        // 最初にヒットしたポイントの情報から次にレイを飛ばす方向を決める
        // とりあえず適当に乱数を生成して法線の向きに飛ばす
        let normal = payload.latest_hit_normal;
        let mut random_engine = payload.kernel.random_engine();
        let new_direction = loop {
            let ratio_x = random_engine.generate_range(-1.0..1.0);
            let ratio_y = random_engine.generate_range(-1.0..1.0);
            let ratio_z = random_engine.generate_range(-1.0..1.0);
            let new_normal = nalgebra::Vector3::new(ratio_x, ratio_y, ratio_z).normalize();
            if new_normal.dot(&normal) <= 0.0 {
                continue;
            }

            break new_normal;
        };

        let from = payload.latest_hit_position + new_direction * 0.001;
        let to = 500.0 * new_direction + from;
        let ray_params = RayParams { from, to, payload };
        crate::TraceAction::Next(ray_params)
    }

    fn write(&self, payload: Self::PayloadType) -> crate::executor::Color {
        let color = payload.value;
        crate::executor::Color::R32G32B32A32_Unorm([color.x, color.y, color.z, 1.0])
    }
}
