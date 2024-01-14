#![allow(dead_code)]

use rayon::prelude::*;
use std::{cmp::Reverse, sync::Arc};
use sysinfo::{ProcessExt, *};

use crate::{types::{enums::sort_by::SortBy, traits::process::IProcessStringView}, backend::utils::bytes};

use super::config::REGULAR_DELIMITER;

#[derive(Debug, Hash, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub struct ProcessItem {
    pid: Pid,
    name: String,
    cpu_usage: u64,
    memory_usage: u64,
    disk_read_usage: u64,
    disk_write_usage: u64,
}

impl ProcessItem {
    fn new(pid: Pid, proc: &Process) -> ProcessItem {
        ProcessItem {
            pid,
            name: proc.name().to_owned(),
            cpu_usage: proc.cpu_usage() as u64,
            memory_usage: proc.memory(),
            disk_read_usage: proc.disk_usage().read_bytes,
            disk_write_usage: proc.disk_usage().written_bytes,
        }
    }

    pub fn kill(&self) -> bool {
        let mut sys = System::new();
        sys.refresh_process_specifics(self.pid, ProcessRefreshKind::everything());

        if let Some(process) = sys.process(self.pid) {
            return process.kill();
        }

        false
    }
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
        bytes::convert(self.memory_usage as f64, REGULAR_DELIMITER)
    }

    fn get_disk_read_usage(&self) -> String {
        bytes::convert(self.disk_read_usage as f64, REGULAR_DELIMITER)
    }

    fn get_disk_write_usage(&self) -> String {
        bytes::convert(self.disk_write_usage as f64, REGULAR_DELIMITER)
    }
}

pub fn arc_processes_sorted_by(sys: &System, sort_by: SortBy) -> Arc<[ProcessItem]> {
    let mut processes = processes_into_boxed_items(sys);
    sort_processes_by(&mut processes, sort_by);

    processes.into()
}

pub fn boxed_filtered_processes_sorted_by(
    sys: &System,
    sort_by: SortBy,
    sub_str: Option<&str>,
) -> Box<[ProcessItem]> {
    let mut processes = match sub_str {
        Some(tmp_str) => processes_into_boxed_filtered_items(sys, tmp_str),
        None => processes_into_boxed_items(sys),
    };
    sort_processes_by(&mut processes, sort_by);

    processes
}

fn processes_into_boxed_filtered_items(sys: &System, sub_str: &str) -> Box<[ProcessItem]> {
    sys.processes()
        .par_iter()
        .filter(|item| item.1.name().contains(sub_str))
        .map(|(pid, proc)| ProcessItem::new(*pid, proc))
        .collect()
}

pub fn boxed_processes_sorted_by(sys: &System, sort_by: SortBy) -> Box<[ProcessItem]> {
    let mut processes = processes_into_boxed_items(sys);
    sort_processes_by(&mut processes, sort_by);

    processes
}

fn processes_into_boxed_items(sys: &System) -> Box<[ProcessItem]> {
    sys.processes()
        .par_iter()
        .map(|(pid, proc)| ProcessItem::new(*pid, proc))
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
