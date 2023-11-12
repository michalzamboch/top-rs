#![allow(dead_code)]

use ratatui::{prelude::*, widgets::*};
use std::{cmp::*, rc::Rc};

use crate::types::traits::app::IApp;

use super::{app_handler::AppHandler, config, paths::*, util::*};

pub fn handle_ui(f: &mut Frame, app_handler: &AppHandler) {
    let chunks = create_chucks(f);

    let info_paragraph = get_pc_info(app_handler.get_app());
    f.render_widget(info_paragraph, chunks[0]);

    let cpu_detail = get_cpu_detail(app_handler.get_app());
    f.render_widget(cpu_detail, chunks[1]);

    let cpu_gauge = get_cpu_gauge(app_handler.get_app());
    f.render_widget(cpu_gauge, chunks[2]);

    let memory_details = get_memory_detail(app_handler.get_app());
    let swap_details = get_swap_detail(app_handler.get_app());
    f.render_widget(memory_details, chunks[3]);
    f.render_widget(swap_details, chunks[3]);

    let memory_gauge = get_memory_gauge(app_handler.get_app());
    let swap_gauge = get_swap_gauge(app_handler.get_app());
    let memory_chunks = create_chucks_memory(chunks[4]);

    f.render_widget(memory_gauge, memory_chunks[0]);
    f.render_widget(swap_gauge, memory_chunks[1]);

    let processes = get_process_table(app_handler);
    let process_table = app_handler.get_ui().get_table_handler(PROCESSES_TABLE_ID);

    f.render_stateful_widget(processes, chunks[5], &mut process_table.get_state());
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

fn create_chucks_memory(size: Rect) -> Rc<[Rect]> {
    Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(size)
}

fn create_main_horizontal_chunks(size: Rect) -> Rc<[Rect]> {
    Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(70), Constraint::Percentage(30)])
        .split(size)
}

fn get_pc_info(app: &dyn IApp) -> Paragraph<'_> {
    let text = app.get_sys_info();

    Paragraph::new(text).wrap(Wrap { trim: true })
}

fn get_memory_detail(app: &dyn IApp) -> Paragraph<'_> {
    let text = app.get_memory_details();

    Paragraph::new(text)
        .wrap(Wrap { trim: true })
        .alignment(Alignment::Left)
        .style(Style::default().fg(config::MEMORY_COLOR))
}

fn get_swap_detail(app: &dyn IApp) -> Paragraph<'_> {
    let text = app.get_swap_details();

    Paragraph::new(text)
        .wrap(Wrap { trim: true })
        .alignment(Alignment::Right)
        .style(Style::default().fg(config::MEMORY_COLOR))
}

fn get_memory_gauge(app: &dyn IApp) -> Gauge<'_> {
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

fn get_swap_gauge(app: &dyn IApp) -> Gauge<'_> {
    let usage = min(app.get_swap_usage(), config::HUNDERED_PERCENT) as u16;
    let color = get_usage_color(usage, config::MEMORY_COLOR);

    Gauge::default()
        .block(
            Block::default()
                .title(config::SWAP_USAGE_TITLE)
                .borders(Borders::ALL),
        )
        .gauge_style(Style::default().fg(color))
        .percent(usage)
}

fn get_cpu_gauge(app: &dyn IApp) -> Gauge<'_> {
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

fn get_cpu_detail(app: &dyn IApp) -> Paragraph<'_> {
    let text = app.get_cpu_details();

    Paragraph::new(text)
        .wrap(Wrap { trim: true })
        .alignment(Alignment::Right)
        .style(Style::default().fg(config::CPU_COLOR))
}

fn get_process_table(app_handler: &AppHandler) -> Table<'_> {
    let table = app_handler.get_ui().get_table_handler(PROCESSES_TABLE_ID);
    get_process_table_from_vec(table.get_data())
}

fn get_process_table_from_vec(data: Vec<Vec<String>>) -> Table<'static> {
    let rows = get_process_rows(&data);
    let header = get_process_header();
    let selected_style = Style::default().add_modifier(Modifier::REVERSED);

    Table::new(rows)
        .header(header)
        .highlight_style(selected_style)
        .widths(&[
            Constraint::Min(config::PID_COL_LEN),
            Constraint::Max(config::NAME_COL_MAX_LEN),
            Constraint::Min(config::CPU_USAGE_COL_LEN),
            Constraint::Min(config::PRETTY_BYTES_COL_LEN),
            Constraint::Min(config::PRETTY_BYTES_COL_LEN),
            Constraint::Min(config::PRETTY_BYTES_COL_LEN),
        ])
}

fn get_process_rows(data: &[Vec<String>]) -> impl Iterator<Item = Row<'static>> + '_ {
    data.iter().map(|item| {
        let cells = item.iter().map(|c| Cell::from(c.clone()));
        Row::new(cells)
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

fn get_disk_table(app_handler: &AppHandler) -> Table<'_> {
    let disks = app_handler.get_ui().get_table_handler(DISKS_TABLE_ID);
    get_disk_table_from_vec(disks.get_data())
}

fn get_disk_table_from_vec(data: Vec<Vec<String>>) -> Table<'static> {
    let rows = get_disk_rows(&data);
    let header = get_disks_header();
    let selected_style = Style::default().add_modifier(Modifier::REVERSED);

    Table::new(rows)
        .header(header)
        .highlight_style(selected_style)
        .widths(&[
            Constraint::Max(config::NAME_COL_MAX_LEN),
            Constraint::Min(config::PRETTY_BYTES_COL_LEN),
            Constraint::Min(config::PRETTY_BYTES_COL_LEN),
            Constraint::Min(config::PRETTY_BYTES_COL_LEN),
        ])
}

fn get_disk_rows(data: &[Vec<String>]) -> impl Iterator<Item = Row<'static>> + '_ {
    data.iter().map(|item| {
        let cells = item.iter().map(|c| Cell::from(c.clone()));
        Row::new(cells)
    })
}

fn get_disks_header() -> Row<'static> {
    let normal_style = Style::default().bg(Color::LightRed);
    let header_cells = ["Label", "Free", "Used", "Total"]
        .iter()
        .map(|h| Cell::from(*h));

    Row::new(header_cells).style(normal_style).height(1)
}
