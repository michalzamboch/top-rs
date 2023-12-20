use std::{fmt::Debug, rc::Rc};

use super::{app::IApp, services::IServices, ui_handler::IUiHandler};

pub trait IAppAccessor: Debug {
    fn get_app(&self) -> &dyn IApp;
    fn get_ui(&self) -> &dyn IUiHandler;
    fn get_services(&self) -> Rc<dyn IServices>;
}
