use std::{cmp::*, rc::Rc};

use ratatui::{prelude::*, widgets::*};
use rayon::prelude::*;

use crate::{backend::app::App, types::app_trait::*};

use super::{app_handler::AppHandler, config, util::*};

pub fn handle_ui(f: &mut Frame, app_handler: &AppHandler) {
    let chunks = create_chucks(f);

    let info_paragraph = get_pc_info(&app_handler.app);
    f.render_widget(info_paragraph, chunks[0]);

    let cpu_detail = get_cpu_detail(&app_handler.app);
    f.render_widget(cpu_detail, chunks[1]);

    let cpu_gauge = get_cpu_gauge(&app_handler.app);
    f.render_widget(cpu_gauge, chunks[2]);

    let memory_datails = get_memory_detail(&app_handler.app);
    f.render_widget(memory_datails, chunks[3]);

    let memory_gauge = get_memory_gauge(&app_handler.app);
    f.render_widget(memory_gauge, chunks[4]);

    let processes = get_process_table(app_handler);
    f.render_stateful_widget(
        processes,
        chunks[5],
        &mut app_handler.ui.process_table_state.borrow_mut(),
    );
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
            Constraint::Max(get_terminal_height() as u16),
        ])
        .split(f.size())
}

fn get_pc_info(app: &App) -> Paragraph<'_> {
    let text = app.get_sys_info();

    Paragraph::new(text).wrap(Wrap { trim: true })
}

fn get_memory_detail(app: &App) -> Paragraph<'_> {
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

fn get_process_table(app_handler: &AppHandler) -> Table<'_> {
    let rows = get_process_rows(app_handler);
    let header = get_process_header();
    let selected_style = Style::default().add_modifier(Modifier::REVERSED);

    Table::new(rows)
        .header(header)
        .highlight_style(selected_style)
        .widths(&[
            Constraint::Ratio(1, 12),
            Constraint::Ratio(4, 12),
            Constraint::Ratio(1, 12),
            Constraint::Ratio(2, 12),
            Constraint::Ratio(2, 12),
            Constraint::Ratio(2, 12),
        ])
}

fn get_process_rows(app_handler: &AppHandler) -> impl Iterator<Item = Row<'_>> {
    app_handler.ui.process_table.iter().map(|item| {
        let height = item
            .par_iter()
            .map(|content| content.chars().filter(|c| *c == '\n').count())
            .max()
            .unwrap_or(0)
            + 1;
        let cells = item.iter().map(|c| Cell::from(c.clone()));
        Row::new(cells).height(height as u16)
    })
}

fn get_process_header() -> Row<'static> {
    let normal_style = Style::default().bg(Color::Blue);

    let header_cells = [
        "Pid [P]",
        "Process [N]",
        "Cpu [C]",
        "Memory [M]",
        "Read [R]",
        "Write [W]",
    ]
    .iter()
    .map(|h| Cell::from(*h));

    Row::new(header_cells).style(normal_style).height(1)
}
