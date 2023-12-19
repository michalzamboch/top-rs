#![allow(dead_code)]

#[derive(Default, Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub enum LogLevel {
    #[default]
    Info,
    Warning,
    Error,
}
