use ratatui::{prelude::*, widgets::*};
use std::cmp::*;

use crate::{
    types::traits::app::IApp,
    ui::{config, util::*},
};

pub fn get_memory_detail(app: &dyn IApp) -> Paragraph<'_> {
    let text = app.get_memory_details();

    Paragraph::new(text)
        .wrap(Wrap { trim: true })
        .alignment(Alignment::Left)
        .style(Style::default().fg(config::MEMORY_COLOR))
}

pub fn get_swap_detail(app: &dyn IApp) -> Paragraph<'_> {
    let text = app.get_swap_details();

    Paragraph::new(text)
        .wrap(Wrap { trim: true })
        .alignment(Alignment::Right)
        .style(Style::default().fg(config::SWAP_COLOR))
}

pub fn get_memory_gauge(app: &dyn IApp) -> Gauge<'_> {
    let usage = min(app.get_memory_usage(), config::HUNDERED_PERCENT) as u16;
    let color = get_usage_color(usage, config::MEMORY_COLOR);

    Gauge::default()
        .block(
            Block::default()
                .title(config::MEM_USAGE_TITLE)
                .borders(Borders::ALL),
        )
        .gauge_style(Style::default().fg(color))
        .percent(usage)
}

pub fn get_swap_gauge(app: &dyn IApp) -> Gauge<'_> {
    let usage = min(app.get_swap_usage(), config::HUNDERED_PERCENT) as u16;
    let color = get_usage_color(usage, config::SWAP_COLOR);

    Gauge::default()
        .block(
            Block::default()
                .title(config::SWAP_USAGE_TITLE)
                .borders(Borders::ALL),
        )
        .gauge_style(Style::default().fg(color))
        .percent(usage)
}
