use actix_web::{HttpServer, web, App, Responder};

struct AppState {
    name: String
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new( || {
        App::new()
            .app_data(web::Data::new(AppState{
                name: String::from("actix_web")
            }))
            .route("/", web::get().to(get_state)) // putting the / was important, it was not running otherwise
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}

async fn get_state(data: web::Data<AppState>) -> impl Responder {
    let data = &data.name;
    format!("app name is: {data}")
}