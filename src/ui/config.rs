use crossterm::event::KeyCode;
use ratatui::prelude::*;

pub const REFRESH_MILIS: u64 = 750;

pub static EXIT_KEY_CODES: &[KeyCode] = &[KeyCode::Char('q'), KeyCode::Esc];
pub static REFRESH_KEY_CODES: &[KeyCode] = &[KeyCode::Char('r'), KeyCode::F(5)];

pub const MEMORY_COLOR: Color = Color::Blue;
pub const CPU_COLOR: Color = Color::Green;
pub const OVERLOAD_COLOR: Color = Color::Red;
