#![allow(dead_code)]

use pretty_bytes::*;
use ratatui::{prelude::*, widgets::*};

use crate::ui::config;

pub fn get_receive_sparkline(data: &Vec<u64>) -> Sparkline<'_> {
    create_spark_line(data.as_slice(), " Received ".to_string())
}

pub fn get_transmited_sparkline(data: &Vec<u64>) -> Sparkline<'_> {
    create_spark_line(data.as_slice(), " Transmitted ".to_string())
}

fn create_spark_line(data: &[u64], title: String) -> Sparkline<'_> {
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

pub fn get_connection_detail_num(num: u64) -> Paragraph<'static> {
    let data = converter::convert(num as f64);
    get_connection_total(data)
}

pub fn get_connection_total(data: String) -> Paragraph<'static> {
    Paragraph::new(data)
        .wrap(Wrap { trim: true })
        .block(Block::default().borders(Borders::ALL).title("Total"))
}
