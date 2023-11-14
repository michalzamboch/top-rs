use std::{collections::*, fmt::Debug};

use crate::types::enums::sort_by::SortBy;

pub trait IApp: Send + Debug {
    fn update(&mut self);
    fn hard_update(&mut self);
    fn get_memory_usage(&self) -> u64;
    fn get_swap_usage(&self) -> u64;
    fn get_memory_details(&self) -> String;
    fn get_swap_details(&self) -> String;
    fn get_total_cpu_usage(&self) -> u64;
    fn get_cpu_details(&self) -> String;
    fn get_sys_info(&self) -> String;
    fn get_filtered_processes_vec_strings(&self) -> Vec<Vec<String>>;
    fn get_temperatures(&self) -> Vec<Vec<String>>;
    fn get_networks_list(&self) -> Vec<Vec<String>>;
    fn get_network_info(&self) -> HashMap<String, (u64, u64)>;
    fn get_disks_vec_string(&self) -> Vec<Vec<String>>;
    fn sort_processes_by(&mut self, sort_by: SortBy);
}
