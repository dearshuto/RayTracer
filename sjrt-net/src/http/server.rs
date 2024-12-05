use crate::detail::generated::sjrt::RenderRequest;
use std::{io::Cursor, net::SocketAddr, sync::Arc, u8};

use sjrt::IBuffer;
use warp::{reject::Rejection, reply::Reply, Filter};

#[allow(unused)]
pub struct Server;

impl Server {
    #[allow(unused)]
    pub async fn serve(addr: SocketAddr) {
        let filter = warp::get()
            .and(warp::query::<RenderRequest>())
            .and_then(Self::render);
        warp::serve(filter).run(addr).await;
    }

    async fn render(render_request: RenderRequest) -> Result<impl Reply, Rejection> {
        let width = render_request.width;
        let height = render_request.height;
        let thread_count_x = render_request.thread_count_x as u8;
        let thread_count_y = render_request.thread_count_y as u8;
        let sampling_count = render_request.sampling_count as u16;

        let depth_count_max = 16;
        let is_nee_enabled = false;

        let scene = sjrt::util::RapierScene::new();

        let renderer = sjrt::PathTracer::new(sampling_count, depth_count_max, is_nee_enabled);
        let mut buffer = Buffer::new(width as u32, height as u32);
        sjrt::ParallelizeSystem::new_with_thread(thread_count_x, thread_count_y)
            .execute(Arc::new(scene), &mut buffer, Arc::new(renderer))
            .await;

        Ok(buffer)
    }
}

struct Buffer {
    width: u32,
    height: u32,
    image: image::RgbImage,
}

impl Buffer {
    pub fn new(width: u32, height: u32) -> Self {
        let image = image::RgbImage::new(width, height);

        Self {
            width,
            height,
            image,
        }
    }
}

impl IBuffer for Buffer {
    fn get_width(&self) -> i32 {
        self.width as i32
    }

    fn get_height(&self) -> i32 {
        self.height as i32
    }

    fn set_color(&mut self, x: i32, y: i32, red: u8, green: u8, blue: u8) {
        let rgb = image::Rgb::from([red, green, blue]);
        self.image.put_pixel(x as u32, y as u32, rgb);
    }
}

impl warp::Reply for Buffer {
    fn into_response(self) -> warp::reply::Response {
        let mut bytes: Vec<u8> = Vec::new();
        self.image
            .write_to(&mut Cursor::new(&mut bytes), image::ImageFormat::Png)
            .unwrap();
        warp::reply::Response::new(bytes.into())
    }
}
