use crate::{MaterialInfo, Vector3f};

pub trait IRenderer<TFloat, TVector3>
where
    TFloat: num::Float,
    TVector3: IVector3<TFloat>,
{
    fn render<TScene: IScene>(
        &self,
        scene: &TScene,
        position: &TVector3,
        direction: &TVector3,
    ) -> (f32, f32, f32);
}

pub struct EnumerateLightResult<TFloat, TVector3>
where
    TFloat: num::Float,
    TVector3: IVector3<TFloat>,
{
    pub centers: Vec<TVector3>,
    pub marker: std::marker::PhantomData<TFloat>,
}

pub trait IScene<TFloat, TVector3, TVectorComponent3>
where
    TFloat: num::Float,
    TVector3: IVector3<TFloat>,
    TVectorComponent3: IVectorComponent3<TFloat>,
{
    fn cast_ray(
        &self,
        from: &TVector3,
        to: &TVector3,
    ) -> Option<MaterialInfo<TFloat, TVector3, TVectorComponent3>>;

    fn enumerate_related_lights(
        &self,
        position: &Vector3f,
    ) -> EnumerateLightResult<TFloat, TVector3>;

    fn find_background_color(&self, position: &TVector3, direction: &TVector3) -> TVector3;
}

pub trait IBuffer {
    fn get_width(&self) -> i32;

    fn get_height(&self) -> i32;

    fn set_color(&mut self, x: i32, y: i32, red: u8, green: u8, blue: u8);
}

pub trait IVector3<TFloat>
where
    TFloat: num::Float,
{
    fn zero() -> Self;

    fn dot(&self, other: &Self) -> TFloat;

    fn normalize(&self) -> Self;

    fn length(&self) -> TFloat;
}

pub trait IVectorComponent3<TFloat>
where
    TFloat: num::Float,
{
    fn new(x: TFloat, y: TFloat, z: TFloat) -> Self;

    fn x(&self) -> TFloat;

    fn y(&self) -> TFloat;

    fn z(&self) -> TFloat;

    fn set_x(&mut self, x: TFloat);

    fn set_y(&mut self, y: TFloat);

    fn set_z(&mut self, z: TFloat);
}
