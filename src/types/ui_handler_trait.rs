use std::{cell::RefMut, fmt::Debug};

use ratatui::widgets::TableState;

pub trait IUiHandler: Send + Debug {
    fn next_process(&mut self);
    fn previous_process(&mut self);
    fn first_process(&mut self);
    fn last_process(&mut self);
    fn set_process_table(&mut self, data: Vec<Vec<String>>);
    fn get_process_table(&self) -> Vec<Vec<String>>;
    fn get_process_table_state(&self) -> RefMut<'_, TableState>;
    fn set_temperature_table(&mut self, data: Vec<Vec<String>>);
    fn get_temperature_table(&self) -> Vec<Vec<String>>;
    fn get_temperature_table_state(&self) -> RefMut<'_, TableState>;
}
