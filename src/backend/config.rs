#![allow(dead_code)]

use crate::types::sort_by::SortBy;

pub const INITIAL_REFRESH_COUNT: usize = 2;

pub const PID_STR_LEN: usize = 10;
pub const NAME_STR_LEN: usize = 40;
pub const NAME_STR_TRIM_LEN: usize = 35;
pub const CPU_USAGE_STR_LEN: usize = 5;
pub const PRETTY_BYTES_STR_LEN: usize = 12;

pub const MAX_PROCESS_LINE_LEN: usize =
    PID_STR_LEN + NAME_STR_LEN + NAME_STR_TRIM_LEN + CPU_USAGE_STR_LEN + 3 * PRETTY_BYTES_STR_LEN;

pub const DEFAULT_PROCESS_SORT_ORDER: SortBy = SortBy::CpuReverse;