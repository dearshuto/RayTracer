use crate::detail;
use std::net::SocketAddr;
use warp::Filter;

pub struct Server;

impl Server {
    pub async fn serve(addr: SocketAddr) {
        let http_filter = detail::server::http::Server::filter();
        let ws_filter = detail::server::ws::Server::filter();
        let filter = http_filter.or(ws_filter);
        warp::serve(filter).run(addr).await;
    }
}
