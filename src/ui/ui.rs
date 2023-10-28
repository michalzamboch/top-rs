use std::rc::Rc;

use ratatui::{prelude::*, widgets::*};

use crate::backend::app::App;

pub fn ui(f: &mut Frame, app: &App) {
    let chunks = create_chucks(f);

    let info_paragraph = get_pc_info(app);
    f.render_widget(info_paragraph, chunks[0]);

    let cpu_gauge = get_cpu_gauge(app);
    f.render_widget(cpu_gauge, chunks[1]);

    let memory_gauge = get_memory_gauge(app);
    f.render_widget(memory_gauge, chunks[2]);

    let processes = get_processes_paragraph(app, " Processes ");
    f.render_widget(processes, chunks[3]);
}

fn create_chucks(f: &mut Frame) -> Rc<[Rect]> {
    Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(2),
            Constraint::Length(3),
            Constraint::Length(3),
            Constraint::Percentage(75),
            Constraint::Length(1),
        ])
        .split(f.size())
}

fn get_pc_info(app: &App) -> Paragraph<'static> {
    let text = app.get_sys_info();
    Paragraph::new(text.cyan()).wrap(Wrap { trim: true })
}

fn get_memory_gauge(app: &App) -> Gauge<'_> {
    let color = cpu_usage_color(app, Color::Blue);

    Gauge::default()
        .block(
            Block::default()
                .title(" Memory usage ")
                .borders(Borders::ALL),
        )
        .gauge_style(Style::default().fg(color))
        .percent(app.get_memory_usage() as u16)
}

fn get_cpu_gauge(app: &App) -> Gauge<'_> {
    let color = cpu_usage_color(app, Color::Green);

    Gauge::default()
        .block(Block::default().title(" CPU usage ").borders(Borders::ALL))
        .gauge_style(Style::default().fg(color))
        .percent(app.get_total_cpu_usage() as u16)
}

fn cpu_usage_color(app: &App, regular_color: Color) -> Color {
    if app.get_total_cpu_usage() >= 95 {
        return Color::Red;
    } else {
        return regular_color;
    }
}

fn get_processes_paragraph<'a>(app: &'a App, title: &'a str) -> Paragraph<'a> {
    let block = Block::default()
        .borders(Borders::ALL)
        .style(Style::default().fg(Color::Gray))
        .title(Span::styled(
            title,
            Style::default().add_modifier(Modifier::BOLD),
        ));

    let text = get_processes_list(app);

    Paragraph::new(text.clone())
        .style(Style::default().fg(Color::Gray))
        .block(block)
}

fn get_processes_list(app: &App) -> Vec<Line<'_>> {
    let result: Vec<Line<'_>> = app
        .get_processes_vec()
        .iter()
        .map(|item| Line::from(item.clone()))
        .collect();

    result
}
