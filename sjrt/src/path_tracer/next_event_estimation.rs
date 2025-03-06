use crate::{IInnerProduct, traits::INormalized};

use super::{
    DefaultKernel, IHitParams, IKernel,
    path_tracer_ex::{IPathTracerPlugin, SamplingData},
};

use num::Zero;

pub struct Payload<TKernel: IKernel> {
    light: Vec<TKernel::Point>,
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
    type Payload = Payload<TKernel>;

    fn entry(&self, _entry_params: &crate::EntryParams<Self::Point>) -> Self::Payload {
        Payload {
            // sjrt::scene::Scene::box_point_light() で生成できるシーンで、
            // 光源が置いてある位置を決め打ちしている
            // 実際はシーン内の光源の位置を利用したいし、サンプリングは光源のジオメトリ情報を利用する
            light: vec![self.kernel.new_point(0.0, 9.0, 0.0)],
        }
    }

    fn react_closest_hit(
        &self,
        depth: u32,
        payload: &Self::Payload,
        hit_params: &Self::HitParams,
        func: impl Fn(&Self::Point, &Self::Point) -> Option<Self::HitParams>,
    ) -> super::path_tracer_ex::SamplingData<Self::Color> {
        // レイが光源に当たった場合は特殊処理
        if !hit_params.emission().is_zero() {
            // いきなり光源に当たった場合は光源の情報を返し、反射の過程で光源に当たった場合はなにも寄与がないものとする
            // これは光源の寄与はひとつ前のヒットの NEE で評価済みなためで、
            // 初手以外の光源へのヒットで寄与を計算してしまうと寄与がダブってしまう
            if depth == 0 {
                return SamplingData {
                    emission: hit_params.emission(),
                    albedo: hit_params.albedo(),
                };
            } else {
                return SamplingData {
                    emission: Self::Color::zero(),
                    albedo: Self::Color::zero(),
                };
            }
        }

        //  サンプリングできる光源がなければなにもしない
        if payload.light.is_empty() {
            return SamplingData {
                emission: Self::Color::zero(),
                albedo: Self::Color::zero(),
            };
        }

        // 光源の位置をサンプリング
        let index = rand::random_range(0..payload.light.len());
        let light_position = &payload.light[index];

        // ヒットした点の情報
        let position = hit_params.position();
        let light_direction = (light_position.clone() - position.clone()).normalized();

        // ヒットした点から光源までレイを生成

        let offset = light_direction.normalized() * 0.001;
        let Some(shadow_ray_hit_params) = func(&(position + offset), light_position) else {
            return SamplingData {
                emission: Self::Color::zero(),
                albedo: Self::Color::zero(),
            };
        };

        // コサイン項
        let cos_ratio = hit_params.normal().dot(&light_direction);

        SamplingData {
            emission: shadow_ray_hit_params.emission() * cos_ratio,
            albedo: hit_params.albedo(),
        }
    }

    fn react_hit_miss(
        &self,
        _payload: &Self::Payload,
    ) -> super::path_tracer_ex::SamplingData<Self::Color> {
        // どこにもヒットしなかったので背景色を返す
        SamplingData {
            emission: Self::Color::zero(),
            albedo: Self::Color::zero(),
        }
    }
}
