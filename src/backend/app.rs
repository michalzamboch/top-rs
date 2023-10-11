#![allow(dead_code)]

use sysinfo::{CpuExt, ProcessExt, System, SystemExt};

use super::utils::get_floored_percentage;

pub struct App {
    sys: System,
}

impl App {
    pub fn new() -> App {
        App { sys: System::new() }
    }

    pub fn on_tick(&mut self) {
        self.sys.refresh_memory();
        self.sys.refresh_cpu();
    }

    pub fn get_memory_usage(&self) -> u64 {
        get_floored_percentage(self.sys.used_memory(), self.sys.total_memory())
    }

    pub fn get_core_usage(&self) -> Vec<u64> {
        let mut percentage = vec![];

        for cpu in self.sys.cpus().clone() {
            percentage.push(cpu.cpu_usage() as u64);
        }

        percentage
    }

    pub fn get_process_info(&self) {
        for (pid, process) in self.sys.processes() {
            println!("[{}] {} {:?}", pid, process.name(), process.disk_usage());
        }
    }
}
