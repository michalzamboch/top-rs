use std::{fmt::Debug, rc::Rc};

use super::{arguments::IArguments, logger::ILogger};

pub trait IServices: Debug {
    fn get_logger(&self) -> Rc<dyn ILogger>;
    fn get_arguments(&self) -> Rc<dyn IArguments>;
}
