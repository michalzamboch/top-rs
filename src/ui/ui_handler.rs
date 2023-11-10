#![allow(dead_code)]

use std::{cell::RefCell, collections::HashMap, rc::Rc};

use super::ui_controls::table_handler::TableHandler;
use crate::types::traits::{table_handler::ITableHandler, ui_handler::IUiHandler};

type TableHandlerMapElement = Rc<RefCell<TableHandler>>;

type TableHandlerMap = HashMap<String, TableHandlerMapElement>;

#[derive(Debug)]
pub struct UiHandler {
    table_handler_map: TableHandlerMap,
}

impl UiHandler {
    pub fn new() -> UiHandler {
        let mut table_map = HashMap::new();
        table_map.insert("processes".to_owned(), Rc::new(RefCell::new(TableHandler::default())));

        UiHandler {
            table_handler_map: table_map,
        }
    }
}

impl IUiHandler for UiHandler {
    fn get_table_handler(&self, id: &str) -> Rc<RefCell<dyn ITableHandler>> {
        let element = self.table_handler_map.get(id);
        
        match element {
            Some(element_ref) => element_ref.clone(),
            None => panic!("Unknown table handler."),
        }
    }
}
