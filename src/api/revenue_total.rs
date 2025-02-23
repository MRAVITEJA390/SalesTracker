use actix_web::{get, HttpResponse, Responder, web};
use sqlx::{PgPool, types::{BigDecimal, time::Date}};
use time::macros::format_description;
use log::error;

use super::revenue::{RevenueQuery, RevenueResponse};


#[get("/api/v1/revenue")]
async fn total_revenue_route(
    db: web::Data<PgPool>,
    query: web::Query<RevenueQuery>,
) -> impl Responder {
    let result = sqlx::query_as::<_, (Option<BigDecimal>,)>(
        "SELECT SUM(total_amount) AS total_revenue
         FROM orders
         WHERE date_of_sale BETWEEN $1 AND $2",
    )
    .bind(Date::parse(&query.start_date, &format_description!("[year]-[month]-[day]")).unwrap())
    .bind(Date::parse(&query.end_date, &format_description!("[year]-[month]-[day]")).unwrap())
    .fetch_one(db.get_ref())
    .await;

    match result {
        Ok((Some(total_revenue),)) => HttpResponse::Ok().json(RevenueResponse { total_revenue }),
        Ok((None,)) => HttpResponse::Ok().json(RevenueResponse { total_revenue: BigDecimal::from(0) }),
        Err(err) => {
            error!("Failed to fetch revenue data: {}", err);
            HttpResponse::InternalServerError().body("Failed to fetch revenue data")
        },
    }
}
