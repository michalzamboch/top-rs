#![allow(dead_code)]

pub const INITIAL_REFRESH_COUNT: usize = 2;

pub const PID_STR_LEN: usize = 10;
pub const NAME_STR_LEN: usize = 40;
pub const NAME_STR_TRIM_LEN: usize = 35;
pub const CPU_USAGE_STR_LEN: usize = 5;
pub const PRETTY_BZTES_STR_LEN: usize = 12;

pub const MAX_PROCESS_LINE_LEN: usize =
    PID_STR_LEN + NAME_STR_LEN + NAME_STR_TRIM_LEN + CPU_USAGE_STR_LEN + 3 * PRETTY_BZTES_STR_LEN;
