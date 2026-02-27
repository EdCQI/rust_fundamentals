//web microservice for basic arithmetic operations
use actix_web::{App, HttpResponse, HttpServer, Responder, get, web};
#[get("/")]
async fn index() -> impl Responder {
    HttpResponse::Ok().body("Welcome to the Rust Calculator API!")
}

#[get("/add/{a}/{b}")]
async fn add(info: web::Path<(i32, i32)>) -> impl Responder {
    let (a, b) = info.into_inner();
    let result = calc::add(a, b);
    HttpResponse::Ok().body(format!("{} + {} = {}", a, b, result))
}

#[get("/subtract/{a}/{b}")]
async fn subtract(info: web::Path<(i32, i32)>) -> impl Responder {
    let (a, b) = info.into_inner();
    let result = calc::subtract(a, b);
    HttpResponse::Ok().body(format!("{} - {} = {}", a, b, result))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(index)
            .service(add)
            .service(subtract)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
