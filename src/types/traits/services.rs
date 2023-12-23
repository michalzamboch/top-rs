use std::{fmt::Debug, rc::Rc};

use super::{arguments::IArguments, logger::ILogger};

pub trait IServices: Debug {
    fn logger(&self) -> Rc<dyn ILogger>;
    fn arguments(&self) -> Rc<dyn IArguments>;
}
