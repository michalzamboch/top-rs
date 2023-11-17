#![allow(dead_code)]

use sysinfo::*;

static UNKNOWN: & str = "Unknown";

pub fn get_sys_info(sys: &System) -> String {
    format!(
        "System name: {}, Kernel version: {}, OS version: {}, Host name: {}",
        get_sys_name(sys),
        get_kernel_version(sys),
        get_os_version(sys),
        get_host_name(sys),
    )
}

fn get_sys_name(sys: &System) -> String {
    sys.name().unwrap_or(UNKNOWN.to_owned())
}

fn get_kernel_version(sys: &System) -> String {
    sys.kernel_version().unwrap_or(UNKNOWN.to_owned())
}

fn get_os_version(sys: &System) -> String {
    sys.os_version().unwrap_or(UNKNOWN.to_owned())
}

fn get_host_name(sys: &System) -> String {
    sys.host_name().unwrap_or(UNKNOWN.to_owned())
}

fn get_boot_time(sys: &System) -> String {
    format!("{} ms", sys.boot_time())
}
