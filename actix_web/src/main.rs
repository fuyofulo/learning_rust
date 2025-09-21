use actix_web::{get, HttpResponse, Responder, HttpServer, App, post, web};
use std::sync::Mutex;


struct AppStateWithCounter{
    counter: Mutex<i32>
}

struct AppState {
    app_name: String
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {

    let counter = web::Data::new(AppStateWithCounter{
        counter: Mutex::new(0)
    });

    HttpServer::new( move || {
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
                app_name: String::from("Actix Web")
            }))
            .service(index)
            .app_data(counter.clone())
            .service(increase)
            .service(decrease)
            .service(getcounter)
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

#[get("/index")]
async fn index(data: web::Data<AppState>) -> impl Responder {
    let app_name = &data.app_name;
    format!("Hello {app_name}!")
}

#[post("/increase")]
async fn increase(data: web::Data<AppStateWithCounter>) -> String {
    let mut counter = data.counter.lock().unwrap();
    *counter += 1;
    format!("request number: {counter}")
}

#[post("/decrease")]
async fn decrease(data: web::Data<AppStateWithCounter>) -> String {
    let mut counter = data.counter.lock().unwrap();
    *counter -= 1;
    format!("request number: {counter}")
}

#[get("/getcounter")]
async fn getcounter(data: web::Data<AppStateWithCounter>) -> String {
    let counter = data.counter.lock().unwrap();
    format!("counter number: {counter}")
}

