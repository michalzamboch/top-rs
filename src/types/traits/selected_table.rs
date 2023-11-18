use std::fmt::Debug;

use crate::types::enums::selected_table::TableSelectionMove;

pub trait ISelectedTable: Send + Debug {
    fn get(&self) -> String;
    fn register_vec(&self, ids: Vec<&str>);
    fn register(&self, name: &str);
    fn move_to(&self, move_to: TableSelectionMove);
    fn next(&self);
    fn previous(&self);
}
