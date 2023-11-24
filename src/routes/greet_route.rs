use actix_web::{Responder, HttpResponse};

pub async fn greet() -> impl Responder {
    log::info!("Handling request for /");
    HttpResponse::Ok().body("Hello world!")
}