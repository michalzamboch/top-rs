#![allow(dead_code)]

use sysinfo::{Pid, Process, ProcessExt, System, SystemExt};

pub struct ProcessList {
    sys: System,
}

impl ProcessList {
    pub fn new(sys: System) -> ProcessList {
        ProcessList {
            sys: sys,
        }
    }
    
    pub fn process_info_vec(&self) -> Vec<String> {
        let mut process_vec = vec![];
        for (pid, process) in self.sys.processes() {
            process_vec.push(self.process_to_string(pid, process));
        }

        process_vec
    }

    pub fn print_process_info(&self) {
        for (pid, process) in self.sys.processes() {
            self.print_process(pid, process);
        }
    }
    
    fn print_process(&self, pid: &Pid, process: &Process) {
        println!("{}", self.process_to_string(pid, process));
    }

    fn process_to_string(&self, pid: &Pid, process: &Process) -> String {
        format!("[{}] {} {:?}", pid, process.name(), process.disk_usage())
    }
}

pub struct ProcessInfo {
    pid: Pid,
    proc: Process,
}

impl ProcessInfo {
    pub fn new(pid: Pid, proc: Process) -> ProcessInfo {
        ProcessInfo {
            pid: pid,
            proc: proc,
        }
    }
}
