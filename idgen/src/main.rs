use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};
use nanoid::nanoid;
use snowflake::SnowflakeIdGenerator;
use std::{sync::Mutex};
use once_cell::sync::Lazy;


static SNOWFLAKE_ID_GENERATOR: Lazy<Mutex<SnowflakeIdGenerator>> = Lazy::new(|| {
    let idgen = SnowflakeIdGenerator::new(1, 1);
    Mutex::new(idgen)
});


#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("this is idgen")
}

async fn ping() -> impl Responder {
    HttpResponse::Ok().body("pong")
}

async fn generate_revision() -> impl Responder {
    let rev = SNOWFLAKE_ID_GENERATOR.lock().unwrap().generate();
    HttpResponse::Ok().body(rev.to_string())
}

async fn generate_id() -> impl Responder {
    let id = nanoid!(20);
    HttpResponse::Ok().body(id)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(hello)
            .route("/ping", web::get().to(ping))
            .route("/revision", web::get().to(generate_revision))
            .route("/id", web::get().to(generate_id),
            )
    })
        .bind(("0.0.0.0", 3030))?
        .run()
        .await
}