use makepad_widgets::*;
use crate::widgets::toggle_icon_button::ToggleIconButton;

live_design! {
    use makepad_widgets::base::*;
    use makepad_widgets::theme_desktop_dark::*;
    use makepad_widgets::window::Window;
    use makepad_widgets::view_ui::View;
    use crate::widgets::toggle_icon_button::ToggleIconButton;
    use makepad_draw::shader::std::*;
    use crate::widgets::text::Text;
    use crate::widgets::icon_button::IconButton;
    use link::styling::*;
    use crate::modules::activity::ActivityScreen;
    use crate::modules::journal::JournalScreen;
    use crate::modules::stat::StatScreen;
    use crate::modules::time::TimeScreen;
    use crate::modules::collection::CollectionScreen;

    App = {{App}} {
        ui: <Window> {
            window: {inner_size: vec2(800, 600), min_size: vec2(320, 480)}
            body = <View> {
                width: Fill, height: Fill
                flow: Overlay
                show_bg: true
                draw_bg: { color: #1f1f1f }

                // === Main content (header + screens) ===
                <View> {
                    width: Fill, height: Fill
                    flow: Down

                    // === Safe area (macOS traffic lights / Android status bar) ===
                    <View> {
                        width: Fill, height: 32.0
                        show_bg: true
                        draw_bg: { color: (THEME_COLOR_BG_PANEL) }
                    }

                    // === Header ===
                    header = <View> {
                        width: Fill, height: 40.0
                        flow: Right
                        align: {y: 0.5}
                        padding: {left: 16.0, right: 8.0}
                        show_bg: true
                        draw_bg: { color: (THEME_COLOR_BG_PANEL) }

                        header_title = <Text> {
                            width: Fill, height: Fit
                            text: "Activity"
                            draw_text: {
                                color: (THEME_COLOR_TEXT_PRIMARY)
                                text_style: { font_size: 18.0 }
                            }
                        }

                        btn_menu = <IconButton> {
                            icon_walk: { width: 20.0, height: 20.0 }
                            draw_icon: {
                                color: (THEME_COLOR_TEXT_SECONDARY)
                                svg_file: dep("crate://self/resources/img/icon_more.svg")
                            }
                        }
                    }

                    // === Screens (one visible at a time) ===
                    screens = <View> {
                        width: Fill, height: Fill
                        flow: Overlay

                        screen_activity = <ActivityScreen> {}
                        screen_journal = <JournalScreen> { visible: false }
                        screen_stat = <StatScreen> { visible: false }
                        screen_time = <TimeScreen> { visible: false }
                        screen_collection = <CollectionScreen> { visible: false }
                    }
                }

                // === Bottom Navigation Bar (floating overlay) ===
                <View> {
                    width: Fill, height: Fill
                    align: {x: 0.5, y: 1.0}
                    padding: {bottom: 12.0}

                    nav_bar = <View> {
                        width: Fit, height: Fit
                        flow: Right
                        spacing: 8.0
                        padding: 6.0
                        show_bg: true
                        draw_bg: {
                            color: (THEME_COLOR_BG_PANEL)
                            fn pixel(self) -> vec4 {
                                let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                                sdf.box(0., 0., self.rect_size.x, self.rect_size.y, 12.);
                                sdf.fill(self.color);
                                return sdf.result;
                            }
                        }

                        nav_activity = <ToggleIconButton> {
                            icon_walk: { width: 26.0, height: 26.0 }
                            draw_icon: {
                                svg_file: dep("crate://self/resources/img/modules/activity.svg")
                            }
                            draw_icon_active: {
                                svg_file: dep("crate://self/resources/img/modules/activity-fill.svg")
                            }
                        }

                        nav_journal = <ToggleIconButton> {
                            icon_walk: { width: 26.0, height: 26.0 }
                            draw_icon: {
                                svg_file: dep("crate://self/resources/img/modules/journal.svg")
                            }
                            draw_icon_active: {
                                svg_file: dep("crate://self/resources/img/modules/journal-fill.svg")
                            }
                        }

                        nav_stat = <ToggleIconButton> {
                            icon_walk: { width: 26.0, height: 26.0 }
                            draw_icon: {
                                svg_file: dep("crate://self/resources/img/modules/stat.svg")
                            }
                            draw_icon_active: {
                                svg_file: dep("crate://self/resources/img/modules/stat-fill.svg")
                            }
                        }

                        nav_time = <ToggleIconButton> {
                            icon_walk: { width: 26.0, height: 26.0 }
                            draw_icon: {
                                svg_file: dep("crate://self/resources/img/modules/time.svg")
                            }
                            draw_icon_active: {
                                svg_file: dep("crate://self/resources/img/modules/time-fill.svg")
                            }
                        }

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

#[derive(Clone, Copy, Debug, PartialEq, Default)]
enum Screen {
    #[default]
    Activity,
    Journal,
    Stat,
    Time,
    Collection,
}

impl Screen {
    fn title(self) -> &'static str {
        match self {
            Screen::Activity => "Activity",
            Screen::Journal => "Journal",
            Screen::Stat => "Statistics",
            Screen::Time => "Time",
            Screen::Collection => "Collection",
        }
    }
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

        // Update header title
        self.ui.widget(ids!(header_title)).set_text(cx, screen.title());

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
