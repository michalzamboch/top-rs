use rayon::iter::*;
use std::{cell::RefCell, collections::VecDeque};

use crate::types::traits::{creatable::ICreatable, spark_line_handler::ISparkLineHandler};

#[derive(Debug, PartialEq, Eq, Default)]
pub struct SparkLineHandler {
    data: RefCell<VecDeque<u64>>,
}

impl ICreatable for SparkLineHandler {
    fn new() -> Self {
        Self {
            data: RefCell::new(VecDeque::from([
                1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 20, 30, 40, 50, 60, 70, 80, 100,
            ])),
        }
    }
}

impl ISparkLineHandler for SparkLineHandler {
    fn add(&self, value: u64) {
        self.data.borrow_mut().push_back(value);
        self.data.borrow_mut().pop_front();
    }

    fn get_vec(&self) -> Vec<u64> {
        self.data.borrow().par_iter().cloned().collect()
    }
}
