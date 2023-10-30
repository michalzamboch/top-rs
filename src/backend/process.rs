use pretty_bytes::converter;
use rayon::prelude::*;
use std::cmp::Reverse;
use sysinfo::{Pid, Process, ProcessExt, System, SystemExt};

use crate::backend::config::*;
use crate::backend::utils::*;
use crate::types::sort_by::SortBy;

struct ProcessItem {
    pub pid: Pid,
    pub name: String,
    pub cpu_usage: u64,
    pub memory_usage: u64,
    pub disk_read_usage: u64,
    pub disk_write_usage: u64,
}

impl ProcessItem {
    pub fn new(pid: Pid, proc: &Process) -> ProcessItem {
        ProcessItem {
            pid,
            name: proc.name().to_owned(),
            cpu_usage: proc.cpu_usage() as u64,
            memory_usage: proc.memory(),
            disk_read_usage: proc.disk_usage().read_bytes,
            disk_write_usage: proc.disk_usage().written_bytes,
        }
    }

    pub fn to_string(item: &ProcessItem) -> String {
        let tmp_pid = format!("[{}]", item.pid);
        let tmp_name = fancy_trim_to_length(&item.name, NAME_STR_TRIM_LEN);
        let tmp_cpu_usage = format!("{}%", item.cpu_usage);
        let tmp_mem_usage = converter::convert(item.memory_usage as f64);
        let tmp_disk_read = converter::convert(item.disk_read_usage as f64);
        let tmp_disk_write = converter::convert(item.disk_write_usage as f64);

        format!(
            "{:PID_STR_LEN$} {:NAME_STR_LEN$} {:CPU_USAGE_STR_LEN$} {:PRETTY_BZTES_STR_LEN$} {:PRETTY_BZTES_STR_LEN$} {:PRETTY_BZTES_STR_LEN$}",
            tmp_pid, tmp_name, tmp_cpu_usage, tmp_mem_usage, tmp_disk_read, tmp_disk_write,
        )
    }
}

pub fn string_processes_sorted_by(sys: &System, sort_by: SortBy, max_count: usize) -> Vec<String> {
    processes_sorted_by(sys, sort_by)
        .par_iter()
        .take(max_count)
        .map(ProcessItem::to_string)
        .collect()
}

fn processes_sorted_by(sys: &System, sort_by: SortBy) -> Vec<ProcessItem> {
    let mut process_vec = process_info_items(sys);
    sort_processes_by(&mut process_vec, sort_by);

    process_vec
}

fn process_info_items(sys: &System) -> Vec<ProcessItem> {
    sys.processes()
        .par_iter()
        .map(|(pid, proc)| ProcessItem::new(*pid, proc))
        .collect()
}

fn sort_processes_by(process_vec: &mut [ProcessItem], sort_by: SortBy) {
    match sort_by {
        SortBy::Pid => process_vec.par_sort_by_key(|item| item.pid),
        SortBy::Name => process_vec.par_sort_by_key(|item| item.name.clone()),
        SortBy::Cpu => process_vec.par_sort_by_key(|item| item.cpu_usage),
        SortBy::Memory => process_vec.par_sort_by_key(|item| item.memory_usage),
        SortBy::DiskRead => process_vec.par_sort_by_key(|item| item.disk_read_usage),
        SortBy::DiskWrite => process_vec.par_sort_by_key(|item| item.disk_write_usage),

        SortBy::PidReverse => process_vec.par_sort_by_key(|item| Reverse(item.pid)),
        SortBy::NameReverse => process_vec.par_sort_by_key(|item| Reverse(item.name.clone())),
        SortBy::CpuReverse => process_vec.par_sort_by_key(|item| Reverse(item.cpu_usage)),
        SortBy::MemoryReverse => process_vec.par_sort_by_key(|item| Reverse(item.memory_usage)),
        SortBy::DiskReadReverse => process_vec.par_sort_by_key(|item| Reverse(item.disk_read_usage)),
        SortBy::DiskWriteReverse => process_vec.par_sort_by_key(|item| Reverse(item.disk_write_usage)),
    }
}
