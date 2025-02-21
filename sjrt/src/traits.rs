use std::ops::Range;

use crate::MaterialInfo;

pub trait IRenderer<T> {
    fn render<TScene: IScene>(
        &self,
        scene: &TScene,
        position: &nalgebra::Vector3<f32>,
        direction: &nalgebra::Vector3<f32>,
        additional_params: T,
    ) -> (f32, f32, f32);
}

pub struct EnumerateLightResult {
    pub centers: Vec<nalgebra::Vector3<f32>>,
}

pub trait IScene {
    fn cast_ray(
        &self,
        from: &nalgebra::Vector3<f32>,
        to: &nalgebra::Vector3<f32>,
    ) -> Option<MaterialInfo>;

    fn enumerate_related_lights(&self, position: &nalgebra::Vector3<f32>) -> EnumerateLightResult;

    fn find_background_color(
        &self,
        position: &nalgebra::Vector3<f32>,
        direction: &nalgebra::Vector3<f32>,
    ) -> nalgebra::Vector3<f32>;
}

pub trait IBuffer {
    fn get_width(&self) -> i32;

    fn get_height(&self) -> i32;

    fn set_color(&mut self, x: i32, y: i32, red: u8, green: u8, blue: u8);
}

pub trait IVectorComponent3<TFloat>
where
    TFloat: num::Float,
{
    fn x(&self) -> TFloat;

    fn y(&self) -> TFloat;

    fn z(&self) -> TFloat;

    fn set_x(&mut self, x: TFloat);

    fn set_y(&mut self, y: TFloat);

    fn set_z(&mut self, z: TFloat);
}

pub trait IConstract<TFloat: num::Float> {
    fn new(x: TFloat, y: TFloat, z: TFloat) -> Self;
}

pub trait INorm {
    fn norm(&self) -> f32;
}

pub trait INormalized {
    fn normalized(&self) -> Self;
}

pub trait IInnerProduct<T>
where
    T: num::Float,
{
    fn dot(&self, other: &Self) -> T;
}

pub trait IOuterProduct {
    fn cross(&self, other: &Self) -> Self;
}

pub trait IRandomEngine<TFloat>
where
    TFloat: num::Float,
{
    fn generate_range(&mut self, range: Range<TFloat>) -> TFloat;
}
