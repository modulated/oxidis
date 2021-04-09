use warp::{Filter, http::StatusCode};
mod data;
mod db;
mod error;
mod handler;

#[tokio::main]
async fn main() {
    let health_route = warp::path!("health")
        .map(|| {
            println!("Helathy.");
            StatusCode::OK
        });

    let routes = health_route.with(warp::cors().allow_any_origin());

    warp::serve(routes).run(([127,0,0,1], 8000)).await;
}