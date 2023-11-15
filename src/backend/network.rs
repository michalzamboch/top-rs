#![allow(dead_code)]

use pretty_bytes::*;
use std::{collections::*, *};
use sysinfo::*;

use super::config;

pub fn get_current_network_vec_strings(sys: &System) -> Vec<Vec<String>> {
    sys.networks()
        .into_iter()
        .filter(|item| item.0 == config::WIFI_ID || item.0 == config::ETHERNET_ID)
        .map(create_connections_vec_strings)
        .collect()
}

pub fn get_network_vec_strings(sys: &System) -> Vec<Vec<String>> {
    sys.networks()
        .into_iter()
        .map(create_connections_vec_strings)
        .collect()
}

fn create_connections_vec_strings(connection: (&String, &NetworkData)) -> Vec<String> {
    vec![
        format!("{}", connection.0),
        converter::convert(connection.1.transmitted() as f64),
        converter::convert(connection.1.received() as f64),
    ]
}

pub fn get_current_network(sys: &System) -> (u64, u64) {
    let network = get_network_map(sys);
    let empty_connection = (0, 0);

    if network.contains_key(config::ETHERNET_ID) {
        network
            .get(config::ETHERNET_ID)
            .unwrap_or(&empty_connection)
            .clone()
    } else if network.contains_key(config::WIFI_ID) {
        network
            .get(config::WIFI_ID)
            .unwrap_or(&empty_connection)
            .clone()
    } else {
        empty_connection
    }
}

pub fn get_network_map(sys: &System) -> HashMap<String, (u64, u64)> {
    sys.networks()
        .into_iter()
        .filter(|item| item.0 == config::WIFI_ID || item.0 == config::ETHERNET_ID)
        .map(create_connection_tuple)
        .collect()
}

fn create_connection_tuple(connection: (&String, &NetworkData)) -> (String, (u64, u64)) {
    (
        connection.0.to_owned(),
        (connection.1.transmitted(), connection.1.received()),
    )
}
