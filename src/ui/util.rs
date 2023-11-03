#![allow(dead_code)]

use crossterm::terminal::*;
use ratatui::style::Color;

use super::config;

pub fn get_terminal_height() -> usize {
    let default_height: usize = 100;

    match size() {
        Ok((_, rows)) => rows as usize,
        Err(_) => default_height,
    }
}

pub fn get_terminal_width() -> usize {
    let default_width: usize = 500;

    match size() {
        Ok((columns, _)) => columns as usize,
        Err(_) => default_width,
    }
}

pub const fn get_usage_color(usage: u16, regular_color: Color) -> Color {
    if usage >= 95 {
        config::OVERLOAD_COLOR
    } else {
        regular_color
    }
}
