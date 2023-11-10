use std::{fmt::Debug, rc::Rc};

use super::table_handler::ITableHandler;

pub trait IUiHandler: Debug {
    fn get_table_handler(&self, id: &str) -> Rc<dyn ITableHandler>;
}
