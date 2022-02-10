mod detail {
    tonic::include_proto!("sjrt");
}

mod rendering_client;
pub use rendering_client::RenderingClient;

mod rendering_server;
pub use rendering_server::RenderingServer;
