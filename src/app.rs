use makepad_widgets::*;
use makepad_widgets::event::TouchState;
use crate::widgets::toggle_icon_button::ToggleIconButton;
use crate::widgets::icon_button::IconButton;
use crate::header::{HeaderAction, MenuItem};
use crate::actions::ScreenAction;
use crate::widgets::date_picker::{DatePicker, DatePickerAction};
use crate::widgets::time_picker::{TimePicker, TimePickerAction};
use crate::widgets::modal::Modal;
use crate::modules::activity::ActivityScreen;

live_design! {
    use makepad_widgets::base::*;
    use makepad_widgets::theme_desktop_dark::*;
    use makepad_widgets::window::Window;
    use makepad_widgets::view_ui::View;
    use crate::widgets::toggle_icon_button::ToggleIconButton;
    use makepad_draw::shader::std::*;
    use crate::widgets::text::Text;
    use crate::widgets::icon_button::IconButton;
    use crate::widgets::button::Btn;
    use crate::widgets::date_picker::DatePicker;
    use crate::widgets::time_picker::TimePicker;
    use crate::widgets::modal::Modal;
    use crate::widgets::modal::TooltipStyle;
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
                draw_bg: { color: (THEME_COLOR_BG_DARK) }

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
                        width: Fill, height: 48.0
                        flow: Overlay
                        show_bg: true
                        draw_bg: { color: (THEME_COLOR_BG_PANEL) }

                        // Layer 1: centered title
                        <View> {
                            width: Fill, height: Fill
                            align: {x: 0.5, y: 0.5}
                            padding: {bottom: 6.0}
                            header_title = <Text> {
                                width: Fit, height: Fit
                                text: "Activity"
                                draw_text: {
                                    color: (THEME_COLOR_TEXT_PRIMARY)
                                    text_style: { font_size: 20.0 }
                                }
                            }
                        }

                        // Layer 2: buttons at edges
                        <View> {
                            width: Fill, height: Fill
                            flow: Right
                            align: {y: 0.5}
                            padding: {left: 8.0, right: 8.0, bottom: 6.0}

                            back_wrap = <View> {
                                width: Fit, height: Fit
                                visible: false
                                btn_back = <IconButton> {
                                    icon_walk: { width: 24.0, height: 24.0 }
                                    draw_icon: {
                                        color: (THEME_COLOR_TEXT_SECONDARY)
                                        svg_file: dep("crate://self/resources/img/icon_back.svg")
                                    }
                                }
                            }

                            <View> { width: Fill, height: Fit }

                            btn_menu = <IconButton> {
                                icon_walk: { width: 24.0, height: 24.0 }
                                draw_icon: {
                                    color: (THEME_COLOR_TEXT_SECONDARY)
                                    svg_file: dep("crate://self/resources/img/icon_more.svg")
                                }
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

                // === Nav (floating overlay) ===
                <View> {
                    width: Fill, height: Fill
                    align: {x: 0.5, y: 1.0}
                    padding: {bottom: 12.0}

                    nav = <View> {
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
                            icon_walk: { width: 30.0, height: 30.0 }
                            draw_icon: {
                                svg_file: dep("crate://self/resources/img/modules/activity.svg")
                            }
                            draw_icon_active: {
                                svg_file: dep("crate://self/resources/img/modules/activity-fill.svg")
                            }
                        }

                        nav_journal = <ToggleIconButton> {
                            icon_walk: { width: 30.0, height: 30.0 }
                            draw_icon: {
                                svg_file: dep("crate://self/resources/img/modules/journal.svg")
                            }
                            draw_icon_active: {
                                svg_file: dep("crate://self/resources/img/modules/journal-fill.svg")
                            }
                        }

                        nav_stat = <ToggleIconButton> {
                            icon_walk: { width: 30.0, height: 30.0 }
                            draw_icon: {
                                svg_file: dep("crate://self/resources/img/modules/stat.svg")
                            }
                            draw_icon_active: {
                                svg_file: dep("crate://self/resources/img/modules/stat-fill.svg")
                            }
                        }

                        nav_time = <ToggleIconButton> {
                            icon_walk: { width: 30.0, height: 30.0 }
                            draw_icon: {
                                svg_file: dep("crate://self/resources/img/modules/time.svg")
                            }
                            draw_icon_active: {
                                svg_file: dep("crate://self/resources/img/modules/time-fill.svg")
                            }
                        }

                        nav_collection = <ToggleIconButton> {
                            icon_walk: { width: 30.0, height: 30.0 }
                            draw_icon: {
                                svg_file: dep("crate://self/resources/img/modules/collection.svg")
                            }
                            draw_icon_active: {
                                svg_file: dep("crate://self/resources/img/modules/collection-fill.svg")
                            }
                        }
                    }
                }

                // === Menu popup ===
                menu_overlay = <View> {
                    width: Fill, height: Fill
                    visible: false
                    align: {x: 1.0, y: 0.0}
                    padding: {top: 80.0, right: 8.0}

                    menu_panel = <View> {
                        width: 200.0, height: Fit
                        flow: Down
                        padding: {top: 4.0, bottom: 4.0}
                        show_bg: true
                        draw_bg: {
                            color: #2a2a2a
                            fn pixel(self) -> vec4 {
                                let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                                sdf.box(0., 0., self.rect_size.x, self.rect_size.y, 6.);
                                sdf.fill(self.color);
                                return sdf.result;
                            }
                        }

                        menu_item_0 = <Btn> {
                            text: ""
                            visible: false
                            width: Fill, height: Fit
                            align: {x: 0.0, y: 0.5}
                            padding: {top: 10.0, bottom: 10.0, left: 16.0, right: 16.0}
                            margin: 0.0
                            draw_bg: {
                                color: #0000
                                color_hover: #ffffff10
                                color_down: #ffffff20
                                border_size: 0.0
                                border_radius: 0.0
                            }
                            draw_text: {
                                color: #fff
                                text_style: { font_size: 14.0 }
                            }
                        }
                        menu_item_1 = <Btn> {
                            text: ""
                            visible: false
                            width: Fill, height: Fit
                            align: {x: 0.0, y: 0.5}
                            padding: {top: 10.0, bottom: 10.0, left: 16.0, right: 16.0}
                            margin: 0.0
                            draw_bg: {
                                color: #0000
                                color_hover: #ffffff10
                                color_down: #ffffff20
                                border_size: 0.0
                                border_radius: 0.0
                            }
                            draw_text: {
                                color: #fff
                                text_style: { font_size: 14.0 }
                            }
                        }
                        menu_item_2 = <Btn> {
                            text: ""
                            visible: false
                            width: Fill, height: Fit
                            align: {x: 0.0, y: 0.5}
                            padding: {top: 10.0, bottom: 10.0, left: 16.0, right: 16.0}
                            margin: 0.0
                            draw_bg: {
                                color: #0000
                                color_hover: #ffffff10
                                color_down: #ffffff20
                                border_size: 0.0
                                border_radius: 0.0
                            }
                            draw_text: {
                                color: #fff
                                text_style: { font_size: 14.0 }
                            }
                        }
                        menu_item_3 = <Btn> {
                            text: ""
                            visible: false
                            width: Fill, height: Fit
                            align: {x: 0.0, y: 0.5}
                            padding: {top: 10.0, bottom: 10.0, left: 16.0, right: 16.0}
                            margin: 0.0
                            draw_bg: {
                                color: #0000
                                color_hover: #ffffff10
                                color_down: #ffffff20
                                border_size: 0.0
                                border_radius: 0.0
                            }
                            draw_text: {
                                color: #fff
                                text_style: { font_size: 14.0 }
                            }
                        }
                    }
                }

                // === Global DatePicker (overlay above everything) ===
                global_date_picker = <DatePicker> {}

                // === Global TimePicker ===
                global_time_picker = <TimePicker> {}

                // === Global Tooltip Modal ===
                global_tooltip = <Modal> {
                    content = <TooltipStyle> {
                        tooltip_title = <Text> {
                            text: "Tooltip"
                            draw_text: { color: #fff, text_style: { font_size: 14.0 } }
                        }
                        tooltip_description = <Text> {
                            width: Fill, height: Fit
                            text: "Description"
                            draw_text: { color: #ccc, wrap: Word, text_style: { font_size: 12.0 } }
                        }
                        ok_button = <Btn> { width: Fill, text: "Got it" }
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

#[derive(Clone, Debug)]
struct ScreenState {
    title: String,
    menu: Vec<MenuItem>,
    back_visible: bool,
}

impl Screen {
    const ALL: [Screen; 5] = [
        Screen::Activity, Screen::Journal, Screen::Stat, Screen::Time, Screen::Collection,
    ];

    fn index(self) -> usize { self as usize }

    fn title(self) -> &'static str {
        match self {
            Screen::Activity => "Activity",
            Screen::Journal => "Journal",
            Screen::Stat => "Statistics",
            Screen::Time => "Time",
            Screen::Collection => "Collection",
        }
    }

    fn menu_items(self) -> Vec<MenuItem> {
        let label = format!("Log {}", self.title());
        vec![MenuItem { id: live_id!(log), label }]
    }
}

#[derive(Live, LiveHook)]
pub struct App {
    #[live] ui: WidgetRef,
    #[rust] active_screen: Screen,
    #[rust] initialized: bool,
    #[rust] menu_items: Vec<MenuItem>,
    #[rust] menu_open: bool,
    #[rust] screen_states: Vec<ScreenState>,
}

impl LiveRegister for App {
    fn live_register(_cx: &mut Cx) {}
}

impl AppMain for App {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event) {
        if !self.initialized {
            self.initialized = true;
            for s in Screen::ALL {
                self.screen_states.push(ScreenState {
                    title: s.title().to_string(),
                    menu: s.menu_items(),
                    back_visible: false,
                });
            }
            self.activate_screen(cx, Screen::Activity);
        }

        // Check if modal is open BEFORE handling event (so modal state is pre-close)
        let modal_was_open = self.any_modal_open();

        let scope = &mut Scope::empty();
        self.ui.handle_event(cx, event, scope);

        if let Event::Actions(actions) = event {
            self.handle_nav(cx, actions);
            self.handle_header_actions(cx, actions);
            self.handle_menu(cx, actions);
            self.handle_app_actions(cx, actions);
        }

        // System back button (Android/iOS gesture + Desktop Escape)
        // Only handle if no modal was open (modal already handled it internally)
        let is_escape = matches!(event, Event::KeyDown(ke) if ke.key_code == KeyCode::Escape);
        let is_back = event.back_pressed() || is_escape;

        if is_back && !modal_was_open {
            if self.menu_open {
                self.close_menu(cx);
            } else {
                let screen_ids = [
                    ids!(screen_activity),
                    ids!(screen_journal),
                    ids!(screen_stat),
                    ids!(screen_time),
                    ids!(screen_collection),
                ];
                let active_id = screen_ids[self.active_screen.index()];
                let uid = self.ui.widget(active_id).widget_uid();
                cx.widget_action(uid, &HeapLiveIdPath::default(), HeaderAction::BackClicked);
            }
        }

        // Close menu on any click (after actions processed)
        if self.menu_open {
            match event {
                Event::MouseUp(_) => self.close_menu(cx),
                Event::TouchUpdate(te) => {
                    if te.touches.iter().any(|t| matches!(t.state, TouchState::Stop)) {
                        self.close_menu(cx);
                    }
                }
                _ => {}
            }
        }
    }
}

impl App {
    /// Check if any global modal is open
    fn any_modal_open(&self) -> bool {
        self.ui.widget(ids!(global_date_picker)).borrow::<DatePicker>().map_or(false, |dp| dp.is_open())
            || self.ui.widget(ids!(global_time_picker)).borrow::<TimePicker>().map_or(false, |tp| tp.is_open())
            || self.ui.widget(ids!(global_tooltip)).borrow::<Modal>().map_or(false, |m| m.is_open())
    }

    fn handle_app_actions(&mut self, cx: &mut Cx, actions: &Actions) {
        // Listen for actions from screens and modals (single loop instead of three)
        for action in actions {
            // Screen wants to open a modal
            match action.as_widget_action().cast::<ScreenAction>() {
                ScreenAction::OpenDatePicker => {
                    if let Some(mut dp) = self.ui.widget(ids!(global_date_picker)).borrow_mut::<DatePicker>() {
                        dp.open(cx);
                    }
                }
                ScreenAction::OpenTimePicker => {
                    if let Some(mut tp) = self.ui.widget(ids!(global_time_picker)).borrow_mut::<TimePicker>() {
                        tp.open(cx);
                    }
                }
                ScreenAction::OpenTooltip { title, description } => {
                    self.ui.widget(ids!(tooltip_title)).set_text(cx, &title);
                    self.ui.widget(ids!(tooltip_description)).set_text(cx, &description);
                    if let Some(mut modal) = self.ui.widget(ids!(global_tooltip)).borrow_mut::<Modal>() {
                        modal.open(cx);
                    }
                }
                _ => {}
            }

            // DatePicker confirmed → send to active screen (only Activity for now)
            if let DatePickerAction::Accepted { year, month, day } = action.as_widget_action().cast::<DatePickerAction>() {
                if self.active_screen == Screen::Activity {
                    if let Some(mut screen) = self.ui.widget(ids!(screen_activity)).borrow_mut::<ActivityScreen>() {
                        screen.handle_date_selected(cx, year as i32, month, day);
                    }
                }
            }

            // TimePicker confirmed → send to active screen (only Activity for now)
            if let TimePickerAction::Accepted { hours, minutes, seconds } = action.as_widget_action().cast::<TimePickerAction>() {
                if self.active_screen == Screen::Activity {
                    if let Some(mut screen) = self.ui.widget(ids!(screen_activity)).borrow_mut::<ActivityScreen>() {
                        screen.handle_time_selected(cx, hours, minutes, seconds);
                    }
                }
            }
        }
    }

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

    fn get_active_screen_id(&self) -> &'static [LiveId] {
        match self.active_screen {
            Screen::Activity => ids!(screen_activity),
            Screen::Journal => ids!(screen_journal),
            Screen::Stat => ids!(screen_stat),
            Screen::Time => ids!(screen_time),
            Screen::Collection => ids!(screen_collection),
        }
    }

    fn activate_screen(&mut self, cx: &mut Cx, screen: Screen) {
        self.active_screen = screen;

        // Restore stored header state for this screen
        let i = screen.index();
        let title = self.screen_states[i].title.clone();
        let menu = self.screen_states[i].menu.clone();
        let back_visible = self.screen_states[i].back_visible;

        self.ui.widget(ids!(header_title)).set_text(cx, &title);
        self.update_menu_config(cx, menu);
        self.ui.widget(ids!(back_wrap)).apply_over(cx, live!{ visible: (back_visible) });

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

    fn handle_header_actions(&mut self, cx: &mut Cx, actions: &Actions) {
        // Back button click → dispatch BackClicked to active screen
        let back_clicked = {
            if let Some(btn) = self.ui.widget(ids!(btn_back)).borrow::<IconButton>() {
                btn.clicked(actions)
            } else { false }
        };
        if back_clicked {
            let active_id = self.get_active_screen_id();
            let uid = self.ui.widget(active_id).widget_uid();
            cx.widget_action(uid, &HeapLiveIdPath::default(), HeaderAction::BackClicked);
        }

        // Listen for HeaderActions from any screen (no need to loop through all screens)
        for action in actions {
            let i = self.active_screen.index();
            match action.as_widget_action().cast::<HeaderAction>() {
                HeaderAction::SetTitle(title) => {
                    self.screen_states[i].title = title.clone();
                    self.ui.widget(ids!(header_title)).set_text(cx, &title);
                }
                HeaderAction::SetMenu(items) => {
                    self.screen_states[i].menu = items.clone();
                    self.update_menu_config(cx, items);
                }
                HeaderAction::ShowBack(show) => {
                    self.screen_states[i].back_visible = show;
                    self.ui.widget(ids!(back_wrap)).apply_over(cx, live!{ visible: (show) });
                }
                _ => {}
            }
        }
    }

    fn close_menu(&mut self, cx: &mut Cx) {
        if !self.menu_open { return; }
        self.menu_open = false;
        self.ui.widget(ids!(menu_overlay)).apply_over(cx, live!{ visible: false });
        self.ui.redraw(cx);
    }

    fn update_menu_config(&mut self, cx: &mut Cx, items: Vec<MenuItem>) {
        self.menu_items = items;
        self.close_menu(cx);

        let slot_ids: [&[LiveId]; 4] = [
            ids!(menu_item_0), ids!(menu_item_1), ids!(menu_item_2), ids!(menu_item_3),
        ];
        for (i, slot) in slot_ids.iter().enumerate() {
            if i < self.menu_items.len() {
                let w = self.ui.widget(slot);
                w.set_text(cx, &self.menu_items[i].label);
                w.apply_over(cx, live!{ visible: true });
            } else {
                self.ui.widget(slot).apply_over(cx, live!{ visible: false });
            }
        }
    }

    fn handle_menu(&mut self, cx: &mut Cx, actions: &Actions) {
        // Three-dot opens menu (only when closed)
        if !self.menu_open {
            let clicked = {
                if let Some(btn) = self.ui.widget(ids!(btn_menu)).borrow::<IconButton>() {
                    btn.clicked(actions)
                } else { false }
            };
            if clicked {
                self.menu_open = true;
                self.ui.widget(ids!(menu_overlay)).apply_over(cx, live!{ visible: true });
                self.ui.redraw(cx);
            }
            return;
        }

        // Menu is open — check item clicks (close handled by handle_event)
        let slot_ids: [&[LiveId]; 4] = [
            ids!(menu_item_0), ids!(menu_item_1), ids!(menu_item_2), ids!(menu_item_3),
        ];
        for (i, slot) in slot_ids.iter().enumerate() {
            if i >= self.menu_items.len() { break; }
            let clicked = {
                if let Some(btn) = self.ui.widget(slot).borrow::<crate::widgets::button::Button>() {
                    btn.clicked(actions)
                } else { false }
            };
            if clicked {
                log!("Menu clicked: {} (id: {:?})", self.menu_items[i].label, self.menu_items[i].id);
            }
        }
    }
}
