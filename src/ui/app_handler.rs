#![allow(dead_code)]

use std::rc::Rc;

use super::{paths::*, ui_handler::UiHandler};
use thread_priority::*;

use crate::{
    backend::system::app::App,
    backend::{services::service_handler::Services, system::mock::MockApp},
    types::{
        enums::{
            selected_table::TableSelectionMove,
            sort_by::SortBy,
            table_commands::{ProcessCommand, TableCommand},
            table_position::TablePosition,
        },
        traits::{
            app::IApp, app_accessor::IAppAccessor, creatable::ICreatable, services::IServices,
            ui_handler::IUiHandler,
        },
    },
};

#[derive(Debug)]
pub struct AppHandler {
    ui: Box<dyn IUiHandler>,
    app: Box<dyn IApp>,
    services: Rc<Services>,
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
        let tmp_services = Services::new_rc();
        if tmp_services.get_arguments().get_max_priority() {
            _ = set_current_thread_priority(ThreadPriority::Max);
        }

        let app: Box<dyn IApp> = match tmp_services.get_arguments().get_debug() {
            true => MockApp::new_boxed(),
            false => App::new_boxed(),
        };

        AppHandler {
            ui: UiHandler::new_boxed(),
            app,
            services: tmp_services.clone(),
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
        self.update_transmitted_network();
        self.update_received_network();
    }

    fn update_processes(&self) {
        let processes = self.app.get_process_data_holder();
        let process_table = self.ui.get_table_handler(PROCESSES_TABLE_ID);
        process_table.set_data(processes);
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

    pub fn selected_table_jump_to(&self, position: TablePosition) {
        let process_table = self.ui.get_selected_table();
        process_table.jump_to(position);
    }

    pub fn move_to_table(&self, move_to: TableSelectionMove) {
        self.ui.get_table_selection().move_to(move_to);
    }

    pub fn sort_processes_by(&mut self, sort_by: SortBy) {
        self.app.sort_processes_by(sort_by);
    }

    pub fn pause_unpause(&mut self) {
        self.pause = !self.pause;
    }

    pub fn kill_process(&self) {
        let process_table = self.ui.get_table_handler(PROCESSES_TABLE_ID);
        let result = process_table.execute(TableCommand::Process(ProcessCommand::KillProcess));

        let status = self.ui.get_status();
        match result {
            Ok(_) => status.set("Process successfully killed."),
            Err(_) => status.set("Process could not be killed."),
        }
    }
}
