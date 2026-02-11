use makepad_widgets::*;

#[derive(Clone, Debug, DefaultNone)]
pub enum HeaderAction {
    None,
    SetTitle(String),
}
