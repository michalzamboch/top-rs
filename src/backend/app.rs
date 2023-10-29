#![allow(dead_code, unused_imports)]

use std::cmp;
use sysinfo::{CpuExt, NetworkData, NetworkExt, NetworksExt, ProcessExt, System, SystemExt};

use super::{config, process::*, utils::*};

pub struct App {
    sys: System,
    max_received_net: Vec<u64>,
    max_transmited_net: Vec<u64>,
}

impl App {
    pub fn new() -> App {
        let mut system = System::new();
        App::initial_sys_refresh(&mut system);
        let network_count = App::get_network_count(&mut system);

        App {
            sys: system,
            max_received_net: vec![0; network_count],
            max_transmited_net: vec![0; network_count],
        }
    }

    fn initial_sys_refresh(sys: &mut System) {
        for _ in 0..config::INITIAL_REFRESH_COUNT {
            sys.refresh_all();
        }
    }

    fn get_network_count(sys: &mut System) -> usize {
        sys.refresh_networks();
        sys.refresh_networks_list();

        let net = sys.networks();
        net.iter().count()
    }

    pub fn on_tick(&mut self) {
        self.sys.refresh_memory();
        self.sys.refresh_cpu();
        self.sys.refresh_networks();
        self.sys.refresh_networks_list();
        self.sys.refresh_processes();
    }

    pub fn get_memory_usage(&self) -> u64 {
        get_floored_percentage(self.sys.used_memory(), self.sys.total_memory())
    }

    pub fn get_core_usage(&self) -> Vec<u64> {
        self.sys
            .cpus()
            .iter()
            .map(|cpu| cpu.cpu_usage() as u64)
            .collect()
    }

    pub fn get_total_cpu_usage(&self) -> u64 {
        let usage = self.sys.global_cpu_info().cpu_usage();

        usage as u64
    }

    pub fn network(&mut self) {
        for (index, net_info) in self.sys.networks().into_iter().enumerate() {
            self.max_received_net[index] = self.get_max_received(index, net_info.1);

            self.max_transmited_net[index] = self.get_max_transmited(index, net_info.1);
        }
    }

    fn get_max_received(&self, index: usize, net_info: &NetworkData) -> u64 {
        let received = self.max_received_net[index];
        cmp::max(received, net_info.received())
    }

    fn get_max_transmited(&self, index: usize, net_info: &NetworkData) -> u64 {
        let transmitted = self.max_transmited_net[index];
        cmp::max(transmitted, net_info.transmitted())
    }

    fn print_net_info(&self, index: usize, net_info: (&String, &NetworkData)) {
        println!(
            "{} {}: {}/{} B",
            index,
            net_info.0,
            net_info.1.received(),
            net_info.1.transmitted()
        );
    }

    pub fn get_sys_info(&self) -> String {
        format!(
            "System name: {}, Kernel version: {}, OS version: {}, Host name: {}",
            self.get_sys_name(),
            self.get_kernel_version(),
            self.get_os_version(),
            self.get_host_name(),
        )
    }

    pub fn get_sys_name(&self) -> String {
        match self.sys.name() {
            Some(version) => version,
            None => "Unknown".to_owned(),
        }
    }

    pub fn get_kernel_version(&self) -> String {
        match self.sys.kernel_version() {
            Some(version) => version,
            None => "Unknown".to_owned(),
        }
    }

    pub fn get_os_version(&self) -> String {
        match self.sys.os_version() {
            Some(version) => version,
            None => "Unknown".to_owned(),
        }
    }

    pub fn get_host_name(&self) -> String {
        match self.sys.host_name() {
            Some(version) => version,
            None => "Unknown".to_owned(),
        }
    }

    pub fn get_processes_vec(&self) -> Vec<String> {
        process_info_sorted_by_cpu_to_string(&self.sys)
    }
}
