use fast_str::FastStr;

pub trait IProcessStringView {
    fn get_pid(&self) -> String;
    fn get_name(&self) -> String;
    fn get_cpu_usage(&self) -> String;
    fn get_memory_usage(&self) -> String;
    fn get_disk_read_usage(&self) -> String;
    fn get_disk_write_usage(&self) -> String;

    fn get_pid_fstr(&self) -> FastStr {
        self.get_pid().into()
    }

    fn get_name_fstr(&self) -> FastStr {
        self.get_name().into()
    }

    fn get_cpu_usage_fstr(&self) -> FastStr {
        self.get_cpu_usage().into()
    }

    fn get_memory_usage_fstr(&self) -> FastStr {
        self.get_memory_usage().into()
    }

    fn get_disk_read_usage_fstr(&self) -> FastStr {
        self.get_disk_read_usage().into()
    }

    fn get_disk_write_usage_fstr(&self) -> FastStr {
        self.get_disk_write_usage().into()
    }
}
