use ui::runner;

mod ui;
mod backend;

use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    runner::start()
}
