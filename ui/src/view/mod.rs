mod common;
mod home;
mod loading;
mod settings;

pub use crate::view::common::{get_default_view, ViewName};
pub use crate::view::{home::Home, loading::Loading, settings::Settings};
