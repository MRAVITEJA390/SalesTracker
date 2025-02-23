use std::sync::LazyLock;

pub struct LoggerConfig {
    pub log_level: log::LevelFilter
}


impl LoggerConfig {
    pub fn new() -> LoggerConfig {
        LoggerConfig {
            log_level: log::LevelFilter::Info
        }
    }

    pub fn init(&self) {
        env_logger::init();
    }
}

pub const  LOGGER_CONFIG: LazyLock<LoggerConfig> = LazyLock::new(|| LoggerConfig::new());