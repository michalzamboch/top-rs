#![allow(dead_code)]

use rayon::prelude::*;
use sysinfo::*;

use crate::types::{enums::sort_by::SortBy, traits::table_data_holder::ITableDataHolder};

use super::process::{self, ProcessItem};

#[derive(Debug, Default, Clone)]
pub struct ProcessDataHolder {
    data: Box<[ProcessItem]>,
}

impl ProcessDataHolder {
    pub fn new(sys: &System, sort_by: SortBy) -> Self {
        Self {
            data: process::boxed_processes_sorted_by(sys, sort_by),
        }
    }

    pub fn new_box(sys: &System, sort_by: SortBy) -> Box<Self> {
        Box::new(Self::new(sys, sort_by))
    }
}

impl ITableDataHolder for ProcessDataHolder {
    fn get_data(&self) -> Vec<Vec<String>> {
        self.data
            .par_iter()
            .map(process::process_into_string_vec)
            .collect()
    }

    fn get_box(&self) -> Box<[Vec<String>]> {
        self.data
            .par_iter()
            .map(process::process_into_string_vec)
            .collect()
    }
}
