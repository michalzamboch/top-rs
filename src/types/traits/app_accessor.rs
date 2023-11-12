use std::fmt::Debug;

use super::{app::IApp, ui_handler::IUiHandler};

pub trait IAppAccessor: Debug {
    fn get_app(&self) -> &dyn IApp;
    fn get_ui(&self) -> &dyn IUiHandler;
}
