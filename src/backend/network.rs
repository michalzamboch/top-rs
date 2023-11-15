#![allow(dead_code)]

use pretty_bytes::*;
use std::{collections::*, *};
use sysinfo::*;

use super::config;

fn get_current_network_vec_strings(sys: &System) -> Vec<Vec<String>> {
    sys.networks()
        .into_iter()
        .filter(|item| item.0 == config::WIFI_ID || item.0 == config::ETHERNET_ID)
        .map(create_connections_vec_strings)
        .collect()
}

fn get_network_vec_strings(sys: &System) -> Vec<Vec<String>> {
    sys.networks()
        .into_iter()
        .map(create_connections_vec_strings)
        .collect()
}

fn create_connections_vec_strings(connection: (&String, &NetworkData)) -> Vec<String> {
    vec![
        format!("{}", connection.0),
        converter::convert(connection.1.received() as f64),
        converter::convert(connection.1.transmitted() as f64),
    ]
}

pub fn get_current_total_connection_strings(sys: &System) -> (String, String) {
    let connection = get_current_network_total(sys);
    (
        converter::convert(connection.0 as f64),
        converter::convert(connection.1 as f64),
    )
}

pub fn get_current_connection_strings(sys: &System) -> (String, String) {
    let connection = get_current_network(sys);
    (
        converter::convert(connection.0 as f64),
        converter::convert(connection.1 as f64),
    )
}

fn get_connection_from_map(network_map: &HashMap<String, (u64, u64)>) -> (u64, u64) {
    let empty_connection = (0, 0);

    if network_map.contains_key(config::ETHERNET_ID) {
        network_map
            .get(config::ETHERNET_ID)
            .unwrap_or(&empty_connection)
            .clone()
    } else if network_map.contains_key(config::WIFI_ID) {
        network_map
            .get(config::WIFI_ID)
            .unwrap_or(&empty_connection)
            .clone()
    } else {
        empty_connection
    }
}

pub fn get_current_network_total(sys: &System) -> (u64, u64) {
    let network = get_network_total_map(sys);
    get_connection_from_map(&network)
}

pub fn get_current_network(sys: &System) -> (u64, u64) {
    let network = get_network_map(sys);
    get_connection_from_map(&network)
}

pub fn get_network_total_map(sys: &System) -> HashMap<String, (u64, u64)> {
    sys.networks()
        .into_iter()
        .filter(|item| item.0 == config::WIFI_ID || item.0 == config::ETHERNET_ID)
        .map(create_connection_total_tuple)
        .collect()
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
        (connection.1.received(), connection.1.transmitted()),
    )
}

fn create_connection_total_tuple(connection: (&String, &NetworkData)) -> (String, (u64, u64)) {
    (
        connection.0.to_owned(),
        (
            connection.1.total_received(),
            connection.1.total_transmitted(),
        ),
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
