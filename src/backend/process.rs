#![allow(dead_code)]

use pretty_bytes::converter;
use rayon::prelude::*;
use std::{cmp::Reverse, sync::Arc};
use sysinfo::*;

use crate::types::{enums::sort_by::SortBy, traits::process::IProcessStringView};

#[derive(Debug, Hash, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub struct ProcessItem {
    pid: Pid,
    name: String,
    cpu_usage: u64,
    memory_usage: u64,
    disk_read_usage: u64,
    disk_write_usage: u64,
}

impl IProcessStringView for ProcessItem {
    fn get_pid(&self) -> String {
        format!("{}", self.pid)
    }

    fn get_name(&self) -> String {
        self.name.clone()
    }

    fn get_cpu_usage(&self) -> String {
        format!("{} %", self.cpu_usage as f32 / 10.0)
    }

    fn get_memory_usage(&self) -> String {
        converter::convert(self.memory_usage as f64)
    }

    fn get_disk_read_usage(&self) -> String {
        converter::convert(self.disk_read_usage as f64)
    }

    fn get_disk_write_usage(&self) -> String {
        converter::convert(self.disk_write_usage as f64)
    }
}

fn new_process_item(pid: Pid, proc: &Process) -> ProcessItem {
    ProcessItem {
        pid,
        name: proc.name().to_owned(),
        cpu_usage: proc.cpu_usage() as u64,
        memory_usage: proc.memory(),
        disk_read_usage: proc.disk_usage().read_bytes,
        disk_write_usage: proc.disk_usage().written_bytes,
    }
}

// TEST -----------------------------------------------------

fn processes_boxed_strings_sorted_by(sys: &System, sort_by: SortBy) -> Box<[Box<[String]>]> {
    boxed_processes_sorted_by(sys, sort_by)
        .par_iter()
        .map(process_into_string_box_arr)
        .collect()
}

fn processes_boxed_slices_sorted_by() -> Box<[Box<[&'static str]>]> {
    Box::new([])
}

fn all_processes_boxed_slices_sorted_by(sys: &System, sort_by: SortBy) -> Box<[[String; 6]]> {
    boxed_processes_sorted_by(sys, sort_by)
        .par_iter()
        .map(process_into_string_arr)
        .collect()
}

fn all_processes_strings_arr_sorted_by(sys: &System, sort_by: SortBy) -> Vec<[String; 6]> {
    boxed_processes_sorted_by(sys, sort_by)
        .par_iter()
        .map(process_into_string_arr)
        .collect()
}

pub fn process_into_string_arr(item: &ProcessItem) -> [String; 6] {
    [
        item.get_pid(),
        item.get_name(),
        item.get_cpu_usage(),
        item.get_memory_usage(),
        item.get_disk_read_usage(),
        item.get_disk_write_usage(),
    ]
}

fn process_into_string_box_arr(item: &ProcessItem) -> Box<[String]> {
    [
        item.get_pid(),
        item.get_name(),
        item.get_cpu_usage(),
        item.get_memory_usage(),
        item.get_disk_read_usage(),
        item.get_disk_write_usage(),
    ]
    .into()
}

fn arc_processes_sorted_by(sys: &System, sort_by: SortBy) -> Arc<[ProcessItem]> {
    let mut processes = processes_into_boxed_items(sys);
    sort_processes_by(&mut processes, sort_by);

    processes.into()
}

// TEST -----------------------------------------------------

pub fn boxed_processes_sorted_by(sys: &System, sort_by: SortBy) -> Box<[ProcessItem]> {
    let mut processes = processes_into_boxed_items(sys);
    sort_processes_by(&mut processes, sort_by);

    processes
}

fn processes_into_boxed_items(sys: &System) -> Box<[ProcessItem]> {
    sys.processes()
        .par_iter()
        .map(|(pid, proc)| new_process_item(*pid, proc))
        .collect()
}

fn sort_processes_by(processes: &mut [ProcessItem], sort_by: SortBy) {
    match sort_by {
        SortBy::Pid => processes.par_sort_by_key(|p| p.pid),
        SortBy::Name => processes.par_sort_by_key(|p| p.name.clone()),
        SortBy::Cpu => processes.par_sort_by_key(|p| p.cpu_usage),
        SortBy::Memory => processes.par_sort_by_key(|p| p.memory_usage),
        SortBy::DiskRead => processes.par_sort_by_key(|p| p.disk_read_usage),
        SortBy::DiskWrite => processes.par_sort_by_key(|p| p.disk_write_usage),

        SortBy::PidReverse => processes.par_sort_by_key(|p| Reverse(p.pid)),
        SortBy::NameReverse => processes.par_sort_by_key(|p| Reverse(p.name.clone())),
        SortBy::CpuReverse => processes.par_sort_by_key(|p| Reverse(p.cpu_usage)),
        SortBy::MemoryReverse => processes.par_sort_by_key(|p| Reverse(p.memory_usage)),
        SortBy::DiskReadReverse => processes.par_sort_by_key(|p| Reverse(p.disk_read_usage)),
        SortBy::DiskWriteReverse => processes.par_sort_by_key(|p| Reverse(p.disk_write_usage)),
    }
}
