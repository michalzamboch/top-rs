use ratatui::widgets::*;
use std::cell::{RefCell, RefMut};

use crate::types::{
    enums::table_position::TablePosition,
    traits::{creatable::ICreatable, table_handler::ITableHandler},
};

#[derive(Debug, PartialEq, Eq, Default)]
pub struct TableHandler {
    state: RefCell<TableState>,
    data: RefCell<Vec<Vec<String>>>,
}

impl ICreatable for TableHandler {
    fn new() -> TableHandler {
        let mut tmp_state = TableState::default();
        tmp_state.select(None);

        TableHandler {
            state: RefCell::new(tmp_state),
            data: RefCell::new(vec![]),
        }
    }
}

impl ITableHandler for TableHandler {
    fn jump_to(&self, position: TablePosition) {
        match position {
            TablePosition::Down => self.next(),
            TablePosition::Up => self.previous(),
            TablePosition::First => self.first(),
            TablePosition::Last => self.last(),
            TablePosition::PageDown => todo!(),
            TablePosition::PageUp => todo!(),
        }
    }

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
