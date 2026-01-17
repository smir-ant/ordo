use makepad_widgets::*;

live_design!{
    use makepad_widgets::base::*;
    use makepad_widgets::theme_desktop_dark::*; 
    use makepad_widgets::window::Window;
    use makepad_widgets::scroll_bars::ScrollBars;
    use makepad_widgets::view_ui::View;
    use makepad_widgets::button::Button;
    use makepad_widgets::text_input::TextInput;
    use crate::widgets::text::Text;
    use crate::theme::*;
    
    App = {{App}} {
        ui: <Window> {
            window: {inner_size: vec2(900, 700), min_size: vec2(600, 400)}
            body = <View> {
                width: Fill, height: Fill,
                flow: Down,
                spacing: 20.0,
                padding: 20.0,
                
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
                
                <Text> {
                    text: "Monospace Text (Ubuntu Mono)"
                    draw_text: {
                        color: #a3d9ff
                        text_style: <THEME_FONT_MONO> { font_size: 20.0 }
                    }
                }
                
                <Text> {
                    text: "code_snippet = fn main() {}"
                    draw_text: {
                        color: #ffaa00
                        text_style: <THEME_FONT_MONO> { font_size: 16.0 }
                    }
                }
                
                <Button> {
                    width: Fill
                    text: "Create Activity" // Matches check.html text
                }
                
                <TextInput> {
                    width: Fill, height: Fit
                    empty_text: "Morning Routine"
                    is_numeric_only: true
                    is_required: true
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
    fn live_register(_cx: &mut Cx) {
    }
}

impl AppMain for App {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event) {
        self.ui.handle_event(cx, event, &mut Scope::empty());
    }
}
