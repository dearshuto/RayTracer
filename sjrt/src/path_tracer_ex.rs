use crate::{HitAction, IRayTracingPipeline, RayParams, Vector3f};

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
    from: Vector3f,
    to: Option<Vector3f>,
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

    // TODO: rapier3d に限定しないよう抽象化する
    type HitParams = T;

    fn entry(&self) -> impl Iterator<Item = Self::PayloadType> {
        [Payload {
            current_depth: 0,
            current_sampling: 0,
            values: Vec::default(),
            random_engine: self.kernel.random_engine(),
            from: Vector3f::zero(),
            to: None,
        }]
        .into_iter()
    }

    fn trace(&self, payload: Self::PayloadType) -> crate::TraceAction<Self::PayloadType> {
        // 反射回数が規定回数を超えていたら終了
        if self.depth < payload.current_depth {
            return crate::TraceAction::Finish(payload);
        }

        let ray_params = RayParams {
            from: Vector3f::zero(),
            to: Vector3f::zero(),
            payload,
        };
        crate::TraceAction::Next(ray_params)
    }

    fn react_closest_hit(
        &self,
        mut payload: Self::PayloadType,
        hit_params: &Self::HitParams,
    ) -> crate::HitAction<
        Self::PayloadType,
        impl Iterator<Item = crate::RayParams<Self::PayloadType>>,
    > {
        if false {
            return HitAction::RayGenerate([].into_iter());
        }

        let ratio_x = payload.random_engine.generate();
        let ratio_y = payload.random_engine.generate();
        let ratio_z = payload.random_engine.generate();
        let normal = hit_params.normal();
        let new_to = 500.0
            * Vector3f::new(normal.x * ratio_x, normal.y * ratio_y, normal.z * ratio_z).normalize();

        let position = hit_params.position();

        // 反射回数をひとつ増やして再びレイの生成判定へ
        let new_depth = payload.current_depth + 1;
        return HitAction::Payload(
            payload
                .with_current_depth(new_depth)
                .with_from(position)
                .with_to(Some(new_to)),
        );
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
}
