#![allow(dead_code)]

use std::{
    cell::RefCell,
    rc::Rc,
    time::{Duration, Instant},
};

#[derive(Debug)]
pub struct SystemItem<T> {
    duration: Duration,
    last_tick: RefCell<Instant>,
    value: RefCell<Rc<T>>,
}

impl<T> SystemItem<T> {
    pub fn update(&self, new_value: T) {
        if self.last_tick.borrow().elapsed() >= self.duration {
            *self.last_tick.borrow_mut() = Instant::now();
            *self.value.borrow_mut() = Rc::new(new_value);
        }
    }

    pub fn get_value(&self) -> Rc<T> {
        self.value.borrow().clone()
    }
}
