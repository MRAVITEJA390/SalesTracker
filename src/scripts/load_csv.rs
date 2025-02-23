use crate::config::APP_CONFIG;

use sqlx::PgPool;
use chrono::Utc;
use tokio::time::sleep;
use log::{error, info};
use std::time::Duration;

use super::lib::load_csv;


pub async fn refresh_data(pool: &PgPool, csv_path: &str) -> Result<(), Box<dyn std::error::Error>> {
    info!("Loading CSV file: {}", csv_path);

    match load_csv(&csv_path, pool).await {
        Ok(_) => {
            info!("Data refreshed successfully at {}", Utc::now());
            Ok(())
        },
        Err(err) => {
            error!("Failed to refresh data: {}", err);
            Err(err)
        },
    }
}


pub async fn start_data_refresh_job(pool: PgPool, csv_path: String) {
    loop {
        match refresh_data(&pool, &csv_path).await {
            Ok(_) => info!("Scheduled data refresh successful."),
            Err(e) => error!("Scheduled data refresh failed: {:?}", e),
        }

        // Wait for 24 hours before the next run
        sleep(Duration::from_secs(APP_CONFIG.refresh_interval)).await;
    }
}
