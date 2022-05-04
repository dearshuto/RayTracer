mod parallelize_system;
pub use parallelize_system::ParallelizeSystem;

use crate::{IBuffer, IScene, IRenderer, Camera};

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
            buffer.set_color(
                buffer.get_width() - (ray_info.x as i32) - 1,
                buffer.get_height() - (ray_info.y as i32) - 1,
                (255.0 * red_result) as u8,
                (255.0 * green_result) as u8,
                (255.0 * blue_result) as u8,
            );
        }
    }
}
