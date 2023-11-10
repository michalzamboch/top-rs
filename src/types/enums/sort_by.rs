

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
