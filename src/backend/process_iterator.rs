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
            return None;
        };

        self.index += 1;
        Some(Box::new(result))
    }
}
