#![allow(dead_code)]

use sysinfo::*;

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
    get_info_or_unknown(sys.name())
}

fn get_kernel_version(sys: &System) -> String {
    get_info_or_unknown(sys.kernel_version())
}

fn get_os_version(sys: &System) -> String {
    get_info_or_unknown(sys.os_version())
}

fn get_host_name(sys: &System) -> String {
    get_info_or_unknown(sys.host_name())
}

fn get_boot_time(sys: &System) -> String {
    format!("{} ms", sys.boot_time())
}

fn get_info_or_unknown(info: Option<String>) -> String {
    match info {
        Some(info_str) => info_str,
        None => "Unknown".to_owned(),
    }
}
