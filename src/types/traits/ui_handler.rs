use std::{fmt::Debug, rc::Rc};

use super::{spark_line_handler::ISparkLineHandler, table_handler::ITableHandler};

pub trait IUiHandler: Debug {
    fn get_table_handler(&self, id: &str) -> Rc<dyn ITableHandler>;
    fn get_spar_line(&self, id: &str) -> Rc<dyn ISparkLineHandler>;
}
