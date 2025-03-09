use std::sync::{Arc, RwLock};

use sjrt::{Color, IBuffer};
type RenderingTask = tokio::task::JoinHandle<TaskContext>;

struct TaskContext {
    scene: Arc<sjrt::util::RapierScene>,
    buffer: AccumulateBuffer,
}

struct RenderRequest {
    width: u32,
    height: u32,
}

pub struct Instance {
    sampling_count: u16,
    current_sampling_count: u16,
    image: Arc<RwLock<image::RgbImage>>,
    rendering_task: Option<RenderingTask>,
    render_request: Option<RenderRequest>,
}

impl Instance {
    pub fn new() -> Self {
        let sampling_count = 1;
        Self {
            sampling_count,
            current_sampling_count: 0,
            image: Arc::new(RwLock::new(image::RgbImage::new(256, 256))),
            rendering_task: None,
            render_request: None,
        }
    }

    pub async fn update(&mut self) {
        if let Some(rendering_task) = &self.rendering_task {
            // タスク実行中だけどキャンセル要求が出てなければ続行
            let is_cancel_requested = false;
            if !is_cancel_requested {
                return;
            }

            // タスクが終わってなければ続行
            if !rendering_task.is_finished() {
                return;
            }

            let mut rendering_task = None;
            std::mem::swap(&mut rendering_task, &mut self.rendering_task);
            let render_request = rendering_task.unwrap().await.unwrap();

            // 規定回数以上のサンプリングが完了していたら終わる
            self.current_sampling_count += 1;
            if self.sampling_count < self.current_sampling_count {
                return;
            }

            // 次のサンプリングを開始
            self.rendering_task = Some(Self::spawn_task(render_request));
        } else {
            // タスク作成要求が出てたら作成
            let mut render_request = None;
            std::mem::swap(&mut render_request, &mut self.render_request);
            let Some(render_request) = render_request else {
                return;
            };

            // TODO: ここでタスク作成
            let task_context = TaskContext {
                scene: Arc::new(sjrt::util::RapierScene::new()),
                buffer: AccumulateBuffer {
                    width: render_request.width as i32,
                    height: render_request.height as i32,
                    image: self.image.clone(),
                },
            };
            let new_task = Self::spawn_task(task_context);
            self.rendering_task = Some(new_task);
        }
    }

    #[allow(unused)]
    pub fn request_render(&mut self, width: u32, height: u32) {
        let render_request = RenderRequest { width, height };
        self.render_request = Some(render_request);
    }

    #[allow(unused)]
    pub fn peek_rendered_image<W>(&self, writer: &mut W)
    where
        W: std::io::Write + std::io::Seek,
    {
        let image = self.image.read().unwrap();
        image.write_to(writer, image::ImageFormat::Png);
    }

    fn spawn_task(mut task_context: TaskContext) -> RenderingTask {
        tokio::spawn(async move {
            let scene = task_context.scene.clone();
            let renderer = Arc::new(sjrt::PathTracerEx::default());
            let buffer = &mut task_context.buffer;
            let executor = sjrt::Executor::default();
            let rays = sjrt::Camera::builder()
                .build()
                .calculate_ray_direction(640, 480);
            executor
                .execute_async(buffer, rays.into_iter(), scene, renderer)
                .await;
            task_context
        })
    }
}

struct AccumulateBuffer {
    width: i32,
    height: i32,
    image: Arc<RwLock<image::RgbImage>>,
}

impl IBuffer for AccumulateBuffer {
    fn get_width(&self) -> i32 {
        self.width
    }

    fn get_height(&self) -> i32 {
        self.height
    }

    fn set_color(&mut self, x: i32, y: i32, red: u8, green: u8, blue: u8) {
        let mut image = self.image.write().unwrap();
        let current = image.get_pixel(x as u32, y as u32);
        let new_value = [
            current.0[0] + red,
            current.0[1] + green,
            current.0[2] + blue,
        ];
        image.put_pixel(x as u32, y as u32, image::Rgb::from(new_value));
    }
}

impl sjrt::IColorBuffer for &mut AccumulateBuffer {
    type Color = Color;

    fn write(&mut self, x: u32, y: u32, color: sjrt::Color) {
        let data = match color {
            sjrt::Color::R8G8B8A8_Uint(data) => data,
            sjrt::Color::R32G32B32A32_Unorm(data) => [
                (data[0] * 255.0).clamp(0.0, 255.0) as u8,
                (data[1] * 255.0).clamp(0.0, 255.0) as u8,
                (data[2] * 255.0).clamp(0.0, 255.0) as u8,
                (data[3] * 255.0).clamp(0.0, 255.0) as u8,
            ],
        };
        self.set_color(x as i32, y as i32, data[0], data[1], data[2]);
    }
}
