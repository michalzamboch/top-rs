pub trait ITemperature: Send {
    fn value(&self) -> String;
    fn max(&self) -> String;
    fn overheating(&self) -> bool;
}
