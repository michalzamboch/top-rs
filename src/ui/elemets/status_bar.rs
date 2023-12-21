#![allow(dead_code)]

use ratatui::{
    style::{Modifier, Style},
    widgets::*,
};

use crate::types::traits::app_accessor::IAppAccessor;

pub fn get_status_bar(app_handler: &dyn IAppAccessor) -> Paragraph<'_> {
    let status = app_handler.get_ui().get_status();
    let msg = status.get();
    let style = Style::default()
        .add_modifier(Modifier::ITALIC)
        .add_modifier(Modifier::REVERSED);

    Paragraph::new(msg).wrap(Wrap { trim: true }).style(style)
}
