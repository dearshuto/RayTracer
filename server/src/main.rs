use clap::Parser;
use std::{
    net::{IpAddr, SocketAddr},
    str::FromStr,
    sync::Arc,
};

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    #[clap(short = 'i', long = "ip-address")]
    ip_address: String,

    #[clap(short = 'p', long = "port")]
    port: u16,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    let Ok(addr) = IpAddr::from_str(&args.ip_address) else {
        return Ok(());
    };

    // RPC
    let scene = Arc::new(sjrt::util::RapierScene::new());
    let rendering_server = sjrt_net::RenderingServer::new(scene);
    let socket_addr = SocketAddr::new(addr, args.port);
    println!("Rpc: {:?}", socket_addr);
    let rpc_server = rendering_server.run(socket_addr);

    // http
    let socket_addr = SocketAddr::new(addr, args.port + 1);
    println!("http: {:?}", socket_addr);
    let http_server = sjrt_net::http::Server::serve(socket_addr);

    // WebSocket
    let socket_addr = SocketAddr::new(addr, args.port + 2);
    println!("WebSocket: {:?}", socket_addr);
    let web_socket_server = sjrt_net::ws::Server::serve(socket_addr);

    futures::join!(rpc_server, http_server, web_socket_server);

    Ok(())
}
