
use std::collections::HashMap;

use super::{sort_by::*, temperature_traits::ITemperature};

pub trait IApp: Sized {
    fn on_tick(&mut self);
    fn get_memory_usage(&self) -> u64;
    fn get_memory_details(&self) -> String;
    fn get_total_cpu_usage(&self) -> u64;
    fn get_cpu_details(&self) -> String;
    fn get_sys_info(&self) -> String;
    fn get_filtered_processes_vec_strings(&self) -> Vec<Vec<String>>;
    fn get_temperatures(&self) -> HashMap<String, Box<dyn ITemperature>>;
    fn sort_processes_by(&mut self, sort_by: SortBy);
}
