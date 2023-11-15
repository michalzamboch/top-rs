#![allow(dead_code)]

use ratatui::prelude::*;
use std::rc::Rc;

use super::{config, elemets::*, paths::*, util::*};
use crate::types::traits::app_accessor::IAppAccessor;

pub fn handle_ui(f: &mut Frame, app_handler: &dyn IAppAccessor) {
    let chunks = create_chucks(f);

    build_info_paragraph(f, chunks[0], app_handler);

    build_cpu_detail(f, chunks[1], app_handler);

    build_cpu_paragraph(f, chunks[2], app_handler);

    build_memory_details(f, chunks[3], app_handler);

    build_memory_gauges(f, chunks[4], app_handler);

    build_process_table(f, chunks[5], app_handler);

    build_network_spark_lines(f, chunks[6], app_handler);
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
            Constraint::Length(3),
        ])
        .split(f.size())
}

fn create_half_chucks(size: Rect) -> Rc<[Rect]> {
    Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(size)
}

fn create_network_chucks(size: Rect) -> Rc<[Rect]> {
    let byte_len = config::PRETTY_BYTES_COL_LEN;
    let spark_line_len = get_spark_line_len();

    Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Length(byte_len),
            Constraint::Length(spark_line_len),
            Constraint::Length(byte_len),
            Constraint::Length(spark_line_len),
        ])
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
    let memory_chunks = create_half_chucks(chunk);

    f.render_widget(memory_details, memory_chunks[0]);
    f.render_widget(swap_details, memory_chunks[1]);
}

fn build_memory_gauges(f: &mut Frame, chunk: Rect, app_handler: &dyn IAppAccessor) {
    let memory_gauge = memory::get_memory_gauge(app_handler.get_app());
    let swap_gauge = memory::get_swap_gauge(app_handler.get_app());
    let memory_chunks = create_half_chucks(chunk);

    f.render_widget(memory_gauge, memory_chunks[0]);
    f.render_widget(swap_gauge, memory_chunks[1]);
}

fn build_process_table(f: &mut Frame, chunk: Rect, app_handler: &dyn IAppAccessor) {
    let processes = process::get_process_table(app_handler);
    let process_table = app_handler.get_ui().get_table_handler(PROCESSES_TABLE_ID);

    f.render_stateful_widget(processes, chunk, &mut process_table.get_state());
}

fn build_network_spark_lines(f: &mut Frame, chunk: Rect, app_handler: &dyn IAppAccessor) {
    let chunks = create_network_chucks(chunk);
    let received = app_handler.get_ui().get_spar_line(RECEIVED_SPARK_LINE_ID);
    let transmitted = app_handler.get_ui().get_spar_line(TRASMITTED_SPARK_LINE_ID);

    let connection_info = app_handler.get_app().get_current_connection_total_strings();

    let binding = received.get_vec();
    let received_spark_line = network::get_receive_sparkline(&binding);
    let received_paragraph = network::get_connection_total(connection_info.0);

    let binding = transmitted.get_vec();
    let transmitted_spark_line = network::get_transmited_sparkline(&binding);
    let trasmitted_paragraph = network::get_connection_total(connection_info.1);

    f.render_widget(received_paragraph, chunks[0]);
    f.render_widget(received_spark_line, chunks[1]);
    f.render_widget(trasmitted_paragraph, chunks[2]);
    f.render_widget(transmitted_spark_line, chunks[3]);
}
