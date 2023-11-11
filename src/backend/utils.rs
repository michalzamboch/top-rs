#![allow(dead_code)]

use std::ffi::*;

pub fn get_floored_percentage(part: u64, total: u64) -> u64 {
    let mut percentage = part as f64 * 100.0;
    percentage /= total as f64;

    percentage.floor() as u64
}

pub fn trim_string_to_length(input: &str, max_length: usize) -> String {
    input.chars().take(max_length).collect()
}

pub fn fancy_trim_to_length(input: &str, max_length: usize) -> String {
    let trimmed_str = trim_string_to_length(input, max_length);
    if input.eq(&trimmed_str) {
        trimmed_str
    } else {
        format!("{}...", trimmed_str)
    }
}

pub fn os_string_to_regular(os_string: &OsStr) -> String {
    let result = os_string.to_os_string().into_string();

    match result {
        Ok(result_str) => result_str,
        Err(_) => "Unknown directory.".to_owned()
    }
}