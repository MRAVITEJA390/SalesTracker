use actix_web::{HttpResponse, get, Responder};

#[get("/api/v1/hello")]
pub async fn hello_route() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}