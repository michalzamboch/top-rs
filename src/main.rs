use ui::view;

mod ui;

use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    view::start()
}
