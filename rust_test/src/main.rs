use actix_web::{get, App, HttpResponse, HttpServer, Responder};
use serde::Serialize;

#[derive(Serialize)]
struct Message {
    message: String,
}

#[get("/")]
async fn hello() -> impl Responder {
    let response = Message {
        message: "Hello, World!".to_string(),
    };
    HttpResponse::Ok().json(response)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(hello))
        .bind(("0.0.0.0", 8000))?
        .run()
        .await
}