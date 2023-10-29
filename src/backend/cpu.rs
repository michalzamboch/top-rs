#![allow(dead_code)]

use sysinfo::{CpuExt, System, SystemExt};

pub fn get_core_usage(sys: &System) -> Vec<u64> {
    sys.cpus()
        .iter()
        .map(|cpu| cpu.cpu_usage() as u64)
        .collect()
}

pub fn get_total_cpu_usage(sys: &System) -> u64 {
    let usage = sys.global_cpu_info().cpu_usage();

    usage as u64
}

pub fn get_cpu_details(sys: &System) -> String {
    let tmp_core_count = sys.cpus().len();
    let tmp_cpu_brand = sys.global_cpu_info().brand().trim_end();

    format!("{} | {} Core", tmp_cpu_brand, tmp_core_count)
}
