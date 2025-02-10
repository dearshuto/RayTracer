use crate::{EntryParams, HitAction, IRayTracingPipeline, RayParams, Vector3f};

pub trait IHitParams {
    fn normal(&self) -> Vector3f;

    fn position(&self) -> Vector3f;

    fn emission(&self) -> Vector3f;

    fn albedo(&self) -> Vector3f;
}

pub trait IRandomEngine {
    fn generate(&mut self) -> f32;
}

pub trait IKernel {
    type RondomEngine: IRandomEngine;

    fn random_engine(&self) -> Self::RondomEngine;
}

#[derive(sjrt_macro::Immutable)]
pub struct Payload<T>
where
    T: IRandomEngine,
{
    current_depth: u32,
    current_sampling: u32,
    values: Vec<(Vector3f /*albedo*/, Vector3f /*emission*/)>,
    random_engine: T,

    latest_hit_position: Vector3f,
    latest_hit_normal: Vector3f,
}

pub struct PathTracerEx<T, TKernel>
where
    TKernel: IKernel,
{
    depth: u32,
    kernel: TKernel,
    _marker: std::marker::PhantomData<T>,
}

impl<T: IHitParams, TKernel> IRayTracingPipeline for PathTracerEx<T, TKernel>
where
    TKernel: IKernel,
{
    type PayloadType = Payload<TKernel::RondomEngine>;
    type HitParams = T;

    fn entry(&self, _entry_params: &EntryParams) -> Self::PayloadType {
        Payload {
            current_depth: 0,
            current_sampling: 0,
            values: Vec::default(),
            random_engine: self.kernel.random_engine(),
            latest_hit_normal: Vector3f::zero(),
            latest_hit_position: Vector3f::zero(),
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
        let mut values = payload.values.clone();
        values.push((albedo, emission));

        HitAction::Payload(
            payload
                .with_current_depth(new_depth)
                .with_latest_hit_position(position)
                .with_latest_hit_normal(normal)
                .with_values(values),
        )
    }

    fn react_hit_miss(&self, payload: Self::PayloadType) -> Self::PayloadType {
        // ミスしたらトレースを完了させたいので反射回数を発散させる
        let next_depth = u32::MAX;

        // どこにもヒットしなかったら背景色を返す
        if payload.current_depth == 0 {
            let mut new_values = payload.values.clone();
            new_values.push((Vector3f::zero(), Vector3f::new(0.1, 0.2, 0.3)));

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
        let mut payload = ray_params.payload;

        // 反射回数が規定回数を超えていたら終了
        if self.depth < payload.current_depth {
            return crate::TraceAction::Finish(payload);
        }

        // 最初にヒットしたポイントの情報から次にレイを飛ばす方向を決める
        // とりあえず適当に乱数を生成して法線の向きに飛ばす
        let normal = payload.latest_hit_normal;
        let ratio_x = payload.random_engine.generate();
        let ratio_y = payload.random_engine.generate();
        let ratio_z = payload.random_engine.generate();
        let new_to = 500.0
            * Vector3f::new(normal.x * ratio_x, normal.y * ratio_y, normal.z * ratio_z).normalize();

        let ray_params = RayParams {
            from: payload.latest_hit_position,
            to: new_to,
            payload,
        };
        crate::TraceAction::Next(ray_params)
    }
}
