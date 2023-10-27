#![allow(dead_code)]

use sysinfo::{Pid, Process};

pub struct ProcessInfo {
    pid: Pid,
    proc: Process,
}

impl ProcessInfo {
    pub fn new(pid: &Pid, proc: Process) -> ProcessInfo {
        ProcessInfo {
            pid: pid.clone(),
            proc: proc,
        }
    }
}