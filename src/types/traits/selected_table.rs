use std::fmt::Debug;

pub trait ISelectedTable: Send + Debug {
    fn get(&self) -> String;
    fn register_vec(&self, ids: Vec<&str>);
    fn register(&self, name: &str);
    fn next(&self);
    fn previous(&self);
}
