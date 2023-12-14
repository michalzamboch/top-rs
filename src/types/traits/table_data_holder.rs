use std::fmt::Debug;

use fast_str::FastStr;

pub trait ITableDataHolder: Send + Debug {
    fn get_data(&self) -> Box<[Vec<FastStr>]>;

    fn len(&self) -> usize;
    fn is_empty(&self) -> bool;
}
