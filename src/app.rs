use makepad_widgets::*;

live_design!{
    use makepad_widgets::base::*;
    use makepad_widgets::theme_desktop_dark::*; 
    use makepad_widgets::window::Window;
    use makepad_widgets::scroll_bars::ScrollBars;
    use makepad_widgets::view_ui::View;
    use crate::widgets::text::Text;
    use crate::widgets::text::MonoText;
    
    App = {{App}} {
        ui: <Window> {
            window: {inner_size: vec2(400, 800), min_size: vec2(300, 400)}
            body = <View> {
                width: Fill, height: Fill,
                flow: Down,
                spacing: 20.0,
                
                show_bg: true,
                draw_bg: {
                    color: #1f1f1f
                }
                
                scroll_bars: <ScrollBars> {}
                
                // Typography Section
                <Text> {
                    width: Fit, margin: {bottom: 10.0}
                    text: "Typography System"
                    draw_text: {
                        color: #9cb4d8
                        text_style: { font_size: 14.0 }
                    }
                }

                <Text> {
                    text: "Regular Text (Inter)"
                    draw_text: {
                        color: #ffffff
                        text_style: { font_size: 20.0 }
                    }
                }
                
                <MonoText> {
                    text: "Monospace Text (Ubuntu Mono)"
                    draw_text: {
                        color: #a3d9ff
                        text_style: { font_size: 20.0 }
                    }
                }
                
                <MonoText> {
                    text: "code_snippet = fn main() {}"
                    draw_text: {
                        color: #ffaa00
                        text_style: { font_size: 16.0 }
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
