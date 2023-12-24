use sysinfo::*;

pub fn get_sys_load(sys: &System) -> String {
    let load_avg = &sys.load_average();
    format!(
        "1 min: {}%, 5 min: {}%, 15 min: {}%",
        load_avg.one, load_avg.five, load_avg.fifteen,
    )
}
