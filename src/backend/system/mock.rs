use crate::types::{
    enums::sort_by::SortBy,
    traits::{app::IApp, creatable::ICreatable, table_data_holder::ITableDataHolder},
};

use super::process_data_holder::ProcessDataHolder;

#[derive(Debug, Default)]
pub struct MockApp;

impl ICreatable for MockApp {
    fn new() -> MockApp {
        MockApp {}
    }
}

impl IApp for MockApp {
    fn update(&mut self) {}
    fn hard_update(&mut self) {}

    fn get_memory_usage(&self) -> u64 {
        100
    }

    fn get_swap_usage(&self) -> u64 {
        100
    }

    fn get_memory_details(&self) -> String {
        "Free: 24GB | Used: 8 GB | Total 24 GB".to_owned()
    }

    fn get_swap_details(&self) -> String {
        "Free: 8 GB | Used: 0 GB | Total 8 GB".to_owned()
    }

    fn get_total_cpu_usage(&self) -> u64 {
        100
    }

    fn get_cpu_details(&self) -> String {
        "AMD Ryzen | 16 Core".to_owned()
    }

    fn get_sys_info(&self) -> String {
        "OS: Windows | Kernel: 69420".to_owned()
    }

    fn get_sys_load(&self) -> String {
        "1 min: 5%, 5 min: 10%, 15 min: 7%".to_string()
    }

    fn get_network_total_sum(&self) -> (String, String) {
        ("100.00 MB".to_owned(), "200.00 MB".to_owned())
    }

    fn get_network_sum(&self) -> (u64, u64) {
        (100, 100)
    }

    fn get_disks_vec_string(&self) -> Vec<Vec<String>> {
        let disk = vec![
            "X:".to_owned(),
            "100.00 GB".to_owned(),
            "100.00 GB".to_owned(),
            "200.00 GB".to_owned(),
        ];
        vec![disk; 10]
    }

    fn sort_processes_by(&mut self, sort_by: SortBy) {
        _ = sort_by;
    }

    fn get_process_data_holder(&self) -> Box<dyn ITableDataHolder> {
        Box::<ProcessDataHolder>::default()
    }
}
