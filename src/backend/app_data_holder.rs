#![allow(dead_code)]

use crate::types::{
    enums::sort_by::SortBy,
    traits::{app::IApp, table_data_holder::ITableDataHolder, creatable::ICreatable},
};

use super::process_data_holder::ProcessDataHolder;

#[derive(Debug)]
pub struct AppDataHolder {
    app: Box<dyn IApp>,

    memory_usage: u64,
    swap_usage: u64,
    memory_details: String,
    swap_details: String,
    total_cpu_usage: u64,
    cpu_details: String,
    sys_info: String,
    process_data: Box<dyn ITableDataHolder>,
    network_total_sum: (String, String),
    network_sum: (u64, u64),
    disks_vec: Vec<Vec<String>>,
}

impl ICreatable for  AppDataHolder {
    fn new() -> AppDataHolder {
        AppDataHolder {
            ..Default::default()
        }
    }
}

impl Default for AppDataHolder {
    fn default() -> Self {
        Self::new()
    }
}

impl IApp for AppDataHolder {
    fn update(&mut self) {
        self.app.update();
    }

    fn hard_update(&mut self) {
        self.app.hard_update();
    }

    fn get_memory_usage(&self) -> u64 {
        self.memory_usage
    }

    fn get_swap_usage(&self) -> u64 {
        self.swap_usage
    }

    fn get_memory_details(&self) -> String {
        self.memory_details.clone()
    }

    fn get_swap_details(&self) -> String {
        self.swap_details.clone()
    }

    fn get_total_cpu_usage(&self) -> u64 {
        self.total_cpu_usage
    }

    fn get_cpu_details(&self) -> String {
        self.cpu_details.clone()
    }

    fn get_sys_info(&self) -> String {
        self.sys_info.clone()
    }

    fn get_process_data_holder(&self) -> Box<dyn ITableDataHolder> {
        ProcessDataHolder::new_empty_dyn_box()
    }

    fn get_network_total_sum(&self) -> (String, String) {
        self.network_total_sum.clone()
    }

    fn get_network_sum(&self) -> (u64, u64) {
        self.network_sum
    }

    fn get_disks_vec_string(&self) -> Vec<Vec<String>> {
        self.disks_vec.clone()
    }

    fn sort_processes_by(&mut self, sort_by: SortBy) {
        self.app.sort_processes_by(sort_by);
    }
}
