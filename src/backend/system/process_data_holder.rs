use fast_str::FastStr;
use rayon::prelude::*;
use std::error::Error;
use sysinfo::*;

use crate::types::{
    enums::{
        sort_by::SortBy,
        table_commands::{ProcessCommand::KillProcess, TableCommand},
    },
    traits::{process::IProcessStringView, table_data_holder::ITableDataHolder},
};

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

    pub fn new_empty_box() -> Box<Self> {
        Box::new(Self { data: Box::new([]) })
    }

    pub fn new_empty_dyn_box() -> Box<dyn ITableDataHolder> {
        Self::new_empty_box()
    }
}

impl ITableDataHolder for ProcessDataHolder {
    fn get_data(&self) -> Box<[Vec<FastStr>]> {
        self.data
            .par_iter()
            .map(|item| item.into_fstring_vec())
            .collect()
    }

    fn execute(&self, command: TableCommand, index: usize) -> Result<(), Box<dyn Error>> {
        match command {
            TableCommand::Process(process_cmd) => {
                let result = match process_cmd {
                    KillProcess => self.data[index].kill(),
                };

                match result {
                    true => Ok(()),
                    false => Err("Failed to kill process".into()),
                }
            }
        }
    }

    fn len(&self) -> usize {
        self.data.len()
    }

    fn is_empty(&self) -> bool {
        self.data.is_empty()
    }
}
