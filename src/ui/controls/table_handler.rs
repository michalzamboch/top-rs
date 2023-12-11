use fast_str::FastStr;
use ratatui::widgets::*;
use std::{
    cell::{RefCell, RefMut},
    ops::Deref,
    rc::Rc,
};

use crate::{
    backend::process_data_holder::ProcessDataHolder,
    types::{
        enums::table_position::TablePosition,
        traits::{table_data_holder::ITableDataHolder, table_handler::ITableHandler},
    },
};

#[derive(Debug)]
pub struct TableHandler {
    state: RefCell<TableState>,
    data: RefCell<Option<Box<dyn ITableDataHolder>>>,
}

impl TableHandler {
    fn len(&self) -> usize {
        self.data
            .borrow()
            .as_ref()
            .unwrap_or(&ProcessDataHolder::new_empty_dyn_box())
            .len()
    }

    fn is_empty(&self) -> bool {
        self.data
            .borrow()
            .as_ref()
            .unwrap_or(&ProcessDataHolder::new_empty_dyn_box())
            .is_empty()
    }

    fn new() -> TableHandler {
        let mut tmp_state = TableState::default();
        tmp_state.select(None);

        TableHandler {
            state: RefCell::new(tmp_state),
            data: RefCell::new(None),
        }
    }

    pub fn new_rc() -> Rc<TableHandler> {
        Rc::new(TableHandler::new())
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
        if self.is_empty() {
            return;
        }
        self.state.borrow_mut().select(Some(0));
    }

    fn last(&self) {
        if self.is_empty() {
            return;
        }
        let last_pos = self.len() - 1;
        self.state.borrow_mut().select(Some(last_pos));
    }

    fn next(&self) {
        if self.is_empty() {
            return;
        }

        let i = match self.state.borrow().selected() {
            Some(i) => {
                if i >= self.len() - 1 {
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
        if self.is_empty() {
            return;
        }

        let i = match self.state.borrow().selected() {
            Some(i) => {
                if i == 0 {
                    self.len() - 1
                } else {
                    i - 1
                }
            }
            None => 0,
        };
        self.state.borrow_mut().select(Some(i));
    }

    fn set_data(&self, data: Box<dyn ITableDataHolder>) {
        *self.data.borrow_mut() = Some(data);
    }

    fn get_box(&self) -> Box<[Vec<String>]> {
        self.data
            .borrow()
            .deref()
            .as_ref()
            .unwrap_or(&ProcessDataHolder::new_empty_dyn_box())
            .get_box()
    }

    fn get_fbox(&self) -> Box<[Vec<FastStr>]> {
        self.data
            .borrow()
            .deref()
            .as_ref()
            .unwrap_or(&ProcessDataHolder::new_empty_dyn_box())
            .get_fbox()
    }

    fn get_state(&self) -> RefMut<'_, TableState> {
        self.state.borrow_mut()
    }
}
