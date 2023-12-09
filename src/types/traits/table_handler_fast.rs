use std::{cell::RefMut, fmt::Debug};

use ratatui::widgets::TableState;

use crate::types::enums::table_position::TablePosition;

use super::table_data_holder::ITableDataHolder;

pub trait ITableHandlerFast: Send + Debug {
    fn first(&self);
    fn last(&self);
    fn next(&self);
    fn previous(&self);
    fn jump_to(&self, position: TablePosition);
    fn set_data(&self, data: Box<dyn ITableDataHolder>);
    fn get_data(&self) -> Vec<Vec<String>>;
    fn get_state(&self) -> RefMut<'_, TableState>;
}
