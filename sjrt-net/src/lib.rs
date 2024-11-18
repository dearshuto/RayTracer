mod detail {
    tonic::include_proto!("sjrt");
}

pub use detail::ImageView;

mod rendering_client;
pub use rendering_client::RenderingClient;

mod rendering_server;
pub use rendering_server::RenderingServer;
