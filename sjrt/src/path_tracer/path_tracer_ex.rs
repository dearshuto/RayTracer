use std::ops::{Add, Div, Mul, Sub};

use crate::{
    EntryParams, IConstract, IInnerProduct, IRayTracingPipeline, ISceneStructure, RayParams,
    traits::{IComponentMul, INormalized},
};

use num::Zero;

use super::{DefaultKernel, SamplingData};

pub struct ClosestHitParams<TPayload, THitParams, TSceneStructure, TVector>
where
    TSceneStructure: ISceneStructure<THitParams, TVector>,
{
    pub depth: u32,
    pub in_direction: TVector,
    pub payload: TPayload,
    pub hit_params: THitParams,
    pub scene_structure: TSceneStructure,
}

pub trait IPathTracerPlugin {
    type MaterialId;
    type Point;
    type Color;
    type HitParams: IHitParams<Self::MaterialId, Self::Point, Self::Color>;
    type Payload;

    fn entry(&self, entry_params: &EntryParams<Self::Point>) -> Self::Payload;

    fn reset_payload(&self, payload: Self::Payload) -> Self::Payload;

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
        TSceneStructure: ISceneStructure<Self::HitParams, Self::Point>;

    fn react_hit_miss(&self, payload: Self::Payload) -> Self::Payload;

    fn write(&self, payload: &mut Self::Payload) -> Self::Color;
}

pub trait IHitParams<TId, TPoint, TColor> {
    fn id(&self) -> TId;

    fn normal(&self) -> TPoint;

    fn position(&self) -> TPoint;

    fn emission(&self) -> TColor;

    fn albedo(&self) -> TColor;
}

pub trait IKernel {
    type MaterialId: Copy;
    type ReflectionEstimationContext;
    type Point: Clone
        + IConstract<f32>
        + INormalized
        + IInnerProduct<f32>
        + Mul<f32, Output = Self::Point>
        + Add<Self::Point, Output = Self::Point>
        // Point に引き算を定義しているがあまり直感的ではない気がする
        // 例えば Vector を関連型に追加して、IKernel に 2 つの Point を渡したら、
        // Vector を生成するようなインターフェースにしてもいいかも
        + Sub<Self::Point, Output = Self::Point>;
    type Color: Clone
        + Into<crate::Color>
        + num::Zero
        + Add<Self::Color, Output = Self::Color>
        + Mul<f32, Output = Self::Color>
        + Div<f32, Output = Self::Color>
        + IComponentMul;
    type HitParams: IHitParams<Self::MaterialId, Self::Point, Self::Color>;

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
pub struct Payload<T, U>
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

    plugin_payload: U,
}

pub struct PathTracerEx<TKernel, TPlugin>
where
    TKernel: IKernel,
    TPlugin: IPathTracerPlugin<Point = TKernel::Point, Color = TKernel::Color>,
{
    depth: u32,
    sampling_count: u32,
    kernel: TKernel,
    plugin: TPlugin,
}

impl Default for PathTracerEx<DefaultKernel, SimplePlugin<DefaultKernel>> {
    fn default() -> Self {
        let kernel = DefaultKernel {};
        let plugin = SimplePlugin::new();
        Self::new(kernel, plugin)
    }
}

impl<TPlugin> PathTracerEx<DefaultKernel, TPlugin>
where
    TPlugin: IPathTracerPlugin<
            MaterialId = <DefaultKernel as IKernel>::MaterialId,
            Point = <DefaultKernel as IKernel>::Point,
            Color = <DefaultKernel as IKernel>::Color,
            HitParams = <DefaultKernel as IKernel>::HitParams,
        >,
{
    pub fn default_with(plugin: TPlugin) -> Self {
        let kernel = DefaultKernel {};
        Self::new(kernel, plugin)
    }
}

impl<TKernel, TPlugin> PathTracerEx<TKernel, TPlugin>
where
    TKernel: IKernel,
    TPlugin: IPathTracerPlugin<Point = TKernel::Point, Color = TKernel::Color>,
{
    pub fn new(kernel: TKernel, plugin: TPlugin) -> Self {
        Self {
            depth: 1,
            sampling_count: 1,
            plugin,
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

impl<TKernel, TPlugin> IRayTracingPipeline for PathTracerEx<TKernel, TPlugin>
where
    TKernel: IKernel + Clone,
    TPlugin: IPathTracerPlugin<
            Point = TKernel::Point,
            Color = TKernel::Color,
            HitParams = TKernel::HitParams,
        >,
{
    type PayloadType = Payload<TKernel, TPlugin::Payload>;
    type HitParams = TKernel::HitParams;
    type Point = TKernel::Point;
    type Color = TKernel::Color;

    fn entry(&self, entry_params: &EntryParams<TKernel::Point>) -> Self::PayloadType {
        Payload {
            from: entry_params.from.clone(),
            to: entry_params.to.clone(),
            value: Self::Color::zero(),
            current_depth: 0,
            current_sampling: 0,
            latest_hit_material_id: None,
            latest_hit_normal: self.kernel.new_point(0.0, 0.0, 0.0),
            latest_hit_position: self.kernel.new_point(0.0, 0.0, 0.0),
            next_reflection_context: self.kernel.new_reflection_estimation_context(),
            plugin_payload: self.plugin.entry(entry_params),
        }
    }

    fn react_closest_hit<TSceneStructure>(
        &self,
        payload: Self::PayloadType,
        hit_params: Self::HitParams,
        scene_structure: TSceneStructure,
    ) -> Self::PayloadType
    where
        TSceneStructure: ISceneStructure<Self::HitParams, Self::Point>,
    {
        let depth = payload.current_depth;

        // 反射回数である深度を増やしつつヒット情報を保持してレイの生成に進む
        payload
            .with_current_depth(depth + 1)
            .with_latest_hit_material_id(Some(hit_params.id()))
            .with_latest_hit_position(hit_params.position())
            .with_latest_hit_normal(hit_params.normal())
            .update_plugin_payload(move |plugin_payload| {
                let in_direction = self.kernel.new_point(0.0, 0.0, 0.0);
                let closest_hit_params = ClosestHitParams {
                    depth,
                    in_direction,
                    payload: plugin_payload,
                    hit_params,
                    scene_structure,
                };

                self.plugin.react_closest_hit(closest_hit_params)
            })
    }

    fn react_hit_miss(&self, payload: Self::PayloadType) -> Self::PayloadType {
        // プラグイン呼び出し
        // スペキュラは適当な値
        let new_payload = payload.update_plugin_payload(|p| self.plugin.react_hit_miss(p));

        // ミスしたらトレースを完了させたいので反射回数を発散させる
        let next_depth = u32::MAX;
        new_payload.with_current_depth(next_depth)
    }

    fn trace(
        &self,
        ray_params: RayParams<Self::PayloadType, Self::Point>,
    ) -> crate::TraceAction<Self::PayloadType, Self::Point> {
        let mut payload = ray_params.payload;

        // 反射回数が規定回数を超えていたら...
        // or レイがどこにもヒットしなかったら...
        if self.depth < payload.current_depth || payload.latest_hit_material_id.is_none() {
            // 今回のサンプリングの結果を保持
            let color = self.plugin.write(&mut payload.plugin_payload);

            // 前回のサンプリング結果との平均をとっていく
            let current_color = color / self.sampling_count as f32;
            let new_color = current_color + payload.value.clone();

            let payload = payload
                .with_value(new_color)
                .update_plugin_payload(|p| self.plugin.reset_payload(p));

            // 今回のサンプリングで保持していた情報を削除して、
            // 開始点に巻き戻してレイのトレースを続ける
            let new_sampling_count = payload.current_sampling + 1;

            // 指定の回数のサンプリングが完了していたら終了
            if self.sampling_count < new_sampling_count {
                return crate::TraceAction::Finish(payload);
            }

            return crate::TraceAction::Next(RayParams {
                from: payload.from.clone(),
                to: payload.to.clone(),
                payload: payload
                    .with_current_depth(0)
                    .with_current_sampling(new_sampling_count),
            });
        }

        let material_id = payload.latest_hit_material_id.unwrap();
        let new_direction = self.kernel.estimate_next_reflection(
            material_id,
            &mut payload.next_reflection_context,
            &payload.from,
            &payload.latest_hit_normal,
        );

        let from = payload.latest_hit_position.clone() + new_direction.clone() * 0.001;
        let to = new_direction * 500.0 + from.clone();
        let ray_params = RayParams { from, to, payload };
        crate::TraceAction::Next(ray_params)
    }

    fn write(&self, mut payload: Self::PayloadType) -> Self::Color {
        self.plugin.write(&mut payload.plugin_payload)
    }
}

#[derive(sjrt_macro::Immutable)]
pub struct SimplePluginPayload<TColor> {
    hit_history: Vec<SamplingData<TColor>>,
}

impl<T> Default for SimplePluginPayload<T> {
    fn default() -> Self {
        Self {
            hit_history: Vec::default(),
        }
    }
}

pub struct SimplePlugin<T> {
    _marker: std::marker::PhantomData<T>,
}

impl<TKernel> SimplePlugin<TKernel> {
    pub fn new() -> Self {
        Self {
            _marker: std::marker::PhantomData,
        }
    }
}

impl<TKernel> IPathTracerPlugin for SimplePlugin<TKernel>
where
    TKernel: IKernel + Clone,
{
    type MaterialId = TKernel::MaterialId;
    type Point = TKernel::Point;
    type Color = TKernel::Color;
    type HitParams = TKernel::HitParams;
    type Payload = SimplePluginPayload<TKernel::Color>;

    fn entry(&self, _entry_params: &EntryParams<Self::Point>) -> Self::Payload {
        SimplePluginPayload::default()
    }

    fn reset_payload(&self, mut payload: Self::Payload) -> Self::Payload {
        payload.hit_history.clear();
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
        // ヒットした点の情報を保持
        let mut payload = closest_hit_params.payload;
        let hit_params = closest_hit_params.hit_params;
        payload.hit_history.push(SamplingData {
            emission: hit_params.emission(),
            albedo: hit_params.albedo(),
        });
        payload
    }

    fn react_hit_miss(&self, payload: Self::Payload) -> Self::Payload {
        // TODO: どこにもヒットしなかったので背景色を保持
        payload
    }

    fn write(&self, payload: &mut Self::Payload) -> Self::Color {
        let mut color = Self::Color::zero();
        while let Some(sampling_data) = payload.hit_history.pop() {
            let emission = sampling_data.emission;
            let albedo = sampling_data.albedo;
            color = albedo.multiply(&color);
            color = color + emission;
        }
        color
    }
}

#[cfg(test)]
mod tests {
    use std::sync::{Arc, Mutex};

    use crate::{EntryParams, IRayTracingPipeline, RayParams, util::HitParams};

    use super::{
        IKernel, IPathTracerPlugin, PathTracerEx, Payload, SimplePlugin, SimplePluginPayload,
    };

    struct Context {
        normal: nalgebra::Vector3<f32>,
    }

    #[derive(Clone)]
    struct MockKernel {
        context: Arc<Mutex<Context>>,
    }

    impl IKernel for MockKernel {
        type MaterialId = u32;
        type ReflectionEstimationContext = Arc<Mutex<Context>>;
        type Point = nalgebra::Vector3<f32>;
        type Color = nalgebra::Vector3<f32>;
        type HitParams = HitParams;

        fn new_point(&self, x: f32, y: f32, z: f32) -> Self::Point {
            nalgebra::Vector3::new(x, y, z)
        }

        fn new_reflection_estimation_context(&self) -> Self::ReflectionEstimationContext {
            self.context.clone()
        }

        fn estimate_next_reflection(
            &self,
            _id: Self::MaterialId,
            context: &mut Self::ReflectionEstimationContext,
            _in_direction: &Self::Point,
            normal: &Self::Point,
        ) -> Self::Point {
            // どんな法線が渡されたかを保持
            context.lock().unwrap().normal = *normal;

            // 適当な値を返す
            nalgebra::Vector3::x()
        }
    }

    /// レイがヒットした点の法線情報が次の反射ベクトルの計算に正しく反映されてるか検証するテスト
    #[test]
    fn test_pipe_normal() {
        let context = Arc::new(Mutex::new(Context {
            normal: Default::default(),
        }));
        let mock_kernel = MockKernel {
            context: context.clone(),
        };

        let pipeline = PathTracerEx::new(mock_kernel, SimplePlugin::<MockKernel>::new());

        let expected_normal = nalgebra::Vector3::y();
        let ray_params = RayParams {
            from: nalgebra::Vector3::zeros(),
            to: nalgebra::Vector3::x(),
            payload: Payload {
                from: nalgebra::Vector3::zeros(),
                to: nalgebra::Vector3::x(),
                value: Default::default(),
                current_depth: 0,
                current_sampling: 0,
                latest_hit_material_id: Some(0),
                latest_hit_position: nalgebra::Vector3::zeros(),
                latest_hit_normal: expected_normal, // これが反射ベクトルの計算に使用される
                next_reflection_context: context.clone(),
                plugin_payload: SimplePluginPayload::default(),
            },
        };
        let _ = pipeline.trace(ray_params);

        // 反射ベクトル計算に渡された法線を取得して、期待した法線が渡っているか確認
        let normal = context.lock().unwrap().normal;
        assert_eq!(normal, expected_normal);
    }

    #[derive(Default)]
    struct MockPluginPayload {
        current_sampling: u32,
        color: [nalgebra::Vector3<f32>; 2],
    }

    struct MockPlugin;
    impl IPathTracerPlugin for MockPlugin {
        type MaterialId = u32;
        type Point = nalgebra::Vector3<f32>;
        type Color = nalgebra::Vector3<f32>;
        type HitParams = HitParams;
        type Payload = MockPluginPayload;

        fn entry(&self, _entry_params: &EntryParams<Self::Point>) -> Self::Payload {
            MockPluginPayload {
                current_sampling: 0,
                color: [
                    nalgebra::Vector3::new(0.1, 0.2, 0.3),
                    nalgebra::Vector3::new(0.3, 0.4, 0.5),
                ],
            }
        }

        fn reset_payload(&self, payload: Self::Payload) -> Self::Payload {
            payload
        }

        fn react_closest_hit<TSceneStructure>(
            &self,
            closest_hit_params: super::ClosestHitParams<
                Self::Payload,
                Self::HitParams,
                TSceneStructure,
                Self::Point,
            >,
        ) -> Self::Payload
        where
            TSceneStructure: crate::ISceneStructure<Self::HitParams, Self::Point>,
        {
            closest_hit_params.payload
        }

        fn react_hit_miss(&self, payload: Self::Payload) -> Self::Payload {
            payload
        }

        fn write(&self, payload: &mut Self::Payload) -> Self::Color {
            // 1 回目のサンプリングでは 0 番目を、
            // それ以降のサンプリングでは 1 番目を返す
            let index = payload.current_sampling;
            payload.current_sampling = (payload.current_sampling + 1).min(1);
            payload.color[index as usize]
        }
    }

    #[test]
    fn test_recursive() {
        let pipeline = PathTracerEx::default_with(MockPlugin {})
            .with_sampling_count(2)
            .with_depth(1);

        let payload = pipeline.entry(&EntryParams {
            x: 0,
            y: 0,
            from: nalgebra::Vector3::zeros(),
            to: nalgebra::Vector3::x(),
        });
        let payload = pipeline.react_hit_miss(payload);

        let mut ray_params = RayParams {
            from: nalgebra::Vector3::zeros(),
            to: nalgebra::Vector3::x(),
            payload,
        };
        let payload = loop {
            let action = pipeline.trace(ray_params);

            match action {
                crate::TraceAction::Next(next_ray_params) => ray_params = next_ray_params,
                crate::TraceAction::Finish(payload) => break payload,
            };
        };

        let color = payload.value;
        assert_eq!(color, nalgebra::Vector3::new(0.2, 0.3, 0.4));
    }
}
