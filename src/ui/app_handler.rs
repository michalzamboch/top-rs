
use super::ui_handler::UiHandler;

use crate::backend::app::App;

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
}
