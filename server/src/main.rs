use clap::Parser;
use std::sync::Arc;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    #[clap(short = 'i', long = "ip-address")]
    ip_address: String,

    #[clap(short = 'p', long = "port")]
    port: i32,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    let scene = Arc::new(sjrt::RapierScene::new());
    let rendering_server = sjrt::net::RenderingServer::new(scene);
    println!("{}:{}", args.ip_address, args.port);
    let addr = format!("{}:{}", args.ip_address, args.port)
        .parse()
        .unwrap();
    rendering_server.run(addr).await;

    Ok(())
}
