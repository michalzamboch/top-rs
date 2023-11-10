use super::ui_handler::UiHandler;

use crate::{
    backend::app::App,
    backend::mock::MockApp,
    types::{
        enums::sort_by::SortBy,
        traits::{app::IApp, ui_handler::IUiHandler, creatable::ICreatable},
    },
};

#[derive(Debug)]
pub struct AppHandler {
    ui: Box<UiHandler>,
    app: Box<dyn IApp>,
    pause: bool,
}

impl AppHandler {
    pub fn new(use_mock: bool) -> AppHandler {
        let app: Box<dyn IApp> = match use_mock {
            true => MockApp::new_boxed(),
            false => App::new_boxed(),
        };

        AppHandler {
            ui: Box::new(UiHandler::new()),
            app,
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
