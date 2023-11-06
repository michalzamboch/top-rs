use super::ui_handler::UiHandler;

use crate::{backend::app::App, types::{app_trait::IApp, ui_handler_trait::IUiHandler, sort_by::SortBy}};

#[derive(Debug)]
pub struct AppHandler {
    ui: Box<dyn IUiHandler>,
    app: Box<dyn IApp>,
    pause: bool,
}

impl AppHandler {
    pub fn new() -> AppHandler {
        AppHandler {
            ui: Box::new(UiHandler::new()),
            app: Box::new(App::new()),
            pause: false,
        }
    }

    pub fn update(&mut self) {
        if self.pause {
            return;
        }

        self.app.on_tick();
        let processes = self.app.get_filtered_processes_vec_strings();
        self.ui.set_process_table(processes);
    }

    pub fn get_app_ref(&self) -> &dyn IApp {
        self.app.as_ref()
    }

    pub fn get_ui_ref(&self) -> &dyn IUiHandler {
        self.ui.as_ref()
    }

    pub fn process_down(&mut self) {
        self.ui.next_process();
    }

    pub fn process_up(&mut self) {
        self.ui.previous_process();
    }

    pub fn first_process(&mut self) {
        self.ui.first_process();
    }

    pub fn last_process(&mut self) {
        self.ui.last_process();
    }

    pub fn sort_processes_by(&mut self, sort_by: SortBy) {
        self.app.sort_processes_by(sort_by);
    }

    pub fn pause_unpause(&mut self) {
        self.pause = !self.pause;
    }
}
