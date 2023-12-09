use std::fmt::Debug;

pub trait ITableDataHolder: Send + Debug {
    fn get_data(&self) -> Vec<Vec<String>>;
    fn get_box(&self) -> Box<[Vec<String>]>;

    fn len(&self) -> usize;
    fn is_empty(&self) -> bool;
}
