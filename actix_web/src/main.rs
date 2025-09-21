use actix_web::{get, HttpResponse, Responder, HttpServer, App, post, web};

struct AppState {
    app_name: String
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(hello)
            .service(echo)
            .route("manual_hello", web::get().to(manual_hello))
            .service(
                web::scope("/app")
                    .service(app1)
                    .route("app2", web::get().to(app2))
            )
            .app_data(web::Data::new(AppState{
                app_nam: String::from("Actix Web")
            }))
            .service(index)

    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}


#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("hello world!")
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("manual hello!")
}

#[get("/app1")]
async fn app1() -> impl Responder {
    HttpResponse::Ok().body("app1")
}

async fn app2() -> impl Responder {
    HttpResponse::Ok().body("app2")
}

async fn index(data: web::Data<AppState>) -> impl Responder {
    let app_name = &data.app_name;
    format!("Hello {app_name}!")
}