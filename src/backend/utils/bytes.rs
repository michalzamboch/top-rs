#![allow(dead_code)]

use fast_str::FastStr;
use std::cmp;

pub fn convert_fstr(num: f64, regular_delimiter: bool) -> FastStr {
    FastStr::from_string(convert(num, regular_delimiter))
}

pub fn convert(num: f64, regular_delimiter: bool) -> String {
    let delimiter = match regular_delimiter {
        true => 1024_f64,
        false => 1000_f64,
    };

    convert_with_delimiter(num, delimiter)
}

fn convert_with_delimiter(num: f64, delimiter: f64) -> String {
    let negative = if num.is_sign_positive() { "" } else { "-" };
    let num = num.abs();
    let units = ["B", "kB", "MB", "GB", "TB", "PB", "EB", "ZB", "YB"];

    if num < 1_f64 {
        return format!("{}{} {}", negative, num, "B");
    }

    let exponent = cmp::min(
        (num.ln() / delimiter.ln()).floor() as i32,
        (units.len() - 1) as i32,
    );

    let pretty_bytes = format!("{:.2}", num / delimiter.powi(exponent))
        .parse::<f64>()
        .unwrap()
        * 1_f64;
    let unit = units[exponent as usize];
    format!("{}{} {}", negative, pretty_bytes, unit)
}
