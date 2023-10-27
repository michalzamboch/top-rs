use ui::runner;

mod ui;
mod backend;
mod types;

use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    runner::start()
}
