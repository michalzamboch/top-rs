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

impl ToString for TablePosition {
    #[inline]
    fn to_string(&self) -> String {
        match self {
            TablePosition::Down => "Down".to_string(),
            TablePosition::Up => "Up".to_string(),
            TablePosition::First => "First".to_string(),
            TablePosition::Last => "Last".to_string(),
            TablePosition::PageDown => "PageDown".to_string(),
            TablePosition::PageUp => "PageUp".to_string(),
        }
    }
}
