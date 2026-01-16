pub mod app;
pub mod widgets;

use makepad_widgets::*;

use app::App;

// Point app_main to the App struct
app_main!(App);

// Explicitly register the app module, as app_main! calls `live_design(cx)`
fn live_design(cx: &mut Cx) {
    crate::app::live_design(cx);
}
