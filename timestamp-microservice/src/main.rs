use actix_web::{get, App, HttpResponse, HttpServer, Responder, web};
use serde::{Deserialize, Serialize};
use chrono::{Utc, TimeZone, LocalResult};

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello World")
}

#[derive(Deserialize, Serialize)]
struct Time {
    unix: i64,
    utc: String
}

#[get("api/timestamp/")]
async fn timestamp() -> impl Responder {
    let now = Utc::now();
    let time = Time {
        unix: now.timestamp(),
        utc: now.to_rfc2822()
    };
    HttpResponse::Ok().json(time)
}


#[get("api/timestamp/{date}")]
async fn timestamp_date(date: web::Path<i64>) -> impl Responder {
    
    let now = Utc.timestamp_millis_opt(*date);
    match now {
        LocalResult::Single(dt) => {
            let time = Time {
                unix: dt.timestamp(),
                utc: dt.to_rfc2822()
            };
            HttpResponse::Ok().json(time)
        }
        _ => HttpResponse::Ok().content_type("application/json").body("{ \"error\":\"Invaild Date\" }")
    }
    /*
    let now = Utc::now();
    */

}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new( || {
        App::new()
        .service(hello)
        .service(timestamp)
        .service(timestamp_date)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
