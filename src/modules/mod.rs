use makepad_widgets::*;

pub mod activity;
pub mod journal;
pub mod stat;
pub mod time;
pub mod collection;

pub fn live_design(cx: &mut Cx) {
    activity::live_design(cx);
    journal::live_design(cx);
    stat::live_design(cx);
    time::live_design(cx);
    collection::live_design(cx);
}
