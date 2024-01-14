use std::rc::Rc;

use crate::types::traits::{
    arguments::IArguments, creatable::ICreatable, logger::ILogger, services::IServices,
};

use super::{arguments::Arguments, logger::Logger};

#[derive(Debug, Default)]
struct Services {
    arguments: Rc<Arguments>,
    logger: Rc<Logger>,
}

pub fn create_services() -> Rc<Box<dyn IServices>> {
    Rc::new(Services::new_boxed())
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
