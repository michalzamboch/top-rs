#![allow(dead_code)]

use sysinfo::{Pid, Process, ProcessExt, System, SystemExt};

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
