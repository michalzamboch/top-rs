
#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub enum SortBy {
    Pid,
    Name,
    Cpu,
    Memory,
    DiskRead,
    DiskWrite,
}