use std::rc::Rc;

use crate::types::traits::{
    arguments::IArguments, creatable::ICreatable, logger::ILogger, services::IServices,
};

use super::{arguments::Arguments, logger::Logger};

#[derive(Debug, Default)]
pub struct Services {
    arguments: Rc<Arguments>,
    logger: Rc<Logger>,
}

impl ICreatable for Services {
    fn new() -> Self {
        Services {
            arguments: Arguments::new_rc(),
            logger: Logger::new_rc(),
        }
    }
}

impl IServices for Services {
    fn logger(&self) -> Rc<dyn ILogger> {
        self.logger.clone()
    }

    fn arguments(&self) -> Rc<dyn IArguments> {
        self.arguments.clone()
    }
}
