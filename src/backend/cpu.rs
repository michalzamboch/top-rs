#![allow(dead_code)]

use rayon::prelude::*;
use sysinfo::*;

pub fn get_cores_usage(sys: &System) -> Vec<u64> {
    sys.cpus()
        .par_iter()
        .map(|cpu| cpu.cpu_usage() as u64)
        .collect()
}

pub fn get_total_cpu_usage(sys: &System) -> u64 {
    let usage = sys.global_cpu_info().cpu_usage();

    usage as u64
}

pub fn get_cpu_details(sys: &System) -> String {
    let core_count = sys.physical_core_count();
    let cpu_brand = sys.global_cpu_info().brand().trim_end();

    match core_count {
        Some(count) => format!("{} | {} Core", cpu_brand, count),
        None => format!("{}", cpu_brand),
    }
}
