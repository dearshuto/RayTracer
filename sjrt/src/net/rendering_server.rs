use tonic::Response;

use super::detail::{self, renderer_server::Renderer};
use crate::{IBuffer, IScene};
use std::sync::Arc;

pub struct RenderingServer<TScene>
where
    TScene: IScene + std::marker::Send + 'static,
{
    scene: Arc<TScene>,
}

impl<TScene> RenderingServer<TScene>
where
    TScene: IScene + std::marker::Send + 'static,
{
    pub fn new(scene: Arc<TScene>) -> Self {
        Self {
            scene: scene.clone(),
        }
    }

    pub async fn run(self, address: std::net::SocketAddr) {
        let _result = tonic::transport::Server::builder()
            .add_service(detail::renderer_server::RendererServer::new(self))
            .serve(address)
            .await;
    }
}

#[tonic::async_trait]
impl<TScene> Renderer for RenderingServer<TScene>
where
    TScene: IScene + std::marker::Send + 'static,
{
    async fn render(
        &self,
        request: tonic::Request<detail::RenderRequest>,
    ) -> Result<tonic::Response<detail::ImageView>, tonic::Status> {
        let info = request.into_inner();
        let width = info.width;
        let height = info.height;
        let thread_count_x = info.thread_count_x as u8;
        let thread_count_y = info.thread_count_y as u8;
        let sampling_count = info.sampling_count as u16;

        let depth_count_max = 16;
        let is_nee_enabled = false;
        println!("Size: {}x{}", width, height);
        println!("Sampling Count: {}", sampling_count);
        println!("Depth: {}", depth_count_max);
        println!("NEE: false");

        let renderer = crate::PathTracer::new(sampling_count, depth_count_max, is_nee_enabled);
        let mut buffer = Buffer::new(width, height);
        crate::ParallelizeSystem::new_with_thread(thread_count_x, thread_count_y)
            .execute(self.scene.clone(), &mut buffer, Arc::new(renderer))
            .await;

        let request = detail::ImageView {
            width_start: 0,
            width_end: width,
            height_start: 0,
            height_end: height,
            pixels: buffer.to_response_data(),
        };

        println!("DONE!");
        Ok(Response::new(request))
    }
}

struct Buffer {
    width: i32,
    height: i32,
    buffer: Vec<u32>,
}

impl Buffer {
    pub fn new(width: i32, height: i32) -> Self {
        Self {
            width,
            height,
            buffer: vec![0; (width * height) as usize],
        }
    }

    pub fn to_response_data(self) -> Vec<u32> {
        self.buffer
    }
}

impl IBuffer for Buffer {
    fn get_width(&self) -> i32 {
        self.width
    }

    fn get_height(&self) -> i32 {
        self.height
    }

    fn set_color(&mut self, x: i32, y: i32, red: u8, green: u8, blue: u8) {
        let value = (((red as u32) << 24) & 0xFF000000)
            | (((green as u32) << 16) & 0xFF0000)
            | (((blue as u32) << 8) & 0xFF00)
            | (std::u8::MAX as u32);
        let index = (x + self.width * y) as usize;
        self.buffer[index] = value;
    }
}
