use std::{collections::HashMap, rc::Rc};

use super::{
    controls::{
        selected_table::SelectedTable, spark_line_handler::SparkLineHandler,
        table_handler_fast::TableHandlerFast,
    },
    paths::*,
};
use crate::types::traits::{
    creatable::ICreatable, selected_table::ISelectedTable, spark_line_handler::ISparkLineHandler,
    table_handler::ITableHandler, ui_handler::IUiHandler,
};

type TableHandlerMapElement = Rc<TableHandlerFast>;
type TableHandlerMap = HashMap<String, TableHandlerMapElement>;

type SparkLineMapElement = Rc<SparkLineHandler>;
type SparkLineMap = HashMap<String, SparkLineMapElement>;

#[derive(Debug, Default)]
pub struct UiHandler {
    table_handler_map: TableHandlerMap,
    spark_line_map: SparkLineMap,
    table_selection: SelectedTable,
}

impl UiHandler {
    fn create_table_map() -> TableHandlerMap {
        let mut table_map = HashMap::new();

        table_map.insert(PROCESSES_TABLE_ID.to_owned(), TableHandlerFast::new_rc());
        table_map.insert(DISKS_TABLE_ID.to_owned(), TableHandlerFast::new_rc());

        table_map
    }

    fn create_spark_line_map() -> SparkLineMap {
        let mut spark_line_map = HashMap::new();

        let spark_line_received = SparkLineHandler::new_rc();
        spark_line_map.insert(RECEIVED_SPARK_LINE_ID.to_owned(), spark_line_received);

        let spark_line_transmitted = SparkLineHandler::new_rc();
        spark_line_map.insert(TRASMITTED_SPARK_LINE_ID.to_owned(), spark_line_transmitted);

        spark_line_map
    }

    fn create_table_selection() -> SelectedTable {
        let selected_table = SelectedTable::new();
        selected_table.register_vec(vec![PROCESSES_TABLE_ID]);
        selected_table
    }
}

impl ICreatable for UiHandler {
    fn new() -> UiHandler {
        UiHandler {
            table_handler_map: Self::create_table_map(),
            spark_line_map: Self::create_spark_line_map(),
            table_selection: Self::create_table_selection(),
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

    fn get_spar_line(&self, id: &str) -> Rc<dyn ISparkLineHandler> {
        let element = self.spark_line_map.get(id);

        match element {
            Some(element_ref) => element_ref.clone(),
            None => panic!("\"{}\": Unknown spark line handler.", id),
        }
    }

    fn get_table_selection(&self) -> &dyn ISelectedTable {
        &self.table_selection
    }

    fn get_selected_table(&self) -> Rc<dyn ITableHandler> {
        let id = self.table_selection.get();
        self.get_table_handler(id.as_str())
    }
}
