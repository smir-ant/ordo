use makepad_widgets::*;
use crate::widgets::input::Input;
use crate::widgets::button::Button as Btn;

live_design!{
    use makepad_widgets::base::*;
    use makepad_widgets::theme_desktop_dark::*; 
    use makepad_widgets::window::Window;
    use makepad_widgets::scroll_bars::ScrollBars;
    use makepad_widgets::view_ui::View;
    use makepad_widgets::button::Button;
    use crate::widgets::input::Input;
    use crate::widgets::button::Btn;
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
                
                input1 = <Input> {
                    width: Fill, height: Fit
                    empty_text: "* Morning Routine"
                    is_numeric_only: true
                    is_required: true
                }
                submit_btn = <Btn> {
                    text: "Submit"
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
        let scope = &mut Scope::empty();
        self.ui.handle_event(cx, event, scope);
        
        if let Event::Actions(actions) = event {
            if let Some(btn) = self.ui.widget(ids!(submit_btn)).borrow::<Btn>() {
                if btn.clicked(&actions) {
                 // Using borrow_mut to access validate directly since helper trait might not be generated automatically
                 // actually standard convention is to assume helper exists or use ref pattern
                 // Let's rely on OrdoTextInputRef if available
                 if let Some(mut input) = self.ui.widget(ids!(input1)).borrow_mut::<Input>() {
                     let valid = input.validate(cx);
                     log!("Validation result: {}", valid);
                }
            }
        }
        }
    }
}
