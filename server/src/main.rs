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

    // http and ws
    let socket_addr = SocketAddr::new(addr, args.port);
    let server = sjrt_net::Server::serve(socket_addr);

    // RPC
    let scene = Arc::new(sjrt::util::RapierScene::new());
    let rendering_server = sjrt_net::RenderingServer::new(scene);
    let socket_addr = SocketAddr::new(addr, args.port + 1);
    let rpc_server = rendering_server.run(socket_addr);

    println!("http or ws: {:?}", socket_addr);
    println!("Rpc: {:?}", socket_addr);
    println!("");
    println!("http request ex: curl 'http://127.0.0.1:8080?width=128&height=128&sampling_count=100&thread_count_x=4&thread_count_y=4' --output ,/test.png");
    println!("ws request ex  : curl -i -N -H \"Connection: Upgrade\" -H \"Upgrade: websocket\" -H \"Sec-WebSocket-Version: 13\" -H \"Sec-WebSocket-Key: WIY4slX50bnnSF1GaedKhg==\" -H \"Host: localhost:8080\" http://localhost:8080/chat/ws");

    futures::join!(rpc_server, server);

    Ok(())
}
