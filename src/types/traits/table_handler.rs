use fast_str::FastStr;
use ratatui::widgets::TableState;
use std::{cell::RefMut, error::Error, fmt::Debug};

use super::table_data_holder::ITableDataHolder;
use crate::types::enums::{table_commands::TableCommand, table_position::TablePosition};

pub trait ITableHandler: Send + Debug {
    fn jump_to(&self, position: TablePosition);
    fn get_state(&self) -> RefMut<'_, TableState>;
    fn set_data(&self, data: Box<dyn ITableDataHolder>);
    fn get_data(&self) -> Box<[Vec<FastStr>]>;
    fn execute(&self, command: TableCommand) -> Result<(), Box<dyn Error>>;
}
