use pretty_bytes::converter;

pub trait IProcessItem {
    fn get_pid(&self) -> usize;
    fn get_name(&self) -> String;
    fn get_cpu_usage(&self) -> u64;
    fn get_memory_usage(&self) -> u64;
    fn get_disk_read_usage(&self) -> u64;
    fn get_disk_write_usage(&self) -> u64;

    fn to_string(&self) -> String {
        let pid = format!("[{}]", self.get_pid());
        let name = self.get_name();
        let cpu_usage = format!("{}%", self.get_cpu_usage());
        let mem_usage = converter::convert(self.get_memory_usage() as f64);
        let disk_read = converter::convert(self.get_disk_read_usage() as f64);
        let disk_write = converter::convert(self.get_disk_write_usage() as f64);

        format!(
            "{} {} {} {} {} {}",
            pid, name, cpu_usage, mem_usage, disk_read, disk_write,
        )
    }
}
