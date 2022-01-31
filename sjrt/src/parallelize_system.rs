use crate::{IBuffer, IRenderer, IScene, Vector3f};
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
        Self{
            thread_count_x: x,
            thread_count_y: y
        }
    }

    pub fn execute<TScene: IScene + std::marker::Send + 'static, TBuffer: IBuffer, TRenderer: IRenderer + std::marker::Send + 'static>(
        &self,
        scene: std::sync::Arc<TScene>,
        buffer: &mut TBuffer,
        renderer: std::sync::Arc<TRenderer>,
    ) {
        let width = buffer.get_width();
        let height = buffer.get_height();

        let width_count = self.thread_count_x as i32;
        let height_count = self.thread_count_y as i32;
        let shared_scene = scene.clone();
        let shared_renderer = renderer.clone();
        let runtime = tokio::runtime::Runtime::new().unwrap();
        runtime.block_on(async {
            let mut handles = Vec::new();
            let partial_width = width / width_count;
            let partial_height = height / height_count;
            for w in 0..width_count {
                for h in 0..height_count {
                    let thread_renderer = shared_renderer.clone();
                    let thread_scene = shared_scene.clone();

                    let handle = tokio::task::spawn(async  move{
                        Self::execute_impl(width, height, thread_scene, thread_renderer, w * partial_width..(w + 1) * partial_width, h *partial_height..(h+1) * partial_height)
                    });
                    handles.push(handle);
                }
            }

            let results = futures::future::join_all(handles).await;
            for result in results {
                result.unwrap().write(buffer);
            }
        });
    }

    fn execute_impl<TScene: IScene, TRenderer: IRenderer>(
        width: i32,
        height: i32,
        scene: std::sync::Arc<TScene>,
        renderer: std::sync::Arc<TRenderer>,
        width_range: Range<i32>,
        height_range: Range<i32>,
    ) -> ImageView {
        let lower_left = Vector3f::new(-5.0, -5.0, 0.0);
        let stride_width = 10.0 / (width as f32);
        let stride_height = 10.0 / (height as f32);

        let mut image_view = ImageView::new(width_range.clone(), height_range.clone());
        for y in height_range.start..height_range.end {
            for x in width_range.start..width_range.end {
                let camera_position = Vector3f::new(0.0, 2.0, 5.0);
                let local_target = lower_left
                    + Vector3f::new((x as f32) * stride_width, y as f32 * stride_height, 0.0);
                let direction = local_target - camera_position;
                let (red_result, green_result, blue_result) =
                    renderer.render(scene.as_ref(), &camera_position, &direction);
                image_view.set_color(
                    x,
                    y,
                    (255.0 * red_result) as u8,
                    (255.0 * green_result) as u8,
                    (255.0 * blue_result) as u8,
                );
            }
        }

        image_view
    }
}

struct ImageView
{
    buffer: Vec<image::Rgb<u8>>,
    width_range: Range<i32>,
    height_range: Range<i32>,
}

impl ImageView {
    pub fn new(width_range: Range<i32>, height_range: Range<i32>) -> Self {
        let size = width_range.len() * height_range.len();
        Self {
            buffer: vec![image::Rgb([0, 0, 0]); size],
            width_range,
            height_range
        }
    }

    pub fn set_color(&mut self, x: i32, y: i32, red: u8, green: u8, blue: u8) {
        assert!(self.width_range.start <= x && x < self.width_range.end);
        assert!(self.height_range.start <= y && y < self.height_range.end);

        let index = self.to_index(x, y);
        self.buffer[index] = image::Rgb([red, green, blue]);
    }

    pub fn write<TBuffer: IBuffer>(&self, buffer: &mut TBuffer) {
        for index in 0..self.buffer.len() {
            let (x, y) = self.to_position(index);
            let data = self.buffer[index];
            buffer.set_color(x, buffer.get_height() -  y - 1, data[0], data[1], data[2]);
        }
    }

    fn to_index(&self, x: i32, y: i32) -> usize {
        let local_x = x - self.width_range.start;
        let local_y = y - self.height_range.start;
        let local_width = self.width_range.end - self.width_range.start;
        (local_x + local_width * local_y) as usize
    }

    fn to_position(&self, local_index: usize) -> (i32, i32) {
        let local_width = self.width_range.end - self.width_range.start;
        let local_y  = (local_index / (local_width as usize)) as i32;
        let local_x = (local_index % (local_width as usize)) as i32;
        (local_x + self.width_range.start, local_y + self.height_range.start)
    }
}
