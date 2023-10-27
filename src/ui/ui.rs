use std::rc::Rc;

use ratatui::{prelude::*, widgets::*};

use crate::backend::app::App;

pub fn ui<B: Backend>(f: &mut Frame<B>, app: &App) {
    let chunks = create_chucks(f);

    let cpu_gauge = get_cpu_gauge(app);
    f.render_widget(cpu_gauge, chunks[0]);

    let memory_gauge = get_memory_gauge(app);
    f.render_widget(memory_gauge, chunks[1]);
}

fn create_chucks<B: Backend>(f: &mut Frame<B>) -> Rc<[Rect]> {
    Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage(10),
            Constraint::Percentage(10),
            Constraint::Percentage(80),
        ])
        .split(f.size())
}

fn get_memory_gauge(app: &App) -> Gauge<'_> {
    Gauge::default()
        .block(Block::default().title("Memory usage").borders(Borders::ALL))
        .gauge_style(Style::default().fg(Color::Blue))
        .percent(app.get_memory_usage() as u16)
}

fn get_cpu_gauge(app: &App) -> Gauge<'_> {
    let color = cpu_usage_color(app);

    Gauge::default()
        .block(Block::default().title("CPU usage").borders(Borders::ALL))
        .gauge_style(Style::default().fg(color))
        .percent(app.get_total_cpu_usage() as u16)
}

fn cpu_usage_color(app: &App) -> Color {
    if app.get_total_cpu_usage() >= 95 {
        return Color::Red;
    }
    else {
        return Color::Green;
    }
}
