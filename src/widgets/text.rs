use makepad_widgets::*;

live_design! {
    use makepad_widgets::base::*;
    use makepad_widgets::theme_desktop_dark::*; 
    use crate::theme::*;

    pub Text = {{Text}} {
        width: Fit, height: Fit
        draw_text: {
            color: #fff
            text_style: <THEME_FONT_REGULAR> {
                font_size: (THEME_FONT_SIZE_BASE)
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

impl Text {
    pub fn set_text(&mut self, cx: &mut Cx, text: &str) {
        self.text = text.to_string();
        self.draw_text.redraw(cx);
    }
}
