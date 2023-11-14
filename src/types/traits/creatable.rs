use std::{cell::RefCell, fmt::Debug, rc::Rc};

pub trait ICreatable: Debug + Default {
    fn new() -> Self;

    fn new_boxed() -> Box<Self> {
        Box::new(Self::new())
    }

    fn new_rc() -> Rc<Self> {
        Rc::new(Self::new())
    }

    fn new_ref() -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(Self::new()))
    }

    fn new_ref_optional() -> Rc<RefCell<Option<Self>>> {
        Rc::new(RefCell::new(Some(Self::new())))
    }
}
