use std::{cmp::*, rc::Rc};

use ratatui::{prelude::*, widgets::*};
use rayon::prelude::*;

use crate::backend::app::App;

use super::{config, util::*};

pub fn ui(f: &mut Frame, app: &App) {
    let chunks = create_chucks(f);

    let info_paragraph = get_pc_info(app);
    f.render_widget(info_paragraph, chunks[0]);

    let cpu_detail = get_cpu_detail(app);
    f.render_widget(cpu_detail, chunks[1]);

    let cpu_gauge = get_cpu_gauge(app);
    f.render_widget(cpu_gauge, chunks[2]);

    let memory_datails = get_memory_detail(app);
    f.render_widget(memory_datails, chunks[3]);

    let memory_gauge = get_memory_gauge(app);
    f.render_widget(memory_gauge, chunks[4]);

    let processes = get_processes_paragraph(app);
    f.render_widget(processes, chunks[5]);
}

fn create_chucks(f: &mut Frame) -> Rc<[Rect]> {
    Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(1),
            Constraint::Length(1),
            Constraint::Length(3),
            Constraint::Length(1),
            Constraint::Length(3),
            Constraint::Length(get_terminal_height() as u16),
            Constraint::Length(1),
        ])
        .split(f.size())
}

fn get_pc_info(app: &App) -> Paragraph<'static> {
    let text = app.get_sys_info();

    Paragraph::new(text).wrap(Wrap { trim: true })
}

fn get_memory_detail(app: &App) -> Paragraph<'static> {
    let text = app.get_memory_details();

    Paragraph::new(text)
        .wrap(Wrap { trim: true })
        .alignment(Alignment::Right)
        .style(Style::default().fg(config::MEMORY_COLOR))
}

fn get_memory_gauge(app: &App) -> Gauge<'_> {
    let usage = min(app.get_memory_usage(), config::HUNDERED_PERCENT) as u16;
    let color = cpu_usage_color(usage, config::MEMORY_COLOR);

    Gauge::default()
        .block(
            Block::default()
                .title(config::MEM_USAGE_TITLE)
                .borders(Borders::ALL),
        )
        .gauge_style(Style::default().fg(color))
        .percent(usage)
}

fn get_cpu_gauge(app: &App) -> Gauge<'_> {
    let usage = min(app.get_total_cpu_usage(), config::HUNDERED_PERCENT) as u16;
    let color = cpu_usage_color(usage, config::CPU_COLOR);

    Gauge::default()
        .block(
            Block::default()
                .title(config::CPU_USAGE_TITLE)
                .borders(Borders::ALL),
        )
        .gauge_style(Style::default().fg(color))
        .percent(usage)
}

fn get_cpu_detail(app: &App) -> Paragraph<'_> {
    let text = app.get_cpu_details();

    Paragraph::new(text)
        .wrap(Wrap { trim: true })
        .alignment(Alignment::Right)
        .style(Style::default().fg(config::CPU_COLOR))
}

fn cpu_usage_color(usage: u16, regular_color: Color) -> Color {
    if usage >= 95 {
        config::OVERLOAD_COLOR
    } else {
        regular_color
    }
}

fn get_processes_paragraph(app: &App) -> Paragraph<'_> {
    let block = Block::default().borders(Borders::TOP).title(Span::styled(
        config::PROCESSES_TITLE,
        Style::default().add_modifier(Modifier::BOLD),
    ));

    let text = get_processes_list(app);

    Paragraph::new(text).block(block)
}

fn get_processes_list(app: &App) -> Vec<Line<'_>> {
    let max_count = get_terminal_height();

    app.get_filtered_processes_vec(max_count)
        .par_iter()
        .map(|item| Line::from(item.clone()))
        .collect()
}
