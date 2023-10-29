#![allow(dead_code, unused_imports)]

use std::cmp;
use sysinfo::{CpuExt, NetworkData, NetworkExt, NetworksExt, ProcessExt, System, SystemExt};

use super::{config, cpu, memory, network::*, pc_info, process};

pub struct App {
    sys: System,
    network: Networking,
}

impl App {
    pub fn new() -> App {
        let mut sys = System::new();
        App::initial_sys_refresh(&mut sys);

        let network = Networking::new(&mut sys);

        App { sys, network }
    }

    fn initial_sys_refresh(sys: &mut System) {
        for _ in 0..config::INITIAL_REFRESH_COUNT {
            sys.refresh_all();
        }
    }

    pub fn on_tick(&mut self) {
        self.sys.refresh_memory();
        self.sys.refresh_cpu();
        self.sys.refresh_networks();
        self.sys.refresh_networks_list();
        self.sys.refresh_processes();
    }

    pub fn get_memory_usage(&self) -> u64 {
        memory::get_memory_usage(&self.sys)
    }

    pub fn get_memory_details(&self) -> String {
        memory::get_memory_details(&self.sys)
    }

    pub fn get_total_cpu_usage(&self) -> u64 {
        cpu::get_total_cpu_usage(&self.sys)
    }

    pub fn get_cpu_details(&self) -> String {
        cpu::get_cpu_details(&self.sys)
    }

    pub fn get_sys_info(&self) -> String {
        pc_info::get_sys_info(&self.sys)
    }

    pub fn get_processes_vec(&self) -> Vec<String> {
        process::process_info_sorted_by_cpu_to_string(&self.sys)
    }
}
