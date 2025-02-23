use sqlx::postgres::PgPoolOptions;
use sqlx::{Pool, Postgres};
use std::env;
use std::sync::LazyLock;

pub struct DatabaseConfig {
    pub connection_string: String,
    pub max_connections: u32,
}

impl DatabaseConfig {
    pub fn new() -> DatabaseConfig {
        dotenvy::from_filename("db.env").ok();
        DatabaseConfig {
            connection_string: env::var("DATABASE_URL").expect("DATABASE_URL must be set"),
            max_connections: env::var("DATABASE_MAX_CONNECTIONS").unwrap_or("5".to_string()).parse::<u32>().unwrap()
        }
    }

    pub async fn get_pool(&self) -> Result<Pool<Postgres>, sqlx::Error> {
        PgPoolOptions::new()
            .max_connections(self.max_connections)
            .connect(&self.connection_string)
            .await
    }
}

pub const DB_CONFIG: LazyLock<DatabaseConfig> = LazyLock::new(|| DatabaseConfig::new());