use makepad_widgets::*;

live_design!{
    use makepad_widgets::base::*;
    use makepad_widgets::theme_desktop_dark::*; 
    use makepad_widgets::window::Window;
    use makepad_widgets::view_ui::View;
    use crate::widgets::text::Text;
    
    App = {{App}} {
        ui: <Window> {
            window: {inner_size: vec2(1000, 800), min_size: vec2(600, 400)}
            body = <View> {
                flow: Down,
                align: {x: 0.5, y: 0.5},
                show_bg: true,
                draw_bg: {
                    color: #1f1f1f
                }
                
                <Text> {
                    text: "Hello, Refactored World!"
                    draw_text: {
                        color: #ffffff
                        text_style: {
                            font_size: 40.0
                        }
                    }
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
        crate::widgets::live_design(cx);
    }
}

impl AppMain for App {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event) {
        self.ui.handle_event(cx, event, &mut Scope::empty());
    }
}
