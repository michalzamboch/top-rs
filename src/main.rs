use thread_priority::*;
use ui::runner;

mod backend;
mod types;
mod ui;

use std::error::Error;

const MAX_THREAD_PRIORITY: bool = false;

fn main() -> Result<(), Box<dyn Error>> {
    if MAX_THREAD_PRIORITY {
        _ = set_current_thread_priority(ThreadPriority::Max);
    }

    runner::start()
}
