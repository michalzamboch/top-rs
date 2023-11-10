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
        let process_table = self.ui.get_table_handler("processes");
        process_table.borrow_mut().set_data(processes);
    }

    pub fn get_app(&self) -> &dyn IApp {
        self.app.as_ref()
    }

    pub fn get_ui(&self) -> &dyn IUiHandler {
        self.ui.as_ref()
    }

    pub fn process_down(&mut self) {
        let process_table = self.ui.get_table_handler("processes");
        process_table.borrow_mut().next();
    }

    pub fn process_up(&mut self) {
        let process_table = self.ui.get_table_handler("processes");
        process_table.borrow_mut().previous();
    }

    pub fn first_process(&mut self) {
        let process_table = self.ui.get_table_handler("processes");
        process_table.borrow_mut().first();
    }

    pub fn last_process(&mut self) {
        let process_table = self.ui.get_table_handler("processes");
        process_table.borrow_mut().last();
    }

    pub fn sort_processes_by(&mut self, sort_by: SortBy) {
        self.app.sort_processes_by(sort_by);
    }

    pub fn pause_unpause(&mut self) {
        self.pause = !self.pause;
    }
}
