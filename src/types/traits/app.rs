use std::fmt::Debug;

use crate::types::enums::sort_by::SortBy;

use super::table_data_holder::ITableDataHolder;

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
    fn get_sys_load(&self) -> String;
    fn get_process_data_holder(&self) -> Box<dyn ITableDataHolder>;

    fn get_network_total_sum(&self) -> (String, String);
    fn get_network_sum(&self) -> (u64, u64);

    fn get_disks_vec_string(&self) -> Vec<Vec<String>>;
    fn sort_processes_by(&mut self, sort_by: SortBy);
}
