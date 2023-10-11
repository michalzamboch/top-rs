#![allow(dead_code)]

use sysinfo::{CpuExt, ProcessExt, System, SystemExt, Pid, Process};
use super::utils::get_floored_percentage;

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