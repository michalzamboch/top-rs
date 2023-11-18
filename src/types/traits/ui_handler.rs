use std::{fmt::Debug, rc::Rc};

use crate::types::enums::selected_table::TableSelectionMove;

use super::{spark_line_handler::ISparkLineHandler, table_handler::ITableHandler};

pub trait IUiHandler: Debug {
    fn get_table_handler(&self, id: &str) -> Rc<dyn ITableHandler>;
    fn get_spar_line(&self, id: &str) -> Rc<dyn ISparkLineHandler>;
    fn move_to_table(&self, move_to: TableSelectionMove);
    fn get_selected_table(&self) -> Rc<dyn ITableHandler>;
}
