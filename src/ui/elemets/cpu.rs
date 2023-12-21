use ratatui::{prelude::*, widgets::*};
use std::cmp::*;

use crate::{
    types::traits::app::IApp,
    ui::{config, util::get_usage_color},
};

pub fn get_cpu_gauge(app: &dyn IApp) -> Gauge<'_> {
    let usage = min(app.get_total_cpu_usage(), config::HUNDERED_PERCENT) as u16;
    let color = get_usage_color(usage, config::CPU_COLOR);

    Gauge::default()
        .block(
            Block::default()
                .title(config::CPU_USAGE_TITLE)
                .borders(Borders::ALL),
        )
        .gauge_style(Style::default().fg(color))
        .percent(usage)
}

pub fn get_cpu_detail(app: &dyn IApp) -> Paragraph<'_> {
    let text = app.get_cpu_details();

    Paragraph::new(text)
        .wrap(Wrap { trim: true })
        .alignment(Alignment::Left)
        .style(Style::default().fg(config::CPU_COLOR))
}
