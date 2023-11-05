#![allow(dead_code)]

use ratatui::widgets::TableState;
use std::cell::RefMut;

use super::ui_controls::table_handler::TableHandler;
use crate::types::{ui_handler_trait::IUiHandler, table_handler_trait::ITableHandler};

#[derive(Debug)]
pub struct UiHandler {
    processes: Box<dyn ITableHandler>,
    temperatures: Box<dyn ITableHandler>,
}

impl UiHandler {
    pub fn new() -> UiHandler {
        UiHandler {
            processes: Box::<TableHandler>::default(),
            temperatures: Box::<TableHandler>::default(),
        }
    }
}

impl IUiHandler for UiHandler {
    fn next_process(&mut self) {
        self.processes.next();
    }

    fn previous_process(&mut self) {
        self.processes.previous();
    }

    fn first_process(&mut self) {
        self.processes.first();
    }

    fn last_process(&mut self) {
        self.processes.last();
    }

    fn set_process_table(&mut self, processes: Vec<Vec<String>>) {
        self.processes.set_data(processes);
    }

    fn get_process_table(&self) -> Vec<Vec<String>> {
        self.processes.get_data()
    }

    fn get_process_table_state(&self) -> RefMut<'_, TableState> {
        self.processes.get_state()
    }
}
