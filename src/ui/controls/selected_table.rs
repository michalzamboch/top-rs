use std::cell::RefCell;

use crate::types::traits::{creatable::ICreatable, selected_table::ISelectedTable};

#[derive(Debug, PartialEq, Eq, Default)]
pub struct SelectedTable {
    selected: RefCell<usize>,
    ids: RefCell<Vec<String>>,
}

impl ICreatable for SelectedTable {
    fn new() -> Self {
        Self {
            selected: RefCell::new(0),
            ids: RefCell::new(Vec::new()),
        }
    }
}

impl ISelectedTable for SelectedTable {
    fn get(&self) -> String {
        let borrow = *self.selected.borrow();
        self.ids.borrow()[borrow].clone()
    }

    fn register_vec(&self, ids: Vec<&str>) {
        for id in ids {
            self.ids.borrow_mut().push(id.to_owned());
        }
    }

    fn register(&self, name: &str) {
        self.ids.borrow_mut().push(name.to_owned());
    }

    fn next(&self) {
        *self.selected.borrow_mut() += 1;
        if *self.selected.borrow() >= self.ids.borrow().len() {
            *self.selected.borrow_mut() = 0;
        }
    }

    fn previous(&self) {
        if *self.selected.borrow() == 0 {
            let last_pos = self.ids.borrow().len() - 1;
            *self.selected.borrow_mut() = last_pos;
        } else {
            *self.selected.borrow_mut() -= 1;
        }
    }
}
