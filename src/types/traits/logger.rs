use std::fmt::Debug;

use crate::types::enums::log_level::LogLevel;

pub trait ILogger: Debug + Send {
    fn log(&self, message: &str, level: LogLevel);
}
