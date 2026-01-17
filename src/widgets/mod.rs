pub mod input;
pub mod button;
pub mod text;
pub mod view;

pub fn live_design(cx: &mut makepad_widgets::Cx) {
    input::live_design(cx);
    button::live_design(cx);
    text::live_design(cx);
    view::live_design(cx);
}
