use makepad_widgets::*;

pub mod input;
pub mod button;
pub mod text;
pub mod view;
pub mod icon_button;

pub mod modal;
pub mod day_of_week;
pub mod group;
pub mod tabs;
pub mod check;
pub mod details;
pub mod wheel;
pub mod time_picker;
pub mod date_picker;

pub fn live_design(cx: &mut Cx) {
    text::live_design(cx);
    button::live_design(cx);
    input::live_design(cx);
    view::live_design(cx);
    icon_button::live_design(cx);
    modal::live_design(cx);
    group::live_design(cx);
    day_of_week::live_design(cx);
    tabs::live_design(cx);
    check::live_design(cx);
    details::live_design(cx);
    wheel::live_design(cx);
    time_picker::live_design(cx);
    date_picker::live_design(cx);
}

