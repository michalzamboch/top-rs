#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub enum SortBy {
    Pid,
    PidReverse,
    Name,
    NameReverse,
    Cpu,
    CpuReverse,
    Memory,
    MemoryReverse,
    DiskRead,
    DiskReadReverse,
    DiskWrite,
    DiskWriteReverse,
}
