use std::{error::Error, fmt::Debug};

use fast_str::FastStr;

use crate::types::enums::table_commands::TableCommand;

pub trait ITableDataHolder: Send + Debug {
    fn get_data(&self) -> Box<[Vec<FastStr>]>;
    fn execute(&self, command: TableCommand, index: usize) -> Result<(), Box<dyn Error>>;

    fn len(&self) -> usize;
    fn is_empty(&self) -> bool;
}
