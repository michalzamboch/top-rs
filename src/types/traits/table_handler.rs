use std::{cell::RefMut, fmt::Debug};

use ratatui::widgets::TableState;

pub trait ITableHandler: Send + Debug {
    fn first(&mut self);
    fn last(&mut self);
    fn next(&mut self);
    fn previous(&mut self);
    fn set_data(&mut self, data: Vec<Vec<String>>);
    fn get_data(&self) -> Vec<Vec<String>>;
    fn get_state(&self) -> RefMut<'_, TableState>;
}
