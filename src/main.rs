use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use serde::{Deserialize, Serialize};


#[get("/")]
async fn home() -> impl Responder {
    HttpResponse::Ok().body("Welcome to Actix Web Server 🚀")
}


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Server running on http://localhost:8080");

    HttpServer::new(|| {
        App::new()
            .service(home)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}