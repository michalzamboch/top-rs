use std::fmt::Debug;

pub trait IStringsLine: Debug + Send {
    fn get_line(&self) -> Box<[String]>;
}
