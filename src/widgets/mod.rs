pub mod text;
pub mod view;

pub fn live_design(cx: &mut makepad_widgets::Cx) {
    text::live_design(cx);
    view::live_design(cx);
}
