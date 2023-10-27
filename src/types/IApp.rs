

pub trait IApp : Sized {
    fn on_tick(&mut self);
    fn get_memory_usage(&self) -> u64 ;
    fn get_core_usage(&self) -> Vec<u64>;
    fn get_total_cpu_usage(&self) -> u64;
    fn print_process_info(&self);
}
