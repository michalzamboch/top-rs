#![allow(dead_code)]

use ratatui::widgets::*;
use std::cell::{RefCell, RefMut};

use crate::types::traits::table_handler::ITableHandler;

#[derive(Debug, PartialEq, Eq, Default)]
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
}

impl ITableHandler for TableHandler {
    fn first(&self) {
        if self.data.borrow().is_empty() {
            return;
        }
        self.state.borrow_mut().select(Some(0));
    }

    fn last(&self) {
        if self.data.borrow().is_empty() {
            return;
        }
        let last_pos = self.data.borrow().len() - 1;
        self.state.borrow_mut().select(Some(last_pos));
    }

    fn next(&self) {
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

    fn previous(&self) {
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

    fn set_data(&self, data: Vec<Vec<String>>) {
        *self.data.borrow_mut() = data;
    }

    fn get_data(&self) -> Vec<Vec<String>> {
        self.data.borrow().clone()
    }

    fn get_state(&self) -> RefMut<'_, TableState> {
        self.state.borrow_mut()
    }
}
