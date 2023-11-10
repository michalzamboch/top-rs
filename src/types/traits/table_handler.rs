use std::{cell::RefMut, fmt::Debug};

use ratatui::widgets::TableState;

pub trait ITableHandler: Send + Debug {
    fn first(&self);
    fn last(&self);
    fn next(&self);
    fn previous(&self);
    fn set_data(&self, data: Vec<Vec<String>>);
    fn get_data(&self) -> Vec<Vec<String>>;
    fn get_state(&self) -> RefMut<'_, TableState>;
}
