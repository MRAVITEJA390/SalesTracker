use actix_web::{get, HttpResponse, Responder, web};
use sqlx::PgPool;
use log::{error, info};
use tokio::time::{sleep, Duration};
use crate::scripts::refresh_data;


#[get("/api/v1/refresh")]
pub async fn refresh_route(pool: web::Data<PgPool>, csv_path: web::Data<String>) -> impl Responder {
    let pool_clone = pool.clone();
    let csv_path_clone = csv_path.clone();

    tokio::spawn(async move {
        sleep(Duration::from_secs(5)).await;
        match refresh_data(&pool_clone, &csv_path_clone).await {
            Ok(_) => info!("Scheduled data refresh successful."),
            Err(e) => error!("Scheduled data refresh failed: {:?}", e),
        }
        info!("Scheduled data refresh completed.");
    });
    HttpResponse::Ok().body("Scheduled data refresh successful.")
}