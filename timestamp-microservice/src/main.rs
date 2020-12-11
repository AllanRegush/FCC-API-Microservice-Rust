use actix_web::{get, App, HttpResponse, HttpServer, Responder};
use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello World")
}

#[derive(Deserialize, Serialize)]
struct Time {
    unix: i64,
    utc: String
}

#[get("/timestamp")]
async fn timestamp() -> impl Responder {
    let now = Utc::now();
    let time = Time {
        unix: now.timestamp(),
        utc: now.to_rfc2822()
    };
    HttpResponse::Ok().json(time)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new( || {
        App::new()
        .service(hello)
        .service(timestamp)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
