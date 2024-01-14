use crate::types::traits::{app::IApp, creatable::ICreatable};

use super::{app::App, mock::MockApp};

pub fn create_app(mock: bool) -> Box<dyn IApp> {
    match mock {
        true => MockApp::new_boxed(),
        false => App::new_boxed(),
    }
}
