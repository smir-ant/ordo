pub mod app;
pub mod widgets;
pub mod styling;
pub mod modules;
pub mod utils;
pub mod header;

use makepad_widgets::*;

use app::App;

// Point app_main to the App struct
app_main!(App);

// Explicitly register the app module, as app_main! calls `live_design(cx)`
fn live_design(cx: &mut Cx) {
    makepad_widgets::live_design(cx);
    crate::styling::live_design(cx);
    crate::widgets::live_design(cx);
    crate::modules::live_design(cx);
    crate::app::live_design(cx);
}
