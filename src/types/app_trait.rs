
use super::sort_by::*;

pub trait IApp: Sized {
    fn on_tick(&mut self);
    fn get_memory_usage(&self) -> u64;
    fn get_memory_details(&self) -> String;
    fn get_total_cpu_usage(&self) -> u64;
    fn get_cpu_details(&self) -> String;
    fn get_sys_info(&self) -> String;
    fn get_filtered_processes_vec(&self, max_count: usize) -> Vec<String>;
    fn sort_processes_by(&mut self, sort_by: SortBy);
}
