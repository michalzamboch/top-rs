use crate::types::{
    enums::log_level::LogLevel,
    traits::{creatable::ICreatable, logger::ILogger},
};

use simplelog::*;
use std::fs::OpenOptions;

use super::config;

#[derive(Debug, Default)]
pub struct Logger {
    allow: bool,
}

impl ICreatable for Logger {
    fn new() -> Self {
        let file = OpenOptions::new()
            .write(config::LOG_WRITE)
            .create(config::LOG_CREATE)
            .append(config::LOG_APPEND)
            .open(config::LOG_FILE_NAME)
            .unwrap();

        CombinedLogger::init(vec![WriteLogger::new(
            LevelFilter::Info,
            Config::default(),
            file,
        )])
        .unwrap();

        Self {
            allow: config::ALLOW_LOG,
        }
    }
}

impl ILogger for Logger {
    fn log(&self, message: &str, level: LogLevel) {
        if !self.allow {
            return;
        }

        match level {
            LogLevel::Info => info!("{}", message),
            LogLevel::Warning => warn!("{}", message),
            LogLevel::Error => error!("{}", message),
        }
    }
}
