use std::{collections::HashMap, rc::Rc};

use super::{
    config,
    controls::{
        selected_table::SelectedTable, spark_line_handler::SparkLineHandler,
        status_handler::StatusHandler, table_handler::TableHandler,
    },
    paths::*,
};
use crate::types::traits::{
    creatable::ICreatable, selected_table::ISelectedTable, spark_line_handler::ISparkLineHandler,
    status_handler::IStatusHandler, table_handler::ITableHandler, ui_handler::IUiHandler,
};

type TableHandlerMapElement = Rc<TableHandler>;
type TableHandlerMap = HashMap<String, TableHandlerMapElement>;

type SparkLineMapElement = Rc<SparkLineHandler>;
type SparkLineMap = HashMap<String, SparkLineMapElement>;

type TableSelection = Rc<SelectedTable>;

type Status = Rc<StatusHandler>;

#[derive(Debug, Default)]
pub struct UiHandler {
    table_handler_map: TableHandlerMap,
    spark_line_map: SparkLineMap,
    table_selection: TableSelection,
    status: Status,
}

impl UiHandler {
    fn create_table_map() -> TableHandlerMap {
        let mut table_map = HashMap::new();

        table_map.insert(PROCESSES_TABLE_ID.to_owned(), TableHandler::new_rc());
        table_map.insert(DISKS_TABLE_ID.to_owned(), TableHandler::new_rc());

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

    fn create_table_selection() -> Rc<SelectedTable> {
        let selected_table = SelectedTable::new();
        selected_table.register_vec(vec![PROCESSES_TABLE_ID]);
        Rc::new(selected_table)
    }

    fn create_status() -> Status {
        let status_handler = StatusHandler::new();
        status_handler.set(config::WELCOME_MESSAGE);
        Rc::new(status_handler)
    }
}

impl ICreatable for UiHandler {
    fn new() -> UiHandler {
        UiHandler {
            table_handler_map: Self::create_table_map(),
            spark_line_map: Self::create_spark_line_map(),
            table_selection: Self::create_table_selection(),
            status: Self::create_status(),
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

    fn get_table_selection(&self) -> Rc<dyn ISelectedTable> {
        self.table_selection.clone()
    }

    fn get_selected_table(&self) -> Rc<dyn ITableHandler> {
        let id = self.table_selection.get();
        self.get_table_handler(id.as_str())
    }

    fn get_status(&self) -> Rc<dyn IStatusHandler> {
        self.status.clone()
    }
}
