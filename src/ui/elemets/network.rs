#![allow(dead_code)]

use ratatui::{prelude::*, widgets::*};

pub fn get_receive_sparkline(data: &Vec<u64>) -> Sparkline<'_> {
    create_spark_line(data.as_slice(), " Received ".to_string())
}

pub fn get_transmited_sparkline(data: &Vec<u64>) -> Sparkline<'_> {
    create_spark_line(data.as_slice(), " Transmitted ".to_string())
}

fn create_spark_line(data: &[u64], title: String) -> Sparkline<'_> {
    Sparkline::default()
        .block(Block::default().title(title).borders(Borders::ALL))
        .data(data)
        .style(Style::default().fg(Color::Green))
}
