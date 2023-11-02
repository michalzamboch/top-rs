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
        //self.ui.set_process_table(processes);
    }
}
