#![allow(clippy::wrong_self_convention)]

use fast_str::FastStr;

pub trait IProcessStringView {
    fn get_pid(&self) -> String;
    fn get_name(&self) -> String;
    fn get_cpu_usage(&self) -> String;
    fn get_memory_usage(&self) -> String;
    fn get_disk_read_usage(&self) -> String;
    fn get_disk_write_usage(&self) -> String;

    fn into_string_vec(&self) -> Vec<String> {
        vec![
            self.get_pid(),
            self.get_name(),
            self.get_cpu_usage(),
            self.get_memory_usage(),
            self.get_disk_read_usage(),
            self.get_disk_write_usage(),
        ]
    }

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

    fn into_fstring_vec(&self) -> Vec<FastStr> {
        vec![
            self.get_pid_fstr(),
            self.get_name_fstr(),
            self.get_cpu_usage_fstr(),
            self.get_memory_usage_fstr(),
            self.get_disk_read_usage_fstr(),
            self.get_disk_write_usage_fstr(),
        ]
    }
}
