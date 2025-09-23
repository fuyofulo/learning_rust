use actix_web::{HttpServer, App, get, Responder, HttpResponse, web, post};
use std::sync::Mutex;

struct CounterStruct {
    counter: Mutex<i32>
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {

    let counter = web::Data::new(CounterStruct{
        counter: Mutex::new(0)
    });

    HttpServer::new( move || {
        App::new()
            .app_data(counter.clone())
            .service(hello_world)
            .service(increase)
            .service(decrease)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}

#[get("/hello_world")]
async fn hello_world () -> impl Responder {
    HttpResponse::Ok().body("hello world")
}

#[post("/increase")]
async fn increase(data: web::Data<CounterStruct>) -> String {
    let mut counter = data.counter.lock().unwrap();
    println!("counter before: {counter}");
    *counter += 1;
    println!("counter after: {counter}");
    format!("counter: {counter}")
}

#[post("/decrease")]
async fn decrease(data: web::Data<CounterStruct>) -> String {
    let mut counter = data.counter.lock().unwrap();
    println!("counter before: {counter}");
    *counter -= 1;
    println!("counter after: {counter}");
    format!("counter: {counter}")
}