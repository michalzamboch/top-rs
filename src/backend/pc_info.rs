#![allow(dead_code)]

use std::rc::Rc;
use sysinfo::*;

static UNKNOWN: &str = "Unknown";

pub fn get_sys_info(sys: &System) -> String {
    format!(
        "System name: {}, Kernel version: {}, OS version: {}, Host name: {}",
        get_sys_name(sys),
        get_kernel_version(sys),
        get_os_version(sys),
        get_host_name(sys),
    )
}

pub fn get_sys_info_vec(sys: &System) -> Rc<[Vec<String>]> {
    vec![
        vec!["System name".to_owned(), get_sys_name(sys)],
        vec!["Kernel version".to_owned(), get_kernel_version(sys)],
        vec!["OS version".to_owned(), get_os_version(sys)],
        vec!["Host name".to_owned(), get_host_name(sys)],
        vec!["Boot time".to_owned(), get_boot_time(sys)],
    ]
    .into()
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
