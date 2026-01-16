use makepad_widgets::*;

pub mod text;

pub fn live_design(cx: &mut Cx) {
    makepad_widgets::live_design(cx);
    text::live_design(cx);
}
