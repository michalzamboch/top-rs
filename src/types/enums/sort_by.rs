#[derive(Default, Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub enum SortBy {
    Pid,
    PidReverse,
    Name,
    NameReverse,
    Cpu,
    #[default]
    CpuReverse,
    Memory,
    MemoryReverse,
    DiskRead,
    DiskReadReverse,
    DiskWrite,
    DiskWriteReverse,
}

impl ToString for SortBy {
    #[inline]
    fn to_string(&self) -> String {
        match self {
            SortBy::Pid => "Pid".to_string(),
            SortBy::PidReverse => "PidReverse".to_string(),
            SortBy::Name => "Name".to_string(),
            SortBy::NameReverse => "NameReverse".to_string(),
            SortBy::Cpu => "Cpu".to_string(),
            SortBy::CpuReverse => "CpuReverse".to_string(),
            SortBy::Memory => "Memory".to_string(),
            SortBy::MemoryReverse => "MemoryReverse".to_string(),
            SortBy::DiskRead => "DiskRead".to_string(),
            SortBy::DiskReadReverse => "DiskReadReverse".to_string(),
            SortBy::DiskWrite => "DiskWrite".to_string(),
            SortBy::DiskWriteReverse => "DiskWriteReverse".to_string(),
        }
    }
}
