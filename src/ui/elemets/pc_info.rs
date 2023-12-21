use ratatui::{layout::Alignment::Right, widgets::*};

use crate::types::traits::app::IApp;

pub fn get_pc_info(app: &dyn IApp) -> Paragraph<'_> {
    let text = app.get_sys_info();

    Paragraph::new(text)
        .wrap(Wrap { trim: true })
        .alignment(Right)
}
