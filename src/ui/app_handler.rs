#![allow(dead_code)]

use super::{arguments::*, paths::*, ui_handler::UiHandler};

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
    cli_args: Box<Arguments>,
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
    pub fn new() -> AppHandler {
        let arguments = Arguments::new_boxed();

        let app: Box<dyn IApp> = match arguments.debug {
            true => MockApp::new_boxed(),
            false => App::new_boxed(),
        };

        AppHandler {
            ui: UiHandler::new_boxed(),
            app,
            cli_args: arguments,
            pause: false,
        }
    }

    pub fn update(&mut self) {
        if self.pause {
            return;
        }

        self.app.update();
        self.update_processes();
        self.update_disks();
    }

    pub fn hard_update(&mut self) {
        self.app.hard_update();
        self.update_processes();
        self.update_disks();
    }

    fn update_processes(&self) {
        let processes = self.app.get_filtered_processes_vec_strings();
        let process_table = self.ui.get_table_handler(PROCESSES_TABLE_ID);
        process_table.set_data(processes);
    }

    fn update_disks(&self) {
        let disks = self.app.get_disks_vec_string();
        let disk_table = self.ui.get_table_handler(DISKS_TABLE_ID);
        disk_table.set_data(disks);
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
