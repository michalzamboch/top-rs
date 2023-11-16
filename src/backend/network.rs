use pretty_bytes::*;
use std::{collections::*, *};
use sysinfo::*;

pub fn get_total_connection_strings(sys: &System) -> (String, String) {
    let connection = get_total_connection_sum(sys);
    (
        converter::convert(connection.0 as f64),
        converter::convert(connection.1 as f64),
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

pub fn get_network_map(sys: &System) -> HashMap<String, (u64, u64)> {
    sys.networks()
        .into_iter()
        .map(create_connection_tuple)
        .collect()
}

fn create_connection_tuple(connection: (&String, &NetworkData)) -> (String, (u64, u64)) {
    (
        connection.0.to_owned(),
        (connection.1.received(), connection.1.transmitted()),
    )
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
