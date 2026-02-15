use makepad_widgets::*;

#[derive(Clone, Debug)]
pub struct MenuItem {
    pub id: LiveId,
    pub label: String,
}

#[derive(Clone, Debug, DefaultNone)]
pub enum HeaderAction {
    None,
    SetTitle(String),
    SetMenu(Vec<MenuItem>),
    ShowBack(bool),
    BackClicked,
}

#[derive(Clone, Debug, DefaultNone)]
pub enum AppAction {
    None,
    OpenDatePicker,
    OpenTooltip { title: String, description: String },
}
