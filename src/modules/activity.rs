use makepad_widgets::*;
use crate::widgets::button::Button;
use crate::header::{HeaderAction, MenuItem};

live_design! {
    use makepad_widgets::base::*;
    use makepad_widgets::theme_desktop_dark::*;
    use makepad_widgets::view_ui::View;
    use makepad_widgets::scroll_bars::ScrollBars;
    use crate::widgets::text::Text;
    use crate::widgets::button::Btn;
    use makepad_draw::shader::std::*;
    use link::styling::*;

    pub ActivityScreen = {{ActivityScreen}} {
        width: Fill, height: Fill
        flow: Overlay
        show_bg: true
        draw_bg: { color: (THEME_COLOR_BG_DARK) }

        // Main list view with FAB
        list_view = <View> {
            width: Fill, height: Fill
            flow: Overlay

            // Content area (placeholder)
            <View> {
                width: Fill, height: Fill
                flow: Down
                padding: {left: 16.0, right: 16.0, top: 10.0, bottom: 10.0}
                scroll_bars: <ScrollBars> {
                    show_scroll_x: false, show_scroll_y: true
                    scroll_bar_y: { drag_scrolling: true, smoothing: 0.15 }
                }
                <Text> {
                    text: "Activity"
                    draw_text: { color: (THEME_COLOR_TEXT_TERTIARY), text_style: { font_size: 32.0 } }
                }
            }

            // FAB — bottom right
            <View> {
                width: Fill, height: Fill
                align: {x: 1.0, y: 1.0}
                padding: {right: 16.0, bottom: 80.0}
                fab_create = <Btn> {
                    text: "+"
                    width: 56.0, height: 56.0
                    accent: true
                    padding: {top: 0.0, bottom: 4.0, left: 0.0, right: 0.0}
                    margin: 0.0
                    draw_bg: {
                        fn pixel(self) -> vec4 {
                            let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                            sdf.circle(
                                self.rect_size.x * 0.5,
                                self.rect_size.y * 0.5,
                                min(self.rect_size.x, self.rect_size.y) * 0.5
                            );
                            let col = mix(self.accent_color, #fff, 0.15 * self.hover);
                            sdf.fill(mix(col, self.accent_color * 0.8, self.down));
                            return sdf.result;
                        }
                    }
                    draw_text: { text_style: { font_size: 24.0 } }
                    animator: {
                        hover = {
                            default: off,
                            off = {
                                from: {all: Forward {duration: 0.08}}
                                apply: {
                                    draw_bg: {hover: 0.0, down: 0.0}
                                    draw_text: {hover: 0.0, down: 0.0}
                                }
                            }
                            on = {
                                from: {all: Forward {duration: 0.05}}
                                apply: {
                                    draw_bg: {hover: 1.0, down: 0.0}
                                    draw_text: {hover: 1.0, down: 0.0}
                                }
                            }
                            down = {
                                from: {all: Forward {duration: 0.03}}
                                apply: {
                                    draw_bg: {hover: 1.0, down: 1.0}
                                    draw_text: {hover: 1.0, down: 1.0}
                                }
                            }
                        }
                    }
                }
            }
        }

        // Create activity page (hidden by default)
        create_view = <View> {
            width: Fill, height: Fill, visible: false
            show_bg: true
            draw_bg: { color: (THEME_COLOR_BG_DARK) }
            flow: Down
            padding: {left: 16.0, right: 16.0, top: 10.0, bottom: 10.0}
            scroll_bars: <ScrollBars> {
                show_scroll_x: false, show_scroll_y: true
                scroll_bar_y: { drag_scrolling: true, smoothing: 0.15 }
            }

            // Content placeholder
            <View> {
                width: Fill, height: Fill
                <Text> {
                    text: "Create Activity"
                    draw_text: { color: (THEME_COLOR_TEXT_TERTIARY), text_style: { font_size: 32.0 } }
                }
            }
        }
    }
}

#[derive(Live, LiveHook, Widget)]
pub struct ActivityScreen {
    #[deref]
    view: View,
}

impl Widget for ActivityScreen {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        self.view.handle_event(cx, event, scope);

        if let Event::Actions(actions) = event {
            let mut show_create = None;

            if let Some(btn) = self.view.widget(ids!(fab_create)).borrow::<Button>() {
                if btn.clicked(actions) {
                    show_create = Some(true);
                }
            }

            // Header back button was clicked — go back to list
            let uid = self.widget_uid();
            for action in actions.filter_widget_actions_cast::<HeaderAction>(uid) {
                if matches!(action, HeaderAction::BackClicked) {
                    show_create = Some(false);
                }
            }

            if let Some(creating) = show_create {
                self.view.widget(ids!(list_view)).apply_over(cx, live!{ visible: (!creating) });
                self.view.widget(ids!(create_view)).apply_over(cx, live!{ visible: (creating) });

                let title = if creating { "Create Activity" } else { "Activity" };
                cx.widget_action(uid, &scope.path, HeaderAction::SetTitle(title.into()));

                cx.widget_action(uid, &scope.path, HeaderAction::ShowBack(creating));

                let menu_label = if creating { "Log Create" } else { "Log Activity" };
                cx.widget_action(uid, &scope.path, HeaderAction::SetMenu(
                    vec![MenuItem { id: live_id!(log), label: menu_label.into() }]
                ));
            }
        }
    }

    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        self.view.draw_walk(cx, scope, walk)
    }
}
