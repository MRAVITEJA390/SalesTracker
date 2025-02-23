use actix_web::{get, HttpResponse, Responder};

#[get("/api/v1/health_check")]
pub async fn health_check_route() -> impl Responder {
    HttpResponse::Ok().body("Healthy")
}