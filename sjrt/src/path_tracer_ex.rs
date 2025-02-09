use crate::{HitAction, IRayTracingPipeline, RayParams, Vector3f};

pub trait IHitParams {
    fn normal(&self) -> Vector3f;

    fn position(&self) -> Vector3f;
}

#[derive(sjrt_macro::Immutable)]
pub struct Payload {
    current_depth: u32,
    current_sampling: u32,
    value: [u8; 4],
}

pub struct PathTracerEx<T> {
    depth: u32,
    _marker: std::marker::PhantomData<T>,
}

impl<T: IHitParams> IRayTracingPipeline for PathTracerEx<T> {
    type PayloadType = Payload;

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
