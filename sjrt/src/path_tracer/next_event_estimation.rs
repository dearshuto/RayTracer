use crate::{
    IInnerProduct, ISceneStructure,
    traits::{IComponentMul, INormalized},
};

use super::{
    DefaultKernel, IHitParams, IKernel, SamplingData,
    path_tracer_ex::{ClosestHitParams, IPathTracerPlugin},
};

use num::Zero;

#[derive(sjrt_macro::Immutable)]
pub struct Payload<TPoint, TColor> {
    light: Vec<TPoint>,
    samplings: Vec<SamplingData<TColor>>,
}

pub struct NextEventEstimation<TKernel> {
    kernel: TKernel,
    _marker: std::marker::PhantomData<TKernel>,
}

impl Default for NextEventEstimation<DefaultKernel> {
    fn default() -> Self {
        let kernel = DefaultKernel {};
        Self {
            kernel,
            _marker: std::marker::PhantomData,
        }
    }
}

impl<TKernel> IPathTracerPlugin for NextEventEstimation<TKernel>
where
    TKernel: IKernel,
{
    type MaterialId = TKernel::MaterialId;
    type Point = TKernel::Point;
    type Color = TKernel::Color;
    type HitParams = TKernel::HitParams;
    type Payload = Payload<TKernel::Point, TKernel::Color>;

    fn entry(&self, _entry_params: &crate::EntryParams<Self::Point>) -> Self::Payload {
        Payload {
            // sjrt::scene::Scene::box_point_light() で生成できるシーンで、
            // 光源が置いてある位置を決め打ちしている
            // 実際はシーン内の光源の位置を利用したいし、サンプリングは光源のジオメトリ情報を利用する
            light: vec![self.kernel.new_point(0.0, 9.0, 0.0)],

            samplings: Vec::default(),
        }
    }

    fn reset_payload(&self, payload: Self::Payload) -> Self::Payload {
        payload
    }

    fn react_closest_hit<TSceneStructure>(
        &self,
        closest_hit_params: ClosestHitParams<
            Self::Payload,
            Self::HitParams,
            TSceneStructure,
            Self::Point,
        >,
    ) -> Self::Payload
    where
        TSceneStructure: ISceneStructure<Self::HitParams, Self::Point>,
    {
        let depth = closest_hit_params.depth;
        let mut payload = closest_hit_params.payload;
        let hit_params = closest_hit_params.hit_params;
        let scene_structure = closest_hit_params.scene_structure;

        // レイが光源に当たった場合は特殊処理
        if !hit_params.emission().is_zero() {
            // いきなり光源に当たった場合は光源の情報を返し、反射の過程で光源に当たった場合はなにも寄与がないものとする
            // これは光源の寄与はひとつ前のヒットの NEE で評価済みなためで、
            // 初手以外の光源へのヒットで寄与を計算してしまうと寄与がダブってしまう
            if depth == 0 {
                return payload.update_samplings(|mut vec| {
                    vec.push(SamplingData {
                        emission: hit_params.emission(),
                        albedo: hit_params.albedo(),
                    });
                    vec
                });
            } else {
                return payload.update_samplings(|mut vec| {
                    vec.push(SamplingData {
                        emission: Self::Color::zero(),
                        albedo: Self::Color::zero(),
                    });
                    vec
                });
            }
        }

        //  サンプリングできる光源がなければなにもしない
        if payload.light.is_empty() {
            return payload;
        }

        // 光源の位置をサンプリング
        let index = rand::random_range(0..payload.light.len());
        let light_position = &payload.light[index];

        // ヒットした点の情報
        let position = hit_params.position();
        let light_direction = (light_position.clone() - position.clone()).normalized();

        // ヒットした点から光源までレイを生成

        let offset = light_direction.normalized() * 0.001;
        let Some(shadow_ray_hit_params) =
            scene_structure.cast(&(position + offset), light_position)
        else {
            return payload;
        };

        // コサイン項
        let cos_ratio = hit_params.normal().dot(&light_direction);

        let sampling_data = SamplingData {
            emission: shadow_ray_hit_params.emission() * cos_ratio,
            albedo: hit_params.albedo(),
        };
        payload.samplings.push(sampling_data);
        payload
    }

    fn react_hit_miss(&self, payload: Self::Payload) -> Self::Payload {
        // どこにもヒットしなかったので背景色をサンプリング
        payload
    }

    fn write(&self, payload: &mut Self::Payload) -> Self::Color {
        let mut color = Self::Color::zero();
        while let Some(sampling_data) = payload.samplings.pop() {
            let emission = sampling_data.emission;
            let albedo = sampling_data.albedo;
            color = albedo.multiply(&color);
            color = color + emission;
        }
        color
    }
}
