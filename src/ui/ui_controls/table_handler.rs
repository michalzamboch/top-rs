#![allow(dead_code)]

use std::cell::{RefCell, RefMut};

use ratatui::widgets::*;

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct TableHandler {
    state: RefCell<TableState>,
    data: RefCell<Vec<Vec<String>>>,
}

impl TableHandler {
    pub fn new() -> TableHandler {
        let mut tmp_state = TableState::default();
        tmp_state.select(Some(0));

        TableHandler {
            state: RefCell::new(tmp_state),
            data: RefCell::new(vec![]),
        }
    }

    pub fn next(&mut self) {
        if self.data.borrow().is_empty() {
            return;
        }

        let i = match self.state.borrow().selected() {
            Some(i) => {
                if i >= self.data.borrow().len() - 1 {
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
        if self.data.borrow().is_empty() {
            return;
        }

        let i = match self.state.borrow().selected() {
            Some(i) => {
                if i == 0 {
                    self.data.borrow().len() - 1
                } else {
                    i - 1
                }
            }
            None => 0,
        };
        self.state.borrow_mut().select(Some(i));
    }

    pub fn set_data(&mut self, processes: Vec<Vec<String>>) {
        self.data.borrow_mut().clone_from(&processes);
    }

    pub fn get_data(&self) -> RefMut<'_, Vec<Vec<String>>> {
        self.data.borrow_mut()
    }

    pub fn get_state(&self) -> RefMut<'_, TableState> {
        self.state.borrow_mut()
    }

}
