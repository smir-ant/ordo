use makepad_widgets::*;

live_design! {
    use makepad_widgets::base::*;
    use makepad_widgets::theme_desktop_dark::*;
    use makepad_widgets::view_ui::View;
    use crate::widgets::text::Text;
    use link::styling::*;

    pub CollectionScreen = {{CollectionScreen}} {
        width: Fill, height: Fill
        show_bg: true
        draw_bg: { color: (THEME_COLOR_BG_DARK) }
        align: {x: 0.5, y: 0.5}

        <Text> {
            text: "Collection"
            draw_text: { color: (THEME_COLOR_TEXT_TERTIARY), text_style: { font_size: 32.0 } }
        }
    }
}

#[derive(Live, LiveHook, Widget)]
pub struct CollectionScreen {
    #[deref]
    view: View,
}

impl Widget for CollectionScreen {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        self.view.handle_event(cx, event, scope);
    }

    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        self.view.draw_walk(cx, scope, walk)
    }
}
