#[derive(Default, Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub enum TableSelectionMove {
    #[default]
    Next,
    Previous,
}

impl ToString for TableSelectionMove {
    #[inline]
    fn to_string(&self) -> String {
        match self {
            TableSelectionMove::Next => "Next".to_string(),
            TableSelectionMove::Previous => "Previous".to_string(),
        }
    }
}
