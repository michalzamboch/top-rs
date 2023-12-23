use ratatui::prelude::*;

pub const REFRESH_MILIS: u64 = 600;

pub const MEMORY_COLOR: Color = Color::Green;
pub const SWAP_COLOR: Color = Color::Yellow;
pub const CPU_COLOR: Color = Color::Blue;
pub const PROCESS_COLOR: Color = CPU_COLOR;
pub const OVERLOAD_COLOR: Color = Color::Red;
pub const NETWORK_COLOR: Color = Color::Green;

pub const CPU_USAGE_TITLE: &str = " CPU usage ";
pub const MEM_USAGE_TITLE: &str = " Memory usage ";
pub const SWAP_USAGE_TITLE: &str = " Swap usage ";
pub const RECEIVED_TITLE: &str = " Received ";
pub const TRANSMITTED_TITLE: &str = " Transmitted ";
pub const TOTAL_TITLE: &str = "Total";

pub const HUNDERED_PERCENT: u64 = 100;

pub const PID_COL_LEN: u16 = 9;
pub const CPU_USAGE_COL_LEN: u16 = 9;
pub const PRETTY_BYTES_COL_LEN: u16 = 11;

pub const WELCOME_MESSAGE: &str = "Top-rs";
