use std::cell::RefCell;

use crate::types::traits::{creatable::ICreatable, status_handler::IStatusHandler};

#[derive(Debug, Default)]
pub struct StatusHandler {
    message: RefCell<String>,
}

impl ICreatable for StatusHandler {
    fn new() -> Self {
        Self {
            message: RefCell::new("".to_owned()),
        }
    }
}

impl IStatusHandler for StatusHandler {
    fn set(&self, message: &str) {
        let string_msg = message.to_owned();
        *self.message.borrow_mut() = string_msg;
    }

    fn get(&self) -> String {
        self.message.borrow().clone()
    }

    fn clear(&self) {
        self.set("");
    }
}
