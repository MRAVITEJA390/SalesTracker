use serde::{Deserialize, Serialize};
use sqlx::types::BigDecimal;

#[derive(Deserialize)]
pub struct RevenueQuery {
    pub start_date: String,
    pub end_date: String,
}

#[derive(Serialize)]
pub struct RevenueResponse {
    pub total_revenue: BigDecimal,
}

#[derive(Serialize)]
pub struct BreakdownResponse {
    pub name: String,
    pub revenue: BigDecimal,
}

#[derive(Serialize)]
pub struct TrendResponse {
    pub period: String,
    pub revenue: f64,
}
