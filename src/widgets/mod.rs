use makepad_widgets::*;

pub mod input;
pub mod button;
pub mod text;
pub mod view;

pub mod modal;
pub mod wrapper;
pub mod day_of_week;
pub mod group;

pub fn live_design(cx: &mut Cx) {
    text::live_design(cx);
    button::live_design(cx);
    input::live_design(cx);
    view::live_design(cx);
    modal::live_design(cx);
    wrapper::live_design(cx);
    group::live_design(cx);
    day_of_week::live_design(cx);
}
