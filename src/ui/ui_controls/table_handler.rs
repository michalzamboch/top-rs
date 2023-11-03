#![allow(dead_code)]

use std::cell::RefCell;

use ratatui::widgets::*;

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct TableHandler {
    pub state: RefCell<TableState>,
    pub data: Vec<Vec<String>>,
}

impl TableHandler {
    pub fn new() -> TableHandler {
        let mut tmp_state = TableState::default();
        tmp_state.select(Some(0));

        TableHandler {
            state: RefCell::new(tmp_state),
            data: vec![],
        }
    }

    pub fn next(&mut self) {
        if self.data.is_empty() {
            return;
        }

        let i = match self.state.borrow().selected() {
            Some(i) => {
                if i >= self.data.len() - 1 {
                    0
                } else {
                    i + 1
                }
            }
            None => 0,
        };
        self.state.borrow_mut().select(Some(i));
    }

    pub fn previous(&mut self) {
        if self.data.is_empty() {
            return;
        }

        let i = match self.state.borrow().selected() {
            Some(i) => {
                if i == 0 {
                    self.data.len() - 1
                } else {
                    i - 1
                }
            }
            None => 0,
        };
        self.state.borrow_mut().select(Some(i));
    }

    pub fn set_data(&mut self, processes: Vec<Vec<String>>) {
        self.data = processes;
    }
}
