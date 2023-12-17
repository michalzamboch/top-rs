#![allow(dead_code)]

use ratatui::widgets::*;

use crate::types::traits::app_accessor::IAppAccessor;

pub fn get_status_bar(app_handler: &dyn IAppAccessor) -> Paragraph<'_> {
    let status = app_handler.get_ui().get_status();
    let msg = status.get();

    Paragraph::new(msg).wrap(Wrap { trim: true })
}
