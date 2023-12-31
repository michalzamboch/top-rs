use std::{fmt::Debug, rc::Rc};

use super::{
    selected_table::ISelectedTable, spark_line_handler::ISparkLineHandler,
    status_handler::IStatusHandler, table_handler::ITableHandler,
};

pub trait IUiHandler: Debug {
    fn get_table_handler(&self, id: &str) -> Rc<dyn ITableHandler>;
    fn get_spar_line(&self, id: &str) -> Rc<dyn ISparkLineHandler>;
    fn get_table_selection(&self) -> Rc<dyn ISelectedTable>;
    fn get_selected_table(&self) -> Rc<dyn ITableHandler>;
    fn get_status(&self) -> Rc<dyn IStatusHandler>;
}
