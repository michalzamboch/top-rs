#![allow(dead_code)]

use ratatui::prelude::*;
use std::rc::Rc;

use super::{elemets::*, paths::*, util::*};
use crate::types::traits::app_accessor::IAppAccessor;

pub fn handle_ui(f: &mut Frame, app_handler: &dyn IAppAccessor) {
    let chunks = create_chucks(f);

    build_info_paragraph(f, chunks[0], app_handler);

    build_cpu_detail(f, chunks[1], app_handler);

    build_cpu_paragraph(f, chunks[2], app_handler);

    build_memory_details(f, chunks[3], app_handler);

    build_memory_gauges(f, chunks[4], app_handler);

    build_process_table(f, chunks[5], app_handler);
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

fn build_info_paragraph(f: &mut Frame, chunk: Rect, app_handler: &dyn IAppAccessor) {
    let info_paragraph = pc_info::get_pc_info(app_handler.get_app());
    f.render_widget(info_paragraph, chunk);
}

fn build_cpu_detail(f: &mut Frame, chunk: Rect, app_handler: &dyn IAppAccessor) {
    let cpu_detail = cpu::get_cpu_detail(app_handler.get_app());
    f.render_widget(cpu_detail, chunk);
}

fn build_cpu_paragraph(f: &mut Frame, chunk: Rect, app_handler: &dyn IAppAccessor) {
    let cpu_gauge = cpu::get_cpu_gauge(app_handler.get_app());
    f.render_widget(cpu_gauge, chunk);
}

fn build_memory_details(f: &mut Frame, chunk: Rect, app_handler: &dyn IAppAccessor) {
    let memory_details = memory::get_memory_detail(app_handler.get_app());
    let swap_details = memory::get_swap_detail(app_handler.get_app());
    let memory_chunks = create_chucks_memory(chunk);

    f.render_widget(memory_details, memory_chunks[0]);
    f.render_widget(swap_details, memory_chunks[1]);
}

fn build_memory_gauges(f: &mut Frame, chunk: Rect, app_handler: &dyn IAppAccessor) {
    let memory_gauge = memory::get_memory_gauge(app_handler.get_app());
    let swap_gauge = memory::get_swap_gauge(app_handler.get_app());
    let memory_chunks = create_chucks_memory(chunk);

    f.render_widget(memory_gauge, memory_chunks[0]);
    f.render_widget(swap_gauge, memory_chunks[1]);
}

fn build_process_table(f: &mut Frame, chunk: Rect, app_handler: &dyn IAppAccessor) {
    let processes = process::get_process_table(app_handler);
    let process_table = app_handler.get_ui().get_table_handler(PROCESSES_TABLE_ID);

    f.render_stateful_widget(processes, chunk, &mut process_table.get_state());
}
