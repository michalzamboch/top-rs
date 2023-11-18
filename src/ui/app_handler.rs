#![allow(dead_code)]

use super::{arguments::*, paths::*, ui_handler::UiHandler};

use crate::{
    backend::app::App,
    backend::mock::MockApp,
    types::{
        enums::{
            selected_table::TableSelectionMove, sort_by::SortBy, table_position::TablePosition,
        },
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
        self.ui_update();
    }

    pub fn hard_update(&mut self) {
        self.app.hard_update();
        self.ui_update();
    }

    fn ui_update(&self) {
        self.update_processes();
        self.update_disks();
        self.update_transmitted_network();
        self.update_received_network();
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

    fn update_transmitted_network(&self) {
        let net = self.app.get_network_sum();
        let spark_line = self.ui.get_spar_line(TRASMITTED_SPARK_LINE_ID);
        spark_line.add(net.1);
    }

    fn update_received_network(&self) {
        let net = self.app.get_network_sum();
        let spark_line = self.ui.get_spar_line(RECEIVED_SPARK_LINE_ID);
        spark_line.add(net.0);
    }

    pub fn process_jump_to(&self, position: TablePosition) {
        let process_table = self.ui.get_selected_table();
        process_table.jump_to(position);
    }

    pub fn move_to_table(&self, move_to: TableSelectionMove) {
        self.ui.move_to_table(move_to);
    }

    pub fn sort_processes_by(&mut self, sort_by: SortBy) {
        self.app.sort_processes_by(sort_by);
    }

    pub fn pause_unpause(&mut self) {
        self.pause = !self.pause;
    }
}
