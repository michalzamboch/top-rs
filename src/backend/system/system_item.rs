#![allow(dead_code)]

use std::{cell::RefCell, rc::Rc, time::*};

#[derive(Debug)]
pub struct SystemItem<T> {
    duration: Duration,
    last_tick: RefCell<Instant>,
    item: RefCell<Rc<T>>,
}

impl<T> SystemItem<T> {
    pub fn new(millis: u64, value_getter: &dyn Fn() -> T) -> Self {
        let value = value_getter();

        Self {
            duration: Duration::from_secs(millis),
            last_tick: RefCell::new(Instant::now()),
            item: RefCell::new(Rc::new(value)),
        }
    }

    pub fn update(&self, value_getter: &dyn Fn() -> T) {
        if self.time_to_update() {
            self.set(value_getter);
            *self.last_tick.borrow_mut() = Instant::now();
        }
    }

    pub fn get(&self) -> Rc<T> {
        self.item.borrow().clone()
    }

    fn time_to_update(&self) -> bool {
        self.last_tick.borrow().elapsed() >= self.duration
    }

    fn set(&self, value_getter: &dyn Fn() -> T) {
        let value = value_getter();
        *self.item.borrow_mut() = Rc::new(value);
    }
}
