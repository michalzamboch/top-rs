#![allow(dead_code)]

use std::rc::Rc;

use crate::types::traits::creatable::ICreatable;

use super::arguments::Arguments;

#[derive(Debug, Default)]
pub struct Services {
    arguments: Rc<Arguments>,
}

impl ICreatable for Services {
    fn new() -> Self {
        Services {
            arguments: Arguments::new_rc(),
        }
    }
}
