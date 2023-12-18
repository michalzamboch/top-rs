#![allow(dead_code)]

use std::{
    cell::RefCell,
    time::{Duration, Instant},
};

#[derive(Debug)]
pub struct SystemItem {
    duration: Duration,
    last_tick: RefCell<Instant>,
}

impl SystemItem {
    pub fn new(millis: u64) -> Self {
        Self {
            duration: Duration::from_secs(millis),
            last_tick: RefCell::new(Instant::now()),
        }
    }

    pub fn update(&self) -> bool {
        if self.last_tick.borrow().elapsed() >= self.duration {
            *self.last_tick.borrow_mut() = Instant::now();

            return true;
        }

        false
    }
}
