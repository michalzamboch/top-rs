use crossterm::event::KeyCode;

pub const REFRESH_MILIS: u64 = 750;
pub static EXIT_KEY_CODES: &[KeyCode] = &[KeyCode::Char('q'), KeyCode::Esc];
pub static REFRESH_KEY_CODES: &[KeyCode] = &[KeyCode::Char('r'), KeyCode::F(5)];
