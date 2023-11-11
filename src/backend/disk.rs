
use pretty_bytes::converter;
use rayon::prelude::*;
use sysinfo::*;

use crate::types::traits::disk::IDiskStringView;

use super::utils::os_string_to_regular;

#[derive(Debug, Hash, PartialEq, Eq, PartialOrd, Ord, Clone)]
struct DiskInfo {
    pub name: String,
    pub free: u64,
    pub used: u64,
    pub total: u64,
}

impl IDiskStringView for DiskInfo {
    fn get_name(&self) -> String {
        self.name.clone()
    }

    fn get_free_space(&self) -> String {
        converter::convert(self.free as f64)
    }

    fn get_used_space(&self) -> String {
        converter::convert(self.used as f64)
    }

    fn get_total_space(&self) -> String {
        converter::convert(self.total as f64)
    }
}

pub fn get_disks_vec_string(sys: &System) -> Vec<Vec<String>> {
    get_disk_info_vec(sys)
        .par_iter()
        .map(get_disk_info_vec_string)
        .collect()
}

fn get_disk_info_vec_string(disk_info: &DiskInfo) -> Vec<String> {
    vec![
        disk_info.get_name(),
        disk_info.get_free_space(),
        disk_info.get_used_space(),
        disk_info.get_total_space(),
    ]
}

fn get_disk_info_vec(sys: &System) -> Vec<DiskInfo> {
    sys.disks().par_iter().map(new_string_disk).collect()
}

fn new_string_disk(disk: &Disk) -> DiskInfo {
    DiskInfo {
        name: os_string_to_regular(disk.name()),
        free: disk.available_space(),
        used: disk.total_space() - disk.available_space(),
        total: disk.total_space(),
    }
}
