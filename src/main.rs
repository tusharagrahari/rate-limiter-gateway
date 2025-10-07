use actix_web::{App, HttpServer};

use crate::routes::home;
pub mod models;
pub mod routes;



#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Server running on http://localhost:8080");

    HttpServer::new(|| {
        App::new()
            .service(home)
            .service(routes::ping)
            .service(routes::test)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}