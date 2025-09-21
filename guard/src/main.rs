use std::sync::Mutex;
use actix_web::{HttpServer, App, web, guard};

struct AppState {
    counter: Mutex<i32>
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {

    let counter = web::Data::new(AppState{
        counter: Mutex::new(0)
    });
    
    HttpServer::new( move || {
        App::new()
            .app_data(counter.clone())
            .service(
                web::scope("/increase")
                .guard(guard::Host("admin.localhost"))
                .route("", web::post().to(increase_admin))
            )
            .service(
                web::scope("/increase")
                .guard(guard::Host("user.localhost"))
                .route("", web::post().to(increase_user))
            )
            .route("/increase", web::post().to(increase_default))
            .service(
                web::scope("/decrease")
                .guard(guard::Host("admin.localhost"))
                .route("", web::post().to(decrease_admin))
            )
            .service(
                web::scope("/decrease")
                .guard(guard::Host("user.localhost"))
                .route("", web::post().to(decrease_user))
            )
            .route("/decrease", web::post().to(decrease_default))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await

}

async fn increase_admin(data: web::Data<AppState>) -> String {
    let mut counter = data.counter.lock().unwrap();
    *counter += 2;
    format!("counter value: {counter}")
}

async fn increase_user(data: web::Data<AppState>) -> String {
    let mut counter = data.counter.lock().unwrap();
    *counter += 1;
    format!("counter value: {counter}")
}

async fn increase_default(data: web::Data<AppState>) -> String {
    let counter = data.counter.lock().unwrap();
    format!("no header found, counter value still {counter}")
}

async fn decrease_admin(data: web::Data<AppState>) -> String {
    let mut counter = data.counter.lock().unwrap();
    *counter += 2;
    format!("counter value: {counter}")
}

async fn decrease_user(data: web::Data<AppState>) -> String {
    let mut counter = data.counter.lock().unwrap();
    *counter += 1;
    format!("counter value: {counter}")
}

async fn decrease_default(data: web::Data<AppState>) -> String {
    let counter = data.counter.lock().unwrap();
    format!("no header found, counter value still {counter}")
}