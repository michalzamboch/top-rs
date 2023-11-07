use thread_priority::*;
use ui::runner;

mod backend;
mod types;
mod ui;

use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    _ = set_current_thread_priority(ThreadPriority::Max);

    runner::start()
}
