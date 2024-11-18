use super::detail::{self, ImageView};

pub struct RenderingClient {
    pub width: i32,
    pub height: i32,
    pub sampling_count: u16,
    pub thread_count_x: i32,
    pub thread_count_y: i32,
}

impl RenderingClient {
    pub async fn run(
        &self,
        address: std::net::SocketAddr,
    ) -> Result<tonic::Response<ImageView>, tonic::Status> {
        let request = detail::RenderRequest {
            width: self.width,
            height: self.height,
            sampling_count: self.sampling_count as u32,
            thread_count_x: self.thread_count_x,
            thread_count_y: self.thread_count_y,
        };
        let url = format!("http://{}", address.to_string());
        let mut client = detail::renderer_client::RendererClient::connect(url)
            .await
            .unwrap();
        client.render(request).await
    }
}
