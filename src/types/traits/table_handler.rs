use fast_str::FastStr;
use ratatui::widgets::TableState;
use std::{cell::RefMut, fmt::Debug};

use super::table_data_holder::ITableDataHolder;
use crate::types::enums::table_position::TablePosition;

pub trait ITableHandler: Send + Debug {
    fn first(&self);
    fn last(&self);
    fn next(&self);
    fn previous(&self);
    fn jump_to(&self, position: TablePosition);
    fn get_state(&self) -> RefMut<'_, TableState>;
    fn set_data(&self, data: Box<dyn ITableDataHolder>);
    fn get_data(&self) -> Box<[Vec<FastStr>]>;
}
