use makepad_widgets::*;

live_design! {
    use makepad_widgets::base::*;
    use makepad_widgets::theme_desktop_dark::*;
    use makepad_widgets::view_ui::View;
    use makepad_widgets::scroll_bars::ScrollBars;
    use crate::widgets::text::Text;
    use link::styling::*;

    pub TimeScreen = {{TimeScreen}} {
        width: Fill, height: Fill
        flow: Down
        show_bg: true
        draw_bg: { color: (THEME_COLOR_BG_DARK) }
        padding: {left: 16.0, right: 16.0, top: 10.0, bottom: 10.0}
        scroll_bars: <ScrollBars> {
            show_scroll_x: false, show_scroll_y: true
            scroll_bar_y: { drag_scrolling: true, smoothing: 0.15 }
        }

        <Text> {
            text: "Time"
            draw_text: { color: (THEME_COLOR_TEXT_TERTIARY), text_style: { font_size: 32.0 } }
        }

        // Spacer to prevent content from being hidden under Nav
        <View> { width: Fill, height: 60.0 }
    }
}

#[derive(Live, LiveHook, Widget)]
pub struct TimeScreen {
    #[deref]
    view: View,
}

impl Widget for TimeScreen {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        self.view.handle_event(cx, event, scope);
    }

    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        self.view.draw_walk(cx, scope, walk)
    }
}
