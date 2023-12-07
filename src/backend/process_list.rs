#![allow(dead_code)]

use super::process::{self, *};
use crate::types::{enums::sort_by::SortBy, traits::strings_line::IStringsLine};
use sysinfo::*;

#[derive(Debug, Default)]
pub struct ProcessIterator {
    processes: Box<[ProcessItem]>,
    index: usize,
}

impl ProcessIterator {
    pub fn new(sys: &System, sort_by: SortBy) -> Self {
        Self {
            processes: process::boxed_processes_sorted_by(sys, sort_by),
            ..Default::default()
        }
    }
}

impl Iterator for ProcessIterator {
    type Item = Box<dyn IStringsLine>;

    fn next(&mut self) -> Option<Box<(dyn IStringsLine)>> {
        let result = if self.index < self.processes.len() {
            self.processes[self.index].clone()
        } else {
            return None
        };

        self.index += 1;
        Some(Box::new(result))
    }
}

fn test_return(sys: &System, sort_by: SortBy) -> impl Iterator<Item = Box<dyn IStringsLine>> {
    ProcessIterator::new(sys, sort_by)
}

fn test(sys: &System, sort_by: SortBy) {
    let proc_iter = ProcessIterator::new(sys, sort_by);

    for i in proc_iter {
        i.get_line();
    }
}

/* 
impl Iterator for ProcessIterator {
    type Item = ProcessItem;
    fn next(&mut self) -> Option<ProcessItem> {
        let result = if self.index < self.processes.len() {
            self.processes[self.index].clone()
        } else {
            return None
        };

        self.index += 1;
        Some(result)
    }
}
*/