use super::utils::*;
use pretty_bytes::converter::*;
use sysinfo::*;

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

pub fn get_swap_usage(sys: &System) -> u64 {
    get_floored_percentage(sys.used_swap(), sys.total_swap())
}

pub fn get_swap_details(sys: &System) -> String {
    let free_swap = convert(sys.free_swap() as f64);
    let used_swap = convert(sys.used_swap() as f64);
    let total_swap = convert(sys.total_swap() as f64);

    format!(
        "Free: {} | Used: {} | Total {}",
        free_swap, used_swap, total_swap
    )
}
