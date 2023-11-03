use std::cell::RefMut;

use ratatui::widgets::TableState;

use super::ui_controls::table_handler::TableHandler;

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct UiHandler {
    processes: TableHandler,
}

impl UiHandler {
    pub fn new() -> UiHandler {
        UiHandler {
            processes: TableHandler::default(),
        }
    }

    pub fn next_process(&mut self) {
        self.processes.next();
    }

    pub fn previous_process(&mut self) {
        self.processes.previous();
    }

    pub fn set_process_table(&mut self, processes: Vec<Vec<String>>) {
        self.processes.set_data(processes);
    }

    pub fn get_process_table(&self) -> Vec<Vec<String>> {
        self.processes.get_data()
    }

    pub fn get_process_table_state(&self) -> RefMut<'_, TableState> {
        self.processes.get_state()
    }
}
