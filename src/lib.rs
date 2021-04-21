use actix_web::{web, App, HttpServer, Responder, HttpResponse, dev::Server};
use std::net::TcpListener;

pub fn run(listener: TcpListener) -> Result<Server, std::io::Error> {

    let server = HttpServer::new(|| {
        App::new()
            .route("/health_check", web::get().to(health_check))
            .route("/subscriptions", web::post().to(subscriptions))
        })
        .listen(listener)?
        .run();
    Ok(server)
}

async fn health_check() -> HttpResponse {
    HttpResponse::Ok().finish()
}

#[derive(serde::Deserialize)]
struct FormData {
    email: String,
    name: String,
}

async fn subscriptions(_form: web::Form<FormData>) -> HttpResponse {
    // if form.email.len() > 0 && form.name.len() > 0 {
    //     return HttpResponse::Ok();
    // }
    HttpResponse::Ok().finish()
}
