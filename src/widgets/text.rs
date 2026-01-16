use makepad_widgets::*;

live_design! {
    use makepad_widgets::base::*;
    use makepad_widgets::theme_desktop_dark::*;

    pub Text = {{Text}} {
        width: Fit, height: Fit
        draw_text: {
            color: #fff
            text_style: {
                font_family: {
                    base = font("", 0.0, 0.0)
                }
                font_size: 32.0
            }
        }
        text: "Hello"
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
