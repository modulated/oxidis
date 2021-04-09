use std::{env, net::SocketAddrV4};
use pretty_env_logger;
use log::*;
use warp::{Filter, http::StatusCode};
mod data;
mod db;
mod error;
mod handler;

#[tokio::main]
async fn main() {
    env::set_var("RUST_LOG", "TRACE");
    pretty_env_logger::init();
    info!("Initializing server.");
    let health_route = warp::path!("health")
        .map(|| {
            info!("Health check.");
            StatusCode::OK
        });

    let routes = health_route.with(warp::cors().allow_any_origin());

    let addr = SocketAddrV4::new(std::net::Ipv4Addr::new(127, 0, 0, 1), 8000);
    info!("Serving on {:?}.", addr);

    warp::serve(routes).run(addr).await;
}