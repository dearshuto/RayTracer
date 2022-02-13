use crate::{IBuffer, IRenderer, IScene, Camera};
use std::ops::Range;

pub struct ParallelizeSystem {
    thread_count_x: u8,
    thread_count_y: u8,
}

impl ParallelizeSystem {
    pub fn new() -> Self {
        Self::new_with_thread(1, 1)
    }

    pub fn new_with_thread(x: u8, y: u8) -> Self {
        Self {
            thread_count_x: x,
            thread_count_y: y,
        }
    }

    pub async fn execute<
        TScene: IScene + std::marker::Send + 'static,
        TBuffer: IBuffer,
        TRenderer: IRenderer + std::marker::Send + 'static,
    >(
        &self,
        scene: std::sync::Arc<TScene>,
        buffer: &mut TBuffer,
        renderer: std::sync::Arc<TRenderer>,
    ) {
        let width = buffer.get_width() as u32;
        let height = buffer.get_height() as u32;

        let width_count = self.thread_count_x as u32;
        let height_count = self.thread_count_y as u32;
        let mut handles = Vec::new();
        let partial_width = (width / width_count) as u32;
        let partial_height = height / height_count;
        for w in 0..width_count {
            for h in 0..height_count {
                let thread_renderer = renderer.clone();
                let thread_scene = scene.clone();

                let handle = tokio::task::spawn(async move {
                    Self::execute_impl(
                        width,
                        height,
                        thread_scene,
                        thread_renderer,
                        w * partial_width..(w + 1) * partial_width,
                        h * partial_height..(h + 1) * partial_height,
                    )
                });
                handles.push(handle);
            }
        }

        let results = futures::future::join_all(handles).await;
        for result in results {
            result.unwrap().write(buffer);
        }
    }

    fn execute_impl<TScene: IScene, TRenderer: IRenderer>(
        width: u32,
        height: u32,
        scene: std::sync::Arc<TScene>,
        renderer: std::sync::Arc<TRenderer>,
        width_range: Range<u32>,
        height_range: Range<u32>,
    ) -> ImageView {
        let camera = Camera::new(width, height);
        let mut image_view = ImageView::new(width_range.clone(), height_range.clone());
        for ray_info in camera.calculate_ray_direction_range(width_range, height_range) {
            let (red_result, green_result, blue_result) =
                renderer.render(scene.as_ref(), &camera.position, &ray_info.directions[0]);
            image_view.set_color(
                ray_info.x as i32,
                ray_info.y as i32,
                (255.0 * red_result) as u8,
                (255.0 * green_result) as u8,
                (255.0 * blue_result) as u8,
            );
        }

        image_view
    }
}

struct ImageView {
    buffer: Vec<image::Rgb<u8>>,
    width_range: Range<u32>,
    height_range: Range<u32>,
}

impl ImageView {
    pub fn new(width_range: Range<u32>, height_range: Range<u32>) -> Self {
        let size = width_range.len() * height_range.len();
        Self {
            buffer: vec![image::Rgb([0, 0, 0]); size],
            width_range,
            height_range,
        }
    }

    pub fn set_color(&mut self, x: i32, y: i32, red: u8, green: u8, blue: u8) {
        assert!(self.width_range.start <= x as u32 && (x as u32) < self.width_range.end);
        assert!(self.height_range.start <= y as u32 && (y as u32) < self.height_range.end);

        let index = self.to_index(x, y);
        self.buffer[index] = image::Rgb([red, green, blue]);
    }

    pub fn write<TBuffer: IBuffer>(&self, buffer: &mut TBuffer) {
        for index in 0..self.buffer.len() {
            let (x, y) = self.to_position(index);
            let data = self.buffer[index];
            buffer.set_color(
                buffer.get_width() - x - 1,
                buffer.get_height() - y - 1,
                data[0],
                data[1],
                data[2],
            );
        }
    }

    fn to_index(&self, x: i32, y: i32) -> usize {
        let local_x = x as u32 - self.width_range.start;
        let local_y = y as u32 - self.height_range.start;
        let local_width = self.width_range.end - self.width_range.start;
        (local_x + local_width * local_y) as usize
    }

    fn to_position(&self, local_index: usize) -> (i32, i32) {
        let local_width = self.width_range.end - self.width_range.start;
        let local_y = (local_index / (local_width as usize)) as i32;
        let local_x = (local_index % (local_width as usize)) as i32;
        (
            local_x + self.width_range.start as i32,
            local_y + self.height_range.start as i32,
        )
    }
}
