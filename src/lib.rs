use makepad_widgets::*;

live_design!{
    use makepad_widgets::window::Window;
    use makepad_widgets::view_ui::View;
    use makepad_widgets::theme_desktop_dark::*; 
    
    App = {{App}} {
        ui: <Window> {
            body = <View> {
                flow: Down,
                align: {x: 0.5, y: 0.5},
                show_bg: true,
                draw_bg: {
                    color: #1f1f1f
                }
                
                <View> {
                    width: 50.0, height: 50.0
                    show_bg: true
                    draw_bg: {color: #ff0000}
                }
            }
        }
    }
}

#[derive(Live, LiveHook)]
pub struct App {
    #[live] ui: WidgetRef,
}

impl LiveRegister for App {
    fn live_register(cx: &mut Cx) {
        makepad_widgets::live_design(cx);
    }
}

impl AppMain for App {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event) {
        self.ui.handle_event(cx, event, &mut Scope::empty());
    }
}

app_main!(App);
