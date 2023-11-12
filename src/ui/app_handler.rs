use super::{paths::*, ui_handler::UiHandler};

use crate::{
    backend::app::App,
    backend::mock::MockApp,
    types::{
        enums::sort_by::SortBy,
        traits::{
            app::IApp, app_accessor::IAppAccessor, creatable::ICreatable, ui_handler::IUiHandler,
        },
    },
};

#[derive(Debug)]
pub struct AppHandler {
    ui: Box<dyn IUiHandler>,
    app: Box<dyn IApp>,
    pause: bool,
}

impl IAppAccessor for AppHandler {
    fn get_app(&self) -> &dyn IApp {
        self.app.as_ref()
    }

    fn get_ui(&self) -> &dyn IUiHandler {
        self.ui.as_ref()
    }
}

impl AppHandler {
    pub fn new(use_mock: bool) -> AppHandler {
        let app: Box<dyn IApp> = match use_mock {
            true => MockApp::new_boxed(),
            false => App::new_boxed(),
        };

        AppHandler {
            ui: UiHandler::new_boxed(),
            app,
            pause: false,
        }
    }

    pub fn update(&mut self) {
        if self.pause {
            return;
        }

        self.app.on_tick();
        self.update_processes();
    }

    fn update_processes(&self) {
        let processes = self.app.get_filtered_processes_vec_strings();
        let process_table = self.ui.get_table_handler(PROCESSES_TABLE_ID);
        process_table.set_data(processes);
    }
    pub fn process_down(&self) {
        let process_table = self.ui.get_table_handler(PROCESSES_TABLE_ID);
        process_table.next();
    }

    pub fn process_up(&self) {
        let process_table = self.ui.get_table_handler(PROCESSES_TABLE_ID);
        process_table.previous();
    }

    pub fn first_process(&self) {
        let process_table = self.ui.get_table_handler(PROCESSES_TABLE_ID);
        process_table.first();
    }

    pub fn last_process(&self) {
        let process_table = self.ui.get_table_handler(PROCESSES_TABLE_ID);
        process_table.last();
    }

    pub fn sort_processes_by(&mut self, sort_by: SortBy) {
        self.app.sort_processes_by(sort_by);
    }

    pub fn pause_unpause(&mut self) {
        self.pause = !self.pause;
    }
}
