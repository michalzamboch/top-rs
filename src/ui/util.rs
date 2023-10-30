use crossterm::terminal::*;

pub fn get_terminal_height() -> usize {
    let default_height: usize = 100;

    match size() {
        Ok((_, rows)) => rows as usize,
        Err(_) => default_height,
    }
}
