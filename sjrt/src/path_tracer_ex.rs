use crate::{HitAction, IRayTracingPipeline, RayParams, Vector3f};

pub trait IHitParams {
    fn normal(&self) -> Vector3f;

    fn position(&self) -> Vector3f;
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
    value: [u8; 4],
    random_engine: T,
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
        [].into_iter()
    }

    fn trace(&self, payload: Self::PayloadType) -> crate::TraceAction<Self::PayloadType> {
        // 反射回数が規定回数を超えていたら終了
        if self.depth < payload.current_depth {
            return crate::TraceAction::Finish(payload);
        }

        let ray_params = RayParams {
            from: Vector3f::zero(),
            to: Vector3f::zero(),
        };
        crate::TraceAction::Next((ray_params, payload))
    }

    fn react_closest_hit(
        &self,
        payload: Self::PayloadType,
        hit_params: &Self::HitParams,
    ) -> crate::HitAction<Self::PayloadType, impl Iterator<Item = crate::RayParams>> {
        if false {
            return HitAction::RayGenerate([].into_iter());
        }

        let _normal = hit_params.normal();
        let _position = hit_params.position();

        // 反射回数をひとつ増やして再びレイの生成判定へ
        let new_depth = payload.current_depth + 1;
        return HitAction::Payload(payload.with_current_depth(new_depth));
    }

    fn react_hit_miss(&self, payload: Self::PayloadType) -> Self::PayloadType {
        // ミスしたらトレースを完了させたいので反射回数を発散させる
        let next_depth = u32::MAX;

        // どこにもヒットしなかったら背景色を返す
        if payload.current_depth == 0 {
            return payload
                .with_current_depth(next_depth)
                .with_value([25, 50, 75, u8::MAX]);
        }

        // 何回か反射してからミスしたら色は更新しない
        payload.with_current_depth(next_depth)
    }
}
