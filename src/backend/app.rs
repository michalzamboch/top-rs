#![allow(dead_code)]

use sysinfo::{CpuExt, NetworkExt, NetworksExt, ProcessExt, System, SystemExt};
use std::cmp;

use super::utils::get_floored_percentage;

pub struct App {
    sys: System,
    max_received_net: u64,
    max_transmited_net: u64,
}

impl App {
    pub fn new() -> App {
        let mut system = System::new();
        system.refresh_all();

        App { sys: system, max_received_net: 0, max_transmited_net: 0 }
    }

    pub fn on_tick(&mut self) {
        self.sys.refresh_memory();
        self.sys.refresh_cpu();
        self.sys.refresh_networks();
        self.sys.refresh_networks_list();
    }

    pub fn get_memory_usage(&self) -> u64 {
        get_floored_percentage(self.sys.used_memory(), self.sys.total_memory())
    }

    pub fn get_core_usage(&self) -> Vec<u64> {
        let mut percentage = vec![];

        for cpu in self.sys.cpus() {
            percentage.push(cpu.cpu_usage() as u64);
        }

        percentage
    }

    pub fn get_total_cpu_usage(&self) -> u64 {
        let usage = self.sys.global_cpu_info().cpu_usage();

        usage as u64
    }

    fn network(&self) {
        println!("=> networks:");
        for (interface_name, data) in self.sys.networks() {
            println!(
                "{}: {}/{} B",
                interface_name,
                data.received(),
                data.transmitted()
            );
        }
    }

    fn update_net_info(&mut self) {
        
    }

    pub fn get_process_info(&self) {
        for (pid, process) in self.sys.processes() {
            println!("[{}] {} {:?}", pid, process.name(), process.disk_usage());
        }
    }
}
