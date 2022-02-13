use crate::{Camera, MaterialInfo, Vector3f};

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

    fn set_color_normalized(&mut self, x: i32, y: i32, red: f32, green: f32, blue: f32) {
        let new_red = (255.0 * red) as u8;
        let new_green = (255.0 * green) as u8;
        let new_blue = (255.0 * blue) as u8;
        self.set_color(x, y, new_red, new_green, new_blue);
    }
}

pub struct System {}

impl System {
    pub fn new() -> Self {
        Self {}
    }

    pub fn execute<TScene: IScene, TBuffer: IBuffer, TRenderer: IRenderer>(
        &self,
        scene: &TScene,
        buffer: &mut TBuffer,
        renderer: &TRenderer,
    ) {
        let camera = Camera::new(buffer.get_width() as u32, buffer.get_height() as u32);
        for ray_info in camera.calculate_ray_direction() {
            let (red_result, green_result, blue_result) =
                renderer.render(scene, &camera.position, &ray_info.directions[0]);
            buffer.set_color_normalized(
                buffer.get_width() - (ray_info.x as i32) - 1,
                buffer.get_height() - (ray_info.y as i32) - 1,
                red_result,
                green_result,
                blue_result,
            );
        }
    }
}
