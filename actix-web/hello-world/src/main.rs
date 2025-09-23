use actix_web::{HttpServer, App, get, Responder, HttpResponse};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new( || {
        App::new()
            .service(hello_world)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}

#[get("/hello_world")]
async fn hello_world () -> impl Responder {
    HttpResponse::Ok().body("hello world")
}