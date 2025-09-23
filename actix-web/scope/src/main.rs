use actix_web::{HttpResponse, HttpServer, Responder, web, App};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new( || {
        App::new()
            .service(
                web::scope("/app")
                .route("", web::get().to(index))
            )
    })
    .bind(("127.0.0.1", 8080))? // this question mark is important, it fails without it. 
    .run()
    .await
} 

async fn index() -> impl Responder {
    HttpResponse::Ok().body("scope function has been hit")
}