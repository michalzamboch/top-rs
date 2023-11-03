pub trait IProcessStringView {
    fn get_pid(&self) -> String;
    fn get_name(&self) -> String;
    fn get_cpu_usage(&self) -> String;
    fn get_memory_usage(&self) -> String;
    fn get_disk_read_usage(&self) -> String;
    fn get_disk_write_usage(&self) -> String;

    fn to_string(&self) -> String {
        format!(
            "{} {} {} {} {} {}",
            self.get_pid(),
            self.get_name(),
            self.get_cpu_usage(),
            self.get_memory_usage(),
            self.get_disk_read_usage(),
            self.get_disk_write_usage(),
        )
    }
}
