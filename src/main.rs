use ui::runner;

mod backend;
mod types;
mod ui;

use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    runner::start()
}
