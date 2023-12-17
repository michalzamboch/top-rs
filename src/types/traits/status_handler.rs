use std::fmt::Debug;

pub trait IStatusHandler: Send + Debug {
    fn set(&self, message: &str);
    fn get(&self) -> String;
    fn clear(&self);
}
