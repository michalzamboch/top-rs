use std::ffi::*;

pub fn get_floored_percentage(part: u64, total: u64) -> u64 {
    let mut percentage = part as f64 * 100.0;
    percentage /= total as f64;

    percentage.floor() as u64
}

pub fn os_string_to_regular(os_string: &OsStr) -> String {
    let result = os_string.to_os_string().into_string();

    match result {
        Ok(result_str) => {
            if !result_str.is_empty() {
                result_str
            } else {
                "Unknown".to_owned()
            }
        }
        Err(_) => "Unknown".to_owned(),
    }
}
