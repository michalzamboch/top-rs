use super::ui_handler::UiHandler;

use crate::{backend::app::App, types::app_trait::IApp};

#[derive(Debug)]
pub struct AppHandler {
    pub ui: UiHandler,
    pub app: App,
}

impl AppHandler {
    pub fn new() -> AppHandler {
        AppHandler {
            ui: UiHandler::new(),
            app: App::new(),
        }
    }

    pub fn update(&mut self) {
        self.app.on_tick();
        //let processes = self.app.get_filtered_processes_vec_strings();
        let processes = vec![
            vec!["YYY".to_owned(), "YYY".to_owned(), "YYY".to_owned()],
            vec!["ZZZ".to_owned(), "ZZZ".to_owned(), "ZZZ".to_owned()],
            vec!["Row21".to_owned(), "Row22".to_owned(), "Row23".to_owned()],
            vec!["Row21".to_owned(), "Row22".to_owned(), "Row23".to_owned()],
            vec!["Row21".to_owned(), "Row22".to_owned(), "Row23".to_owned()],
            vec!["Row21".to_owned(), "Row22".to_owned(), "Row23".to_owned()],
            vec!["Row21".to_owned(), "Row22".to_owned(), "Row23".to_owned()],
            vec!["Row21".to_owned(), "Row22".to_owned(), "Row23".to_owned()],
            vec!["Row21".to_owned(), "Row22".to_owned(), "Row23".to_owned()],
            vec!["XXX".to_owned(), "XXX".to_owned(), "XXX".to_owned()],
        ];
        self.ui.set_process_table(processes);
    }

    pub fn process_down(&mut self) {
        self.ui.next_process();
    }

    pub fn process_up(&mut self) {
        self.ui.previous_process();
    }
}
