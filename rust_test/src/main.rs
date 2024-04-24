use std::fs::File;
use std::io::{BufRead, BufReader};
use std::thread;
use std::time::Duration;

use actix_web::{get, App, HttpResponse, HttpServer, Responder};
use serde::Serialize;

#[derive(Serialize)]
struct Message {
    message: String,
}

#[get("/")]
async fn root() -> impl Responder {
    HttpResponse::Ok().json(Message {
        message: "Hello, World!".to_string(),
    })
}

#[get("/file_read")]
async fn file_read() -> impl Responder {
    let file = File::open("../data/tested.csv").unwrap();
    let reader = BufReader::new(file);

    for line in reader.lines() {
        if let Ok(_row) = line {
        }
    }

    HttpResponse::Ok().json(Message {
        message: "file_read".to_string(),
    })
}

#[get("/wait_time")]
async fn wait_time() -> impl Responder {
    thread::sleep(Duration::from_secs(1));
    HttpResponse::Ok().json(Message {
        message: "time_sleep".to_string(),
    })
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(root)
            .service(file_read)
            .service(wait_time)
    })
    .bind(("0.0.0.0", 8000))?
    .run()
    .await
}