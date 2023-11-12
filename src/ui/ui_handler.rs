use std::{collections::HashMap, rc::Rc};

use super::{controls::table_handler::TableHandler, paths::*};
use crate::types::traits::{
    creatable::ICreatable, table_handler::ITableHandler, ui_handler::IUiHandler,
};

type TableHandlerMapElement = Rc<TableHandler>;
type TableHandlerMap = HashMap<String, TableHandlerMapElement>;

#[derive(Debug, Default)]
pub struct UiHandler {
    table_handler_map: TableHandlerMap,
}

impl UiHandler {
    fn create_table_map() -> TableHandlerMap {
        let mut table_map = HashMap::new();

        table_map.insert(
            PROCESSES_TABLE_ID.to_owned(),
            Rc::new(TableHandler::default()),
        );

        table_map.insert(DISKS_TABLE_ID.to_owned(), Rc::new(TableHandler::default()));

        table_map
    }
}

impl ICreatable for UiHandler {
    fn new() -> UiHandler {
        UiHandler {
            table_handler_map: Self::create_table_map(),
        }
    }
}

impl IUiHandler for UiHandler {
    fn get_table_handler(&self, id: &str) -> Rc<dyn ITableHandler> {
        let element = self.table_handler_map.get(id);

        match element {
            Some(element_ref) => element_ref.clone(),
            None => panic!("\"{}\": Unknown table handler.", id),
        }
    }
}
