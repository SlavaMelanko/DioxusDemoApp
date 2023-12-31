#[derive(Debug, PartialEq, Copy, Clone)]
pub enum ViewName {
    Loading,
    Home,
    Settings,
    Error,
}

pub fn get_default_view() -> ViewName {
    ViewName::Home
}
