use makepad_widgets::*;

// pub mod widgets;

live_design!{
    use makepad_widgets::base::*;
    use makepad_widgets::theme_desktop_dark::*; 
    use makepad_widgets::window::Window;
    use makepad_widgets::view_ui::View;
    
    
    pub Text = {{Text}} {
        draw_text: {
            color: #fff
            text_style: {
                font_family: {
                    base = font("crate://self/resources/font/Inter_opt.ttf", 0.0, 0.0)
                }
                font_size: 32.0
            }
        }
        text: "Hello"
    }
    
    App = {{App}} {
        ui: <Window> {
            body = <View> {
                flow: Down,
                align: {x: 0.5, y: 0.5},
                show_bg: true,
                draw_bg: {
                    color: #1f1f1f
                }
                
                <Text> {
                    text: "Hello, мир!"
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
    }
}

impl AppMain for App {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event) {
        self.ui.handle_event(cx, event, &mut Scope::empty());
    }
}

#[derive(Live, LiveHook, Widget)]
pub struct Text {
    #[redraw] #[live] draw_text: DrawText,
    #[live] text: String,
    #[walk] walk: Walk,
}

impl Widget for Text {
    fn handle_event(&mut self, _cx: &mut Cx, _event: &Event, _scope: &mut Scope) {
        // No interaction needed for now
    }

    fn draw_walk(&mut self, cx: &mut Cx2d, _scope: &mut Scope, walk: Walk) -> DrawStep {
        self.draw_text.draw_walk(cx, walk, Align::default(), &self.text);
        DrawStep::done()
    }
}

app_main!(App);
