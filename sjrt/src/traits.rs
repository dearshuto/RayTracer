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
}

pub trait IBuffer {
    fn get_width(&self) -> i32;

    fn get_height(&self) -> i32;

    fn set_color(&mut self, x: i32, y: i32, red: u8, green: u8, blue: u8);
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
        let lower_left = Vector3f::new(-5.0, -5.0, 0.0);
        let stride_width = 10.0 / (buffer.get_height() as f32);
        let stride_height = 10.0 / (buffer.get_height() as f32);

        for y in 0..buffer.get_height() {
            for x in 0..buffer.get_height() {
                let camera_position = Vector3f::new(0.0, 2.0, 5.0);
                let local_target = lower_left
                    + Vector3f::new((x as f32) * stride_width, y as f32 * stride_height, 0.0);
                let direction = local_target - camera_position;
                let (red_result, green_result, blue_result) =
                    renderer.render(scene, &camera_position, &direction);
                buffer.set_color(
                    x,
                    buffer.get_width() - y - 1,
                    (255.0 * red_result) as u8,
                    (255.0 * green_result) as u8,
                    (255.0 * blue_result) as u8,
                );
            }
        }
    }
}
