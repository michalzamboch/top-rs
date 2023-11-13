use clap::*;

use crate::types::traits::creatable::ICreatable;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
#[derive(Debug, Default)]
pub struct Arguments {
    #[arg(short = 'd', value_name = "DEBUG", long = "debug")]
    pub debug: bool,
}

impl ICreatable for Arguments {
    fn new() -> Self {
        Arguments::parse()
    }
}
