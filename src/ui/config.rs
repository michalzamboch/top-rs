use ratatui::prelude::*;

pub const REFRESH_MILIS: u64 = 750;

pub const MEMORY_COLOR: Color = Color::Blue;
pub const CPU_COLOR: Color = Color::Green;
pub const OVERLOAD_COLOR: Color = Color::Red;

pub const CPU_USAGE_TITLE: &str = " CPU usage ";
pub const MEM_USAGE_TITLE: &str = " Memory usage ";

pub const HUNDERED_PERCENT: u64 = 100;

pub const PID_COL_LEN: u16 = 9;
pub const NAME_COL_MAX_LEN: u16 = 500;
pub const CPU_USAGE_COL_LEN: u16 = 9;
pub const PRETTY_BYTES_COL_LEN: u16 = 12;
