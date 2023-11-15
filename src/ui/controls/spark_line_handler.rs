use std::{cell::RefCell, collections::VecDeque};

use crate::{types::traits::{creatable::ICreatable, spark_line_handler::ISparkLineHandler}, ui::util::*};

#[derive(Debug, PartialEq, Eq, Default)]
pub struct SparkLineHandler {
    data: RefCell<VecDeque<u64>>,
}

impl ICreatable for SparkLineHandler {
    fn new() -> Self {
        Self {
            data: RefCell::new(VecDeque::new()),
        }
    }
}

impl ISparkLineHandler for SparkLineHandler {
    fn add(&self, value: u64) {
        self.data.borrow_mut().push_front(value);
        self.resize();
    }

    fn resize(&self) {
        let new_size = get_spark_line_len() as usize;
        self.data.borrow_mut().resize(new_size, 0);
    }

    fn get_vec(&self) -> Vec<u64> {
        self.data.borrow().iter().cloned().collect()
    }

    fn init(&self, size: u64) {
        self.data.borrow_mut().clear();
        for _ in 0..size {
            self.data.borrow_mut().push_back(0);
        }
    }
}
