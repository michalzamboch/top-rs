#[allow(dead_code)]
#[derive(Default, Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub enum TablePosition {
    #[default]
    Down,
    Up,
    First,
    Last,
    PageDown,
    PageUp,
}
