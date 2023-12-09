use std::{fmt::Debug, rc::Rc};

use super::{
    selected_table::ISelectedTable, spark_line_handler::ISparkLineHandler,
    table_handler_fast::ITableHandlerFast,
};

pub trait IUiHandler: Debug {
    fn get_table_handler(&self, id: &str) -> Rc<dyn ITableHandlerFast>;
    fn get_spar_line(&self, id: &str) -> Rc<dyn ISparkLineHandler>;
    fn get_table_selection(&self) -> &dyn ISelectedTable;
    fn get_selected_table(&self) -> Rc<dyn ITableHandlerFast>;
}
