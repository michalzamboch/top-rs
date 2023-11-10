#![allow(dead_code)]

use ratatui::widgets::*;
use std::cell::{RefCell, RefMut};

use crate::types::traits::table_handler::ITableHandler;

#[derive(Debug, PartialEq, Eq, Default)]
pub struct TableHandler {
    state: RefCell<TableState>,
    data: Vec<Vec<String>>,
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
}

impl ITableHandler for TableHandler {
    fn first(&mut self) {
        if self.data.is_empty() {
            return;
        }
        self.state.borrow_mut().select(Some(0));
    }

    fn last(&mut self) {
        if self.data.is_empty() {
            return;
        }
        let last_pos = self.data.len() - 1;
        self.state.borrow_mut().select(Some(last_pos));
    }

    fn next(&mut self) {
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

    fn previous(&mut self) {
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

    fn set_data(&mut self, data: Vec<Vec<String>>) {
        self.data = data
    }

    fn get_data(&self) -> Vec<Vec<String>> {
        self.data.clone()
    }

    fn get_state(&self) -> RefMut<'_, TableState> {
        self.state.borrow_mut()
    }
}
