#![allow(dead_code)]

use chrono::{DateTime, Utc};
use ratatui::{
    layout::Alignment::*,
    style::{Modifier, Style},
    widgets::*,
};

use crate::types::traits::app_accessor::IAppAccessor;

pub fn get_message_bar(app_handler: &dyn IAppAccessor) -> Paragraph<'_> {
    let status = app_handler.get_ui().get_status();
    let msg = status.get();
    let style = get_bar_style();

    Paragraph::new(msg)
        .wrap(Wrap { trim: true })
        .style(style)
        .alignment(Left)
}

pub fn get_load_bar(app_handler: &dyn IAppAccessor) -> Paragraph<'_> {
    let msg = app_handler.get_app().get_sys_load();
    let style = get_bar_style();

    Paragraph::new(msg)
        .wrap(Wrap { trim: true })
        .style(style)
        .alignment(Center)
}

pub fn get_time_bar() -> Paragraph<'static> {
    let now: DateTime<Utc> = Utc::now();
    let msg = format!("{}", now.format("%H:%M %d/%m/%Y"));
    let style = get_bar_style();

    Paragraph::new(msg)
        .wrap(Wrap { trim: true })
        .style(style)
        .alignment(Right)
}

fn get_bar_style() -> Style {
    Style::default()
        .add_modifier(Modifier::ITALIC)
        .add_modifier(Modifier::DIM)
}
