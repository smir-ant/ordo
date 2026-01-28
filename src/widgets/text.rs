use makepad_widgets::*;

live_design! {
    use makepad_widgets::base::*;
    use makepad_widgets::theme_desktop_dark::*;
    use crate::theme::*;

    pub Text = {{Text}} {
        width: Fit, height: Fit
        draw_text: {
            color: #fff
            wrap: Word
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
    #[live] align: Align,
    #[area] area: Area,
}

impl Widget for Text {
    fn handle_event(&mut self, _cx: &mut Cx, _event: &Event, _scope: &mut Scope) {}

    fn draw_walk(&mut self, cx: &mut Cx2d, _scope: &mut Scope, walk: Walk) -> DrawStep {
        // Use Flow::right_wrap() to enable text wrapping
        cx.begin_turtle(walk, Layout {
            flow: Flow::right_wrap(),
            ..Layout::default()
        });
        self.draw_text.draw_walk(cx, walk, self.align, &self.text);
        cx.end_turtle_with_area(&mut self.area);
        DrawStep::done()
    }

    fn text(&self) -> String {
        self.text.clone()
    }

    fn set_text(&mut self, cx: &mut Cx, v: &str) {
        self.text = v.to_string();
        self.draw_text.redraw(cx);
    }
}
