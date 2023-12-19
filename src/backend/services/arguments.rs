use clap::*;

use crate::types::traits::{arguments::IArguments, creatable::ICreatable};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
#[derive(Debug, Default)]
pub struct Arguments {
    #[arg(short = 'd', value_name = "DEBUG", long = "debug")]
    pub debug: bool,

    #[arg(short = 'm', value_name = "MAX THREAD PRIORITY", long = "max-priority")]
    pub max_thread_priority: bool,

    #[arg(short = 'l', value_name = "LOG", long = "log")]
    pub log: bool,
}

impl ICreatable for Arguments {
    fn new() -> Self {
        Arguments::parse()
    }
}

impl IArguments for Arguments {
    fn get_log(&self) -> bool {
        self.log
    }

    fn get_debug(&self) -> bool {
        self.debug
    }

    fn get_max_priority(&self) -> bool {
        self.max_thread_priority
    }
}
