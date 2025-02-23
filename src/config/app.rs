use dotenvy;
use std::env;
use std::sync::LazyLock;


pub struct AppConfig {
    pub refresh_interval: u64, // Refresh interval in seconds
    pub csv_path: String // Path to the CSV file
}

impl AppConfig {
    pub fn new() -> AppConfig {
        dotenvy::from_filename("app.env").ok();
        AppConfig {
            refresh_interval: env::var("REFRESH_INTERVAL").expect("REFRESH_INTERVAL must be set").parse().unwrap(),
            csv_path: env::var("CSV_PATH").expect("CSV_PATH must be set")
        }
    }
}

pub const APP_CONFIG: LazyLock<AppConfig> = LazyLock::new(|| AppConfig::new());