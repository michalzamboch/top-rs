use std::cell::RefCell;

use ratatui::widgets::*;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UiHandler {
    pub process_table_state: RefCell<TableState>,
    pub process_table: Vec<Vec<String>>,
}

impl UiHandler {
    pub fn new() -> UiHandler {
        let mut tmp_table_state = TableState::default();
        tmp_table_state.select(Some(0));

        UiHandler {
            process_table_state: RefCell::new(tmp_table_state),
            process_table: vec![],
        }
    }

    pub fn next_process(&mut self) {
        if self.process_table.is_empty() {
            return;
        }

        let i = match self.process_table_state.borrow().selected() {
            Some(i) => {
                if i >= self.process_table.len() - 1 {
                    0
                } else {
                    i + 1
                }
            }
            None => 0,
        };
        self.process_table_state.borrow_mut().select(Some(i));
    }

    pub fn previous_process(&mut self) {
        if self.process_table.is_empty() {
            return;
        }

        let i = match self.process_table_state.borrow().selected() {
            Some(i) => {
                if i == 0 {
                    self.process_table.len() - 1
                } else {
                    i - 1
                }
            }
            None => 0,
        };
        self.process_table_state.borrow_mut().select(Some(i));
    }

    pub fn set_process_table(&mut self, processes: Vec<Vec<String>>) {
        self.process_table = processes;
    }
}
