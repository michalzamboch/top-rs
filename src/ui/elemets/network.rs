use ratatui::{prelude::*, widgets::*};

use crate::ui::config;

pub fn get_receive_sparkline(data: &[u64]) -> Sparkline<'_> {
    create_spark_line(data, config::RECEIVED_TITLE)
}

pub fn get_transmited_sparkline(data: &[u64]) -> Sparkline<'_> {
    create_spark_line(data, config::TRANSMITTED_TITLE)
}

fn create_spark_line<'a>(data: &'a [u64], title: &'a str) -> Sparkline<'a> {
    Sparkline::default()
        .block(
            Block::default()
                .title(title)
                .borders(Borders::ALL)
                .title_alignment(Alignment::Center),
        )
        .data(data)
        .style(Style::default().fg(config::NETWORK_COLOR))
}

pub fn get_connection_total(data: String) -> Paragraph<'static> {
    Paragraph::new(data).wrap(Wrap { trim: true }).block(
        Block::default()
            .borders(Borders::ALL)
            .title(config::TOTAL_TITLE),
    )
}
