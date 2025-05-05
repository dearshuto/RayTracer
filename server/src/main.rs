use clap::Parser;
use std::{net::ToSocketAddrs, sync::Arc};

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    #[clap(short = 'p', long = "socket-addr")]
    socket_address: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    let Ok(mut socket_addr) = args.socket_address.to_socket_addrs() else {
        return Ok(());
    };

    let socket_addr = socket_addr.next().unwrap();

    // http and ws
    let server = sjrt_net::Server::serve(socket_addr);

    // RPC
    let scene = Arc::new(sjrt::util::RapierScene::new());
    let rendering_server = sjrt_net::RenderingServer::new(scene);

    let mut socket_addr_rpc = socket_addr.clone();
    socket_addr_rpc.set_port(socket_addr.port() + 1);
    let rpc_server = rendering_server.run(socket_addr_rpc);

    println!("http or ws: {:?}", socket_addr);
    println!("Rpc: {:?}", socket_addr);
    println!("");
    println!(
        "http request ex: curl 'http://127.0.0.1:8080?width=128&height=128&sampling_count=100&thread_count_x=4&thread_count_y=4' --output ,/test.png"
    );
    println!(
        "ws request ex  : curl -i -N -H \"Connection: Upgrade\" -H \"Upgrade: websocket\" -H \"Sec-WebSocket-Version: 13\" -H \"Sec-WebSocket-Key: WIY4slX50bnnSF1GaedKhg==\" -H \"Host: localhost:8080\" http://localhost:8080/chat/ws"
    );

    futures::join!(rpc_server, server);

    Ok(())
}
