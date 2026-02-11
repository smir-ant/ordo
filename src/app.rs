use makepad_widgets::*;
use crate::widgets::toggle_icon_button::ToggleIconButton;

live_design! {
    use makepad_widgets::base::*;
    use makepad_widgets::theme_desktop_dark::*;
    use makepad_widgets::window::Window;
    use makepad_widgets::view_ui::View;
    use crate::widgets::text::Text;
    use crate::widgets::toggle_icon_button::ToggleIconButton;

    App = {{App}} {
        ui: <Window> {
            window: {inner_size: vec2(800, 600), min_size: vec2(320, 480)}
            body = <View> {
                width: Fill, height: Fill
                flow: Overlay

                app_content = <View> {
                    width: Fill, height: Fill
                    flow: Down

                    // === Screens (stacked, one visible at a time) ===
                    screens = <View> {
                        width: Fill, height: Fill
                        flow: Overlay

                        screen_activity = <View> {
                            width: Fill, height: Fill
                            show_bg: true, draw_bg: { color: #1f1f1f }
                            align: {x: 0.5, y: 0.5}
                            <Text> {
                                text: "Activity"
                                draw_text: { color: #333, text_style: { font_size: 32.0 } }
                            }
                        }

                        screen_journal = <View> {
                            width: Fill, height: Fill, visible: false
                            show_bg: true, draw_bg: { color: #1f1f1f }
                            align: {x: 0.5, y: 0.5}
                            <Text> {
                                text: "Journal"
                                draw_text: { color: #333, text_style: { font_size: 32.0 } }
                            }
                        }

                        screen_stat = <View> {
                            width: Fill, height: Fill, visible: false
                            show_bg: true, draw_bg: { color: #1f1f1f }
                            align: {x: 0.5, y: 0.5}
                            <Text> {
                                text: "Statistics"
                                draw_text: { color: #333, text_style: { font_size: 32.0 } }
                            }
                        }

                        screen_time = <View> {
                            width: Fill, height: Fill, visible: false
                            show_bg: true, draw_bg: { color: #1f1f1f }
                            align: {x: 0.5, y: 0.5}
                            <Text> {
                                text: "Time"
                                draw_text: { color: #333, text_style: { font_size: 32.0 } }
                            }
                        }

                        screen_collection = <View> {
                            width: Fill, height: Fill, visible: false
                            show_bg: true, draw_bg: { color: #1f1f1f }
                            align: {x: 0.5, y: 0.5}
                            <Text> {
                                text: "Collection"
                                draw_text: { color: #333, text_style: { font_size: 32.0 } }
                            }
                        }
                    }

                    // === Separator ===
                    <View> {
                        width: Fill, height: 1.0
                        show_bg: true
                        draw_bg: { color: #2a2a2a }
                    }

                    // === Bottom Navigation Bar ===
                    nav_bar = <View> {
                        width: Fill, height: 56.0
                        flow: Right
                        show_bg: true
                        draw_bg: { color: #181818 }

                        <View> {
                            width: Fill, height: Fill
                            align: {x: 0.5, y: 0.5}
                            nav_activity = <ToggleIconButton> {
                                icon_walk: { width: 26.0, height: 26.0 }
                                draw_icon: {
                                    svg_file: dep("crate://self/resources/img/modules/activity.svg")
                                }
                                draw_icon_active: {
                                    svg_file: dep("crate://self/resources/img/modules/activity-fill.svg")
                                }
                            }
                        }

                        <View> {
                            width: Fill, height: Fill
                            align: {x: 0.5, y: 0.5}
                            nav_journal = <ToggleIconButton> {
                                icon_walk: { width: 26.0, height: 26.0 }
                                draw_icon: {
                                    svg_file: dep("crate://self/resources/img/modules/journal.svg")
                                }
                                draw_icon_active: {
                                    svg_file: dep("crate://self/resources/img/modules/journal-fill.svg")
                                }
                            }
                        }

                        <View> {
                            width: Fill, height: Fill
                            align: {x: 0.5, y: 0.5}
                            nav_stat = <ToggleIconButton> {
                                icon_walk: { width: 26.0, height: 26.0 }
                                draw_icon: {
                                    svg_file: dep("crate://self/resources/img/modules/stat.svg")
                                }
                                draw_icon_active: {
                                    svg_file: dep("crate://self/resources/img/modules/stat-fill.svg")
                                }
                            }
                        }

                        <View> {
                            width: Fill, height: Fill
                            align: {x: 0.5, y: 0.5}
                            nav_time = <ToggleIconButton> {
                                icon_walk: { width: 26.0, height: 26.0 }
                                draw_icon: {
                                    svg_file: dep("crate://self/resources/img/modules/time.svg")
                                }
                                draw_icon_active: {
                                    svg_file: dep("crate://self/resources/img/modules/time-fill.svg")
                                }
                            }
                        }

                        <View> {
                            width: Fill, height: Fill
                            align: {x: 0.5, y: 0.5}
                            nav_collection = <ToggleIconButton> {
                                icon_walk: { width: 26.0, height: 26.0 }
                                draw_icon: {
                                    svg_file: dep("crate://self/resources/img/modules/collection.svg")
                                }
                                draw_icon_active: {
                                    svg_file: dep("crate://self/resources/img/modules/collection-fill.svg")
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Default)]
enum Screen {
    #[default]
    Activity,
    Journal,
    Stat,
    Time,
    Collection,
}

#[derive(Live, LiveHook)]
pub struct App {
    #[live] ui: WidgetRef,
    #[rust] active_screen: Screen,
    #[rust] initialized: bool,
}

impl LiveRegister for App {
    fn live_register(_cx: &mut Cx) {}
}

impl AppMain for App {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event) {
        if !self.initialized {
            self.initialized = true;
            self.activate_screen(cx, Screen::Activity);
        }

        let scope = &mut Scope::empty();
        self.ui.handle_event(cx, event, scope);

        if let Event::Actions(actions) = event {
            self.handle_nav(cx, actions);
        }
    }
}

impl App {
    fn handle_nav(&mut self, cx: &mut Cx, actions: &Actions) {
        let nav = [
            (ids!(nav_activity), Screen::Activity),
            (ids!(nav_journal), Screen::Journal),
            (ids!(nav_stat), Screen::Stat),
            (ids!(nav_time), Screen::Time),
            (ids!(nav_collection), Screen::Collection),
        ];

        // Detect which button was clicked, then drop the borrow before activate_screen
        let mut target = None;
        for (id, screen) in nav {
            if let Some(btn) = self.ui.widget(id).borrow::<ToggleIconButton>() {
                if btn.toggled(actions).is_some() {
                    target = Some(screen);
                    break;
                }
            }
        }

        if let Some(screen) = target {
            self.activate_screen(cx, screen);
        }
    }

    fn activate_screen(&mut self, cx: &mut Cx, screen: Screen) {
        self.active_screen = screen;

        // Radio behavior: sync nav button states
        let nav = [
            (ids!(nav_activity), Screen::Activity),
            (ids!(nav_journal), Screen::Journal),
            (ids!(nav_stat), Screen::Stat),
            (ids!(nav_time), Screen::Time),
            (ids!(nav_collection), Screen::Collection),
        ];
        for (id, s) in nav {
            if let Some(mut btn) = self.ui.widget(id).borrow_mut::<ToggleIconButton>() {
                btn.set_activated(cx, s == self.active_screen);
            }
        }

        // Show active screen, hide others
        let screens = [
            (ids!(screen_activity), Screen::Activity),
            (ids!(screen_journal), Screen::Journal),
            (ids!(screen_stat), Screen::Stat),
            (ids!(screen_time), Screen::Time),
            (ids!(screen_collection), Screen::Collection),
        ];
        for (id, s) in screens {
            let visible = s == self.active_screen;
            self.ui.widget(id).apply_over(cx, live!{ visible: (visible) });
        }
    }
}
