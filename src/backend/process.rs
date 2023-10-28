#![allow(dead_code)]

use sysinfo::{Pid, Process, ProcessExt, System, SystemExt};
use std::cmp::Reverse;

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
            pid: pid,
            name: proc.name().to_owned(),
            cpu_usage: proc.cpu_usage() as u64,
            memory_usage: proc.memory(),
            disk_read_usage: proc.disk_usage().read_bytes,
            disk_write_usage: proc.disk_usage().written_bytes,
        }
    }

    pub fn to_string(item: &ProcessItem) -> String {
        format!("[{}] {} {}", item.pid, item.name, item.cpu_usage)
    }
}

pub fn process_info_sorted_by_cpu_to_string(sys: &System) -> Vec<String> {
    let vec_string: Vec<String> = process_info_sorted_by_cpu(sys)
        .iter()
        .map(|item| ProcessItem::to_string(item))
        .collect();
    vec_string
}

fn process_info_sorted_by_cpu(sys: &System) -> Vec<ProcessItem> {
    let mut process_vec = process_info_items(sys);
    process_vec.sort_by_key(|item| Reverse(item.cpu_usage.clone()));

    process_vec
}


pub fn process_info_sorted_by_name_to_string(sys: &System) -> Vec<String> {
    let vec_string: Vec<String> = process_info_sorted_by_name(sys)
        .iter()
        .map(|item| ProcessItem::to_string(item))
        .collect();
    vec_string
}

fn process_info_sorted_by_name(sys: &System) -> Vec<ProcessItem> {
    let mut process_vec = process_info_items(sys);
    process_vec.sort_by_key(|item| item.name.clone());

    process_vec
}

fn process_info_items(sys: &System) -> Vec<ProcessItem> {
    let process_vec: Vec<ProcessItem> = sys
        .processes()
        .iter()
        .map(|(pid, proc)| ProcessItem::new(pid.clone(), proc))
        .collect();

    process_vec
}

pub fn process_info_vec(sys: &System) -> Vec<String> {
    let mut process_vec = vec![];
    for (pid, process) in sys.processes() {
        process_vec.push(process_to_string(pid, process));
    }

    process_vec
}

pub fn print_process_info(sys: &System) {
    for (pid, process) in sys.processes() {
        print_process(pid, process);
    }
}

fn print_process(pid: &Pid, process: &Process) {
    println!("{}", process_to_string(pid, process));
}

fn process_to_string(pid: &Pid, process: &Process) -> String {
    format!("[{}] {} {}", pid, process.name(), process.cpu_usage())
}
