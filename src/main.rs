use ui::view;

mod ui;
mod backend;

use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    view::start()
}
