use crate::{Vector3f, MaterialInfo};

pub trait IRenderer
{
    fn render<TScene: IScene,TBuffer: IBuffer>(&self, scene: &TScene, buffer: &mut TBuffer);
}

pub trait IScene
{
    fn cast_ray(&self, from: &Vector3f, to: &Vector3f) -> Option<MaterialInfo>;
}

pub trait IBuffer
{
    fn get_width(&self) -> i32;

    fn get_height(&self) -> i32;

    fn set_color(&mut self, x: i32, y: i32 ,red: u8, green: u8, blue: u8);
}
