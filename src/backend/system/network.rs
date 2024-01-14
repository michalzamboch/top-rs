use sysinfo::*;

use crate::backend::utils::bytes;

use super::config::*;

pub fn get_total_connection_strings(sys: &System) -> (String, String) {
    let connection = get_total_connection_sum(sys);
    (
        bytes::convert(connection.0 as f64, REGULAR_DELIMITER),
        bytes::convert(connection.1 as f64, REGULAR_DELIMITER),
    )
}

fn get_total_connection_sum(sys: &System) -> (u64, u64) {
    (get_total_receive_sum(sys), get_total_transmitted_sum(sys))
}

fn get_total_receive_sum(sys: &System) -> u64 {
    sys.networks()
        .into_iter()
        .map(|x| x.1.total_received())
        .sum()
}

fn get_total_transmitted_sum(sys: &System) -> u64 {
    sys.networks()
        .into_iter()
        .map(|x| x.1.total_transmitted())
        .sum()
}

pub fn get_connection_sum(sys: &System) -> (u64, u64) {
    (get_receive_sum(sys), get_transmitted_sum(sys))
}

fn get_transmitted_sum(sys: &System) -> u64 {
    sys.networks().into_iter().map(|x| x.1.transmitted()).sum()
}

fn get_receive_sum(sys: &System) -> u64 {
    sys.networks().into_iter().map(|x| x.1.received()).sum()
}
