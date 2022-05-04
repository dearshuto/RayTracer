use crate::{MaterialInfo, Vector3f};

pub trait IRenderer: Sync {
    fn render<TScene: IScene>(
        &self,
        scene: &TScene,
        position: &Vector3f,
        direction: &Vector3f,
    ) -> (f32, f32, f32);
}

pub struct EnumerateLightResult {
    pub centers: Vec<Vector3f>,
}

pub trait IScene: Sync {
    fn cast_ray(&self, from: &Vector3f, to: &Vector3f) -> Option<MaterialInfo>;

    fn enumerate_related_lights(&self, position: &Vector3f) -> EnumerateLightResult;

    fn find_background_color(&self, position: &Vector3f, direction: &Vector3f) -> Vector3f;
}

pub trait IBuffer {
    fn get_width(&self) -> i32;

    fn get_height(&self) -> i32;

    fn set_color(&mut self, x: i32, y: i32, red: u8, green: u8, blue: u8);
}
