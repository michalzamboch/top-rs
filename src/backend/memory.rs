use super::utils::*;
use pretty_bytes::converter::convert;
use sysinfo::{System, SystemExt};

pub fn get_memory_usage(sys: &System) -> u64 {
    get_floored_percentage(sys.used_memory(), sys.total_memory())
}

pub fn get_memory_details(sys: &System) -> String {
    let tmp_free_mem = convert(sys.free_memory() as f64);
    let tmp_used_mem = convert(sys.used_memory() as f64);
    let tmp_total_mem = convert(sys.total_memory() as f64);

    format!(
        "Free: {} | Used: {} | Total {}",
        tmp_free_mem, tmp_used_mem, tmp_total_mem
    )
}
