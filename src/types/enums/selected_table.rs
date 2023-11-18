#[derive(Default, Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub enum TableSelectionMove {
    #[default]
    Next,
    Previous,
}
