use actix_web::{dev::Server, HttpResponse, HttpServer, web, App};
use serde::Deserialize;
use std::net::TcpListener;

async fn health_check() -> HttpResponse {
    HttpResponse::Ok().finish()
}

/*
    serde does the heavylifting
    it's responsible for genercally serializing and deserializing data structures
    by providing a set of interfaces which they define as a data model
    Refer to 'Understanding Serde' by Josh Mcguigan for more info. on serde
*/
#[derive(Deserialize)]
struct FormData {
    email: String,
    name: String,
}

async fn subscribe(_form: web::Form<FormData>) -> HttpResponse {
    HttpResponse::Ok().finish()
}

pub fn run(listener: TcpListener) -> Result<Server, std::io::Error> {
    let server = HttpServer::new(|| {
        App::new()
            .route("/health_check", web::get().to(health_check))
            .route("/subscriptions", web::post().to(subscribe))
    })
    .listen(listener)?
    .run();

    Ok(server)
}
