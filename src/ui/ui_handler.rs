#![allow(dead_code)]

use ratatui::widgets::*;

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct UiHandler {
    process_table_state: TableState,
    process_table: Vec<Vec<String>>,
}

impl UiHandler {
    pub fn new() -> UiHandler {
        UiHandler {
            process_table_state: TableState::default(),
            process_table: vec![
                vec!["Row11".to_owned(), "Row12".to_owned(), "Row13".to_owned()],
                vec!["Row21".to_owned(), "Row22".to_owned(), "Row23".to_owned()],
                vec!["Row21".to_owned(), "Row22".to_owned(), "Row23".to_owned()],
                vec!["Row21".to_owned(), "Row22".to_owned(), "Row23".to_owned()],
                vec!["Row21".to_owned(), "Row22".to_owned(), "Row23".to_owned()],
                vec!["Row21".to_owned(), "Row22".to_owned(), "Row23".to_owned()],
                vec!["Row21".to_owned(), "Row22".to_owned(), "Row23".to_owned()],
            ],
        }
    }

    pub fn next(&mut self) {
        let i = match self.process_table_state.selected() {
            Some(i) => {
                if i >= self.process_table.len() - 1 {
                    0
                } else {
                    i + 1
                }
            }
            None => 0,
        };
        self.process_table_state.select(Some(i));
    }

    pub fn previous(&mut self) {
        let i = match self.process_table_state.selected() {
            Some(i) => {
                if i == 0 {
                    self.process_table.len() - 1
                } else {
                    i - 1
                }
            }
            None => 0,
        };
        self.process_table_state.select(Some(i));
    }
}
