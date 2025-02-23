use actix_web::{get, HttpResponse, Responder, web};
use sqlx::{PgPool, types::{BigDecimal, time::Date}};
use time::macros::format_description;
use log::error;
use super::revenue::{BreakdownResponse, RevenueQuery};

#[get("/api/v1/revenue/by-product")]
async fn revenue_by_product_route(
    db: web::Data<PgPool>,
    query: web::Query<RevenueQuery>,
) -> impl Responder {
    let start_date = Date::parse(&query.start_date, &format_description!("[year]-[month]-[day]"));
    if start_date.is_err() {
        return HttpResponse::BadRequest().body("Invalid start date");
    }
    let end_date = Date::parse(&query.end_date, &format_description!("[year]-[month]-[day]"));
    if end_date.is_err() {
        return HttpResponse::BadRequest().body("Invalid end date");
    }
    let result = sqlx::query_as::<_, (String, BigDecimal)>(
        r#"SELECT products.name AS name, 
                SUM(orders.total_amount) AS revenue
         FROM orders
         JOIN order_items ON orders.id = order_items.order_id
         JOIN products ON order_items.product_id = products.id
         WHERE orders.date_of_sale BETWEEN $1 AND $2
         GROUP BY products.name"#,
    )
    .bind(start_date.unwrap())
    .bind(end_date.unwrap())
    .fetch_all(db.get_ref())
    .await;

    match result {
        Ok(rows) => {
            let response: Vec<BreakdownResponse> = rows
                .into_iter()
                .map(|row| BreakdownResponse {
                    name: row.0,
                    revenue: row.1,
                })
                .collect();
            HttpResponse::Ok().json(response)
        }
        Err(err) => {
            error!("Failed to fetch revenue by product: {}", err);
            HttpResponse::InternalServerError().body("Failed to fetch revenue by product")
        },
    }
}
