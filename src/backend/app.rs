#![allow(dead_code)]

use std::{collections::HashMap, sync::Arc};

use sysinfo::*;

use crate::types::{
    enums::sort_by::SortBy,
    traits::{app::IApp, creatable::ICreatable},
};

use super::*;

#[derive(Debug, Default)]
pub struct App {
    sys: System,
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

        App {
            sys,
            processes_sorted_by: SortBy::default(),
        }
    }
}

impl IApp for App {
    fn update(&mut self) {
        self.sys.refresh_memory();
        self.sys.refresh_cpu();
        self.sys.refresh_networks();
        self.sys.refresh_networks_list();
        self.sys.refresh_processes();
        self.sys.refresh_disks();
        self.sys.refresh_system();
    }

    fn hard_update(&mut self) {
        self.sys.refresh_all();
    }

    fn get_memory_usage(&self) -> u64 {
        memory::get_memory_usage(&self.sys)
    }

    fn get_swap_usage(&self) -> u64 {
        memory::get_swap_usage(&self.sys)
    }

    fn get_memory_details(&self) -> String {
        memory::get_memory_details(&self.sys)
    }

    fn get_swap_details(&self) -> String {
        memory::get_swap_details(&self.sys)
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

    fn get_temperatures(&self) -> Arc<[Vec<String>]> {
        temperatures::get_temperatures_vec_strings(&self.sys).clone()
    }

    fn get_network_total_sum(&self) -> (String, String) {
        network::get_total_connection_strings(&self.sys)
    }

    fn get_network_map(&self) -> HashMap<String, (u64, u64)> {
        network::get_network_map(&self.sys)
    }

    fn get_network_sum(&self) -> (u64, u64) {
        network::get_connection_sum(&self.sys)
    }

    fn get_disks_vec_string(&self) -> Vec<Vec<String>> {
        disk::get_disks_vec_string(&self.sys)
    }

    fn sort_processes_by(&mut self, sort_by: SortBy) {
        self.processes_sorted_by = sort_by;
    }
}
