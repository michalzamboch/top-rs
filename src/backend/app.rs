#![allow(dead_code)]

use sysinfo::*;

use crate::types::{enums::sort_by::SortBy, traits::{app::IApp, creatable::ICreatable}};

use super::{
    config, cpu, memory,
    network::{self, *},
    pc_info, process, temperatures,
};

#[derive(Debug, Default)]
pub struct App {
    sys: System,
    network: Networking,
    processes_sorted_by: SortBy,
}

impl App {
    fn initial_sys_refresh(sys: &mut System) {
        for _ in 0..config::INITIAL_REFRESH_COUNT {
            sys.refresh_all();
        }
    }
}

impl ICreatable for App {
    fn new() -> App {
        let mut sys = System::new();
        App::initial_sys_refresh(&mut sys);

        let network = Networking::new(&mut sys);

        App {
            sys,
            network,
            processes_sorted_by: SortBy::default(),
        }
    }
}

impl IApp for App {
    fn on_tick(&mut self) {
        self.sys.refresh_memory();
        self.sys.refresh_cpu();
        self.sys.refresh_networks();
        self.sys.refresh_processes();
        self.sys.refresh_components();
    }

    fn get_memory_usage(&self) -> u64 {
        memory::get_memory_usage(&self.sys)
    }

    fn get_memory_details(&self) -> String {
        memory::get_memory_details(&self.sys)
    }

    fn get_total_cpu_usage(&self) -> u64 {
        cpu::get_total_cpu_usage(&self.sys)
    }

    fn get_cpu_details(&self) -> String {
        cpu::get_cpu_details(&self.sys)
    }

    fn get_sys_info(&self) -> String {
        pc_info::get_sys_info(&self.sys)
    }

    fn get_filtered_processes_vec_strings(&self) -> Vec<Vec<String>> {
        process::all_processes_strings_vec_sorted_by(&self.sys, self.processes_sorted_by)
    }

    fn get_temperatures(&self) -> Vec<Vec<String>> {
        temperatures::get_temperatures_vec_strings(&self.sys)
    }

    fn get_networks_list(&self) -> Vec<Vec<String>> {
        network::get_network_vec_strings(&self.sys)
    }

    fn sort_processes_by(&mut self, sort_by: SortBy) {
        self.processes_sorted_by = sort_by;
    }
}
