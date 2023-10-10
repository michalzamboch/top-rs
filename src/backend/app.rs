
use sysinfo::{System, SystemExt};

pub struct App {
    sys: System,
}

impl App {
    pub fn new() -> App {
        App {
            sys: System::new(),
        }
    }

    pub fn on_tick(&mut self) {
        self.sys.refresh_memory();
    }

    pub fn get_memory_usage(&self) -> u64 {
        get_floored_percentage(self.sys.used_memory(), self.sys.total_memory())
    }
}

fn get_floored_percentage(part: u64, total: u64) -> u64 {
    let mut percentage = part as f64 * 100.0;
    percentage /= total as f64;

    percentage.floor() as u64
}
