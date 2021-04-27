use std::net::TcpListener;
use actix_web::{web, HttpServer, dev::Server, App};
use crate::routes;
use sqlx::SqliteConnection;

pub fn run(listener: TcpListener, connection: SqliteConnection) -> Result<Server, std::io::Error> {

    let server = HttpServer::new(move || {
        App::new()
            .route("/health_check", web::get().to(routes::health_check))
            .route("/subscriptions", web::post().to(routes::subscriptions))
            .data(connection.clone())
        })
        .listen(listener)?
        .run();
    Ok(server)
}