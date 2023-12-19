use std::fmt::Debug;

pub trait IArguments: Debug + Send {
    fn get_log(&self) -> bool;
    fn get_debug(&self) -> bool;
    fn get_max_priority(&self) -> bool;
}
