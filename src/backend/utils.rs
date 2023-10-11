
pub fn get_floored_percentage(part: u64, total: u64) -> u64 {
    let mut percentage = part as f64 * 100.0;
    percentage /= total as f64;

    percentage.floor() as u64
}