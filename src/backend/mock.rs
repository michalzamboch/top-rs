use crate::types::{app_trait::IApp, sort_by::SortBy};

#[derive(Debug, Default)]
pub struct MockApp;

impl MockApp {
    pub fn new() -> MockApp {
        MockApp {}
    }
}

impl IApp for MockApp {
    fn on_tick(&mut self) {}

    fn get_memory_usage(&self) -> u64 {
        50
    }

    fn get_memory_details(&self) -> String {
        format!("Total: 32GB | Used: 8 GB | Free 24 GB")
    }

    fn get_total_cpu_usage(&self) -> u64 {
        25
    }

    fn get_cpu_details(&self) -> String {
        format!("AMD Ryzen | 16 Core")
    }

    fn get_sys_info(&self) -> String {
        format!("OS: Windows | Kernel: 69420")
    }

    fn get_filtered_processes_vec_strings(&self) -> Vec<Vec<String>> {
        vec![
            vec![
                "123456".to_owned(),
                "Process".to_owned(),
                "100 %".to_owned(),
                "100.00 MB".to_owned(),
                "100.00 KB".to_owned(),
                "100.00 KB".to_owned(),
            ],
            vec![
                "123456".to_owned(),
                "Process".to_owned(),
                "100 %".to_owned(),
                "100.00 MB".to_owned(),
                "100.00 KB".to_owned(),
                "100.00 KB".to_owned(),
            ],
            vec![
                "123456".to_owned(),
                "Process".to_owned(),
                "100 %".to_owned(),
                "100.00 MB".to_owned(),
                "100.00 KB".to_owned(),
                "100.00 KB".to_owned(),
            ],
            vec![
                "123456".to_owned(),
                "Process".to_owned(),
                "100 %".to_owned(),
                "100.00 MB".to_owned(),
                "100.00 KB".to_owned(),
                "100.00 KB".to_owned(),
            ],
            vec![
                "123456".to_owned(),
                "Process".to_owned(),
                "100 %".to_owned(),
                "100.00 MB".to_owned(),
                "100.00 KB".to_owned(),
                "100.00 KB".to_owned(),
            ],
            vec![
                "123456".to_owned(),
                "Process".to_owned(),
                "100 %".to_owned(),
                "100.00 MB".to_owned(),
                "100.00 KB".to_owned(),
                "100.00 KB".to_owned(),
            ],
            vec![
                "123456".to_owned(),
                "Process".to_owned(),
                "100 %".to_owned(),
                "100.00 MB".to_owned(),
                "100.00 KB".to_owned(),
                "100.00 KB".to_owned(),
            ],
            vec![
                "123456".to_owned(),
                "Process".to_owned(),
                "100 %".to_owned(),
                "100.00 MB".to_owned(),
                "100.00 KB".to_owned(),
                "100.00 KB".to_owned(),
            ],
            vec![
                "123456".to_owned(),
                "Process".to_owned(),
                "100 %".to_owned(),
                "100.00 MB".to_owned(),
                "100.00 KB".to_owned(),
                "100.00 KB".to_owned(),
            ],
            vec![
                "123456".to_owned(),
                "Process".to_owned(),
                "100 %".to_owned(),
                "100.00 MB".to_owned(),
                "100.00 KB".to_owned(),
                "100.00 KB".to_owned(),
            ],
            vec![
                "123456".to_owned(),
                "Process".to_owned(),
                "100 %".to_owned(),
                "100.00 MB".to_owned(),
                "100.00 KB".to_owned(),
                "100.00 KB".to_owned(),
            ],
        ]
    }

    fn get_temperatures(&self) -> Vec<Vec<String>> {
        vec![vec!["CPU".to_owned(), "100 °C".to_owned()]]
    }

    fn get_networks_list(&self) -> Vec<Vec<String>> {
        vec![
            vec![
                "Ethernet".to_owned(),
                "100.00 MB".to_owned(),
                "100.00 MB".to_owned(),
            ],
            vec![
                "Ethernet".to_owned(),
                "100.00 MB".to_owned(),
                "100.00 MB".to_owned(),
            ],
        ]
    }

    fn sort_processes_by(&mut self, sort_by: SortBy) {
        _ = sort_by;
    }
}
