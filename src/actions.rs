use makepad_widgets::*;

// Actions that screens emit to communicate with the main app
// (e.g., to open global modals like DatePicker, TimePicker, Tooltip)
#[derive(Clone, Debug, DefaultNone)]
pub enum ScreenAction {
    None,
    OpenDatePicker,
    OpenTimePicker,
    OpenTooltip { title: String, description: String },
}
