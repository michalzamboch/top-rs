use std::fmt::Debug;

pub trait ISparkLineHandler: Send + Debug {
    fn add(&self, value: u64);
    fn get_vec(&self) -> Vec<u64>;
    fn init(&self, value: u64);
}
