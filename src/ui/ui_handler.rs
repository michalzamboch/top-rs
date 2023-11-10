#![allow(dead_code)]

use ratatui::widgets::TableState;
use std::cell::RefMut;

use super::ui_controls::table_handler::TableHandler;
use crate::types::traits::{table_handler::ITableHandler, ui_handler::IUiHandler};

#[derive(Debug)]
pub struct UiHandler {
    pub processes: Box<dyn ITableHandler>,
    pub temperatures: Box<dyn ITableHandler>,
    pub networks: Box<dyn ITableHandler>,
}

impl UiHandler {
    pub fn new() -> UiHandler {
        UiHandler {
            processes: Box::<TableHandler>::default(),
            temperatures: Box::<TableHandler>::default(),
            networks: Box::<TableHandler>::default(),
        }
    }
}

impl IUiHandler for UiHandler {
    fn next_process(&mut self) {
        self.processes.next();
    }

    fn previous_process(&mut self) {
        self.processes.previous();
    }

    fn first_process(&mut self) {
        self.processes.first();
    }

    fn last_process(&mut self) {
        self.processes.last();
    }

    fn set_process_table(&mut self, data: Vec<Vec<String>>) {
        self.processes.set_data(data);
    }

    fn get_process_table(&self) -> Vec<Vec<String>> {
        self.processes.get_data()
    }

    fn get_process_table_state(&self) -> RefMut<'_, TableState> {
        self.processes.get_state()
    }

    fn set_temperature_table(&mut self, data: Vec<Vec<String>>) {
        self.temperatures.set_data(data)
    }

    fn get_temperature_table(&self) -> Vec<Vec<String>> {
        self.temperatures.get_data()
    }

    fn get_temperature_table_state(&self) -> RefMut<'_, TableState> {
        self.processes.get_state()
    }

    /*
    fn set_networks_table(&mut self, data: Vec<Vec<String>>) {
        self.temperatures.set_data(data)
    }

    fn get_temperature_table(&self) -> Vec<Vec<String>> {
        self.temperatures.get_data()
    }

    fn get_temperature_table_state(&self) -> RefMut<'_, TableState> {
        self.processes.get_state()
    }
    */
}
