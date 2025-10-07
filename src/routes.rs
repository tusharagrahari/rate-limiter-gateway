use actix_web::{get, HttpResponse, Responder};



#[get("/")]
pub async fn home() -> impl Responder {
    HttpResponse::Ok().body("Welcome to Actix Web Server 🚀")
}


#[get("/ping")]
pub async fn ping() -> impl Responder {
    HttpResponse::Ok().body("PONG")
}

#[get("/test")]
pub async fn test() -> impl Responder {
    HttpResponse::Ok().body("test route working")
}