use crossterm::event::KeyCode;
use ratatui::prelude::*;

pub const REFRESH_MILIS: u64 = 750;

pub static EXIT_KEY_CODES: &[KeyCode] = &[KeyCode::Char('q'), KeyCode::Esc];

pub const MEMORY_COLOR: Color = Color::Blue;
pub const CPU_COLOR: Color = Color::Green;
pub const OVERLOAD_COLOR: Color = Color::Red;

pub const PROCESSES_TITLE: &str = "Processes ";
pub const CPU_USAGE_TITLE: &str = " CPU usage ";
pub const MEM_USAGE_TITLE: &str = " Memory usage ";

pub const HUNDERED_PERCENT: u64 = 100;