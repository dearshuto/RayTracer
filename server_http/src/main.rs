use std::{
    net::{IpAddr, SocketAddr},
    str::FromStr,
};

use clap::Parser;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    #[clap(
        required = false,
        short = 'i',
        long = "ip-address",
        default_value = "127.0.0.1"
    )]
    ip_address: String,

    #[clap(required = false, short = 'p', long = "port", default_value = "8080")]
    port: u16,
}

// Run and request ↓
// ex. curl 'http://127.0.0.1:8080?width=200&height=200&sampling_count=256&thread_count_x=4&thread_count_y=4' --output output.png
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    let Ok(addr) = IpAddr::from_str(&args.ip_address) else {
        return Ok(());
    };

    let socket_addr = SocketAddr::new(addr, args.port);
    println!("run: {:?}", socket_addr);
    sjrt_net::http::Server::serve(socket_addr).await;

    Ok(())
}
