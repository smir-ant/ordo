use makepad_widgets::*;
use makepad_widgets::drop_down::DropDownAction;
use crate::widgets::button::Button;
use crate::widgets::check::{Check, CheckAction};
use crate::widgets::modal::TooltipTriggerAction;
use crate::header::{HeaderAction, MenuItem};
use crate::actions::ScreenAction;
use crate::widgets::input::Input;
use crate::widgets::day_of_week::DayOfWeek;
use crate::widgets::days_of_month::DaysOfMonth;
use crate::utils::calendar;

// Constants for dropdown options and labels
const TIME_UNITS: &[&str] = &["day", "week", "month", "year"];
const REGULARITY_TYPES: &[&str] = &["Interval", "Days of Week", "Days of Month", "Target Goal", "One-time"];
const EVALUATION_TYPES: &[&str] = &["Numeric", "Yes/No", "Time"];
const DAY_NAMES: &[&str] = &["Mon", "Tue", "Wed", "Thu", "Fri", "Sat", "Sun"];

live_design! {
    use makepad_widgets::base::*;
    use makepad_widgets::theme_desktop_dark::*;
    use makepad_widgets::view_ui::View;
    use makepad_widgets::scroll_bars::ScrollBars;
    use crate::widgets::text::Text;
    use crate::widgets::button::Btn;
    use crate::widgets::input::Input;
    use crate::widgets::group::Group;
    use crate::widgets::date_picker::DatePicker;
    use crate::widgets::check::Check;
    use crate::widgets::day_of_week::DayOfWeek;
    use crate::widgets::days_of_month::DaysOfMonth;
    use crate::widgets::modal::TooltipTrigger;
    use crate::widgets::details::Details;
    use makepad_widgets::drop_down::DropDown;
    use makepad_draw::shader::std::*;
    use link::styling::*;

    // Custom DropDown with larger text
    pub Dropdown = <DropDown> {
        draw_text: {
            text_style: { font_size: 15.0 }
        }
    }

    pub ActivityScreen = {{ActivityScreen}} {
        width: Fill, height: Fill
        flow: Overlay  // Multiple layers: list, create
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

                // Spacer to prevent content from being hidden under Nav
                <View> { width: Fill, height: 60.0 }
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

            // Form content with scroll
            form_content = <View> {
                width: Fill, height: Fill
                show_bg: true
                draw_bg: { color: (THEME_COLOR_BG_DARK) }
                flow: Down
                padding: {left: 16.0, right: 16.0, top: 10.0, bottom: 10.0}
                spacing: 15.0
                scroll_bars: <ScrollBars> {
                    show_scroll_x: false, show_scroll_y: true
                    scroll_bar_y: { drag_scrolling: true, smoothing: 0.15 }
                }

                // Activity Name
                <Group> {
                    width: Fill, height: Fit
                    <Text> {
                        text: "Activity Name"
                        draw_text: { color: (THEME_COLOR_TEXT_PRIMARY), text_style: { font_size: 13.0 } }
                    }
                    activity_name = <Input> {
                        width: Fill, height: Fit
                        placeholder: "Morning routine"
                        is_required: true
                    }
                }

                // Start Date
                <Group> {
                    width: Fill, height: Fit
                    <Text> {
                        text: "Start Date"
                        draw_text: { color: (THEME_COLOR_TEXT_PRIMARY), text_style: { font_size: 13.0 } }
                    }
                    start_date_btn = <Btn> {
                        width: Fill, height: Fit
                        text: "Today"
                    }
                }

                // Regularity
                <Group> {
                    width: Fill, height: Fit
                    <Text> {
                        text: "Regularity"
                        draw_text: { color: (THEME_COLOR_TEXT_PRIMARY), text_style: { font_size: 13.0 } }
                    }
                    regularity_dropdown = <Dropdown> {
                        width: Fill
                        labels: ["Interval", "Days of Week", "Days of Month", "Target Goal", "One-time"]
                        selected_item: 0
                    }

                    // Interval view (default visible)
                    regularity_interval = <View> {
                        width: Fill, height: Fit
                        flow: Down
                        spacing: 10.0

                        <View> {
                            width: Fill, height: Fit
                            flow: Right
                            spacing: 10.0
                            align: {y: 0.5}

                            <Text> {
                                text: "each"
                                draw_text: { color: (THEME_COLOR_TEXT_SECONDARY), text_style: { font_size: 14.0 } }
                            }

                            interval_step_input_wrap = <View> {
                                width: Fit, height: Fit
                                visible: false

                                interval_step_input = <Input> {
                                    width: 60.0, height: Fit
                                    is_numeric_only: true
                                    placeholder: "1"
                                }
                            }

                            interval_unit_dropdown = <Dropdown> {
                                width: Fill
                                labels: ["day", "week", "month", "year"]
                                selected_item: 0
                            }
                        }

                        interval_step_check = <Check> {
                            label: "Set step"
                        }
                    }

                    // Days of Week view
                    regularity_dow = <View> {
                        width: Fill, height: Fit, visible: false
                        flow: Down, spacing: 10.0

                        dow_selector = <DayOfWeek> {
                            width: Fill
                        }
                    }

                    // Days of Month view
                    regularity_dom = <View> {
                        width: Fill, height: Fit, visible: false
                        flow: Down, spacing: 10.0

                        dom_selector = <DaysOfMonth> {
                            width: Fill
                        }

                        <TooltipTrigger> {
                            dom_carry_over_check = <Check> {
                                label: "Carry over if month has fewer days"
                            }
                        }
                    }

                    // Target Goal view
                    regularity_goal = <View> {
                        width: Fill, height: Fit, visible: false
                        flow: Down, spacing: 10.0

                        goal_type_dropdown = <Dropdown> {
                            width: Fill
                            labels: ["For a period", "By a specific date"]
                            selected_item: 0
                        }

                        // "For a period" view (default visible)
                        goal_period_view = <View> {
                            width: Fill, height: Fit
                            flow: Right, spacing: 10.0
                            align: {y: 0.5}

                            <Text> {
                                text: "Complete in"
                                draw_text: { color: (THEME_COLOR_TEXT_SECONDARY), text_style: { font_size: 14.0 } }
                            }

                            goal_period_amount = <Input> {
                                width: 80.0, height: Fit
                                is_numeric_only: true
                                placeholder: "7"
                            }

                            goal_period_unit = <Dropdown> {
                                width: Fill
                                labels: ["days", "weeks", "months", "years"]
                                selected_item: 0
                            }
                        }

                        // "By a specific date" view (hidden by default)
                        goal_date_view = <View> {
                            width: Fill, height: Fit, visible: false
                            flow: Right, spacing: 10.0
                            align: {y: 0.5}

                            <Text> {
                                text: "Complete by"
                                draw_text: { color: (THEME_COLOR_TEXT_SECONDARY), text_style: { font_size: 14.0 } }
                            }

                            goal_date_btn = <Btn> {
                                width: Fill, height: Fit
                                text: "Select date"
                            }
                        }
                    }
                }

                // Evaluation
                <Group> {
                    width: Fill, height: Fit
                    <Text> {
                        text: "Evaluation"
                        draw_text: { color: (THEME_COLOR_TEXT_PRIMARY), text_style: { font_size: 13.0 } }
                    }

                    evaluation_dropdown = <Dropdown> {
                        width: Fill
                        labels: ["Numeric", "Yes/No", "Time"]
                        selected_item: 0
                    }

                    // Numeric view (default visible)
                    evaluation_numeric = <View> {
                        width: Fill, height: Fit
                        flow: Down, spacing: 10.0

                        <View> {
                            width: Fill, height: Fit
                            flow: Right, spacing: 10.0
                            align: {y: 0.5}

                            eval_numeric_target = <Input> {
                                width: 80.0, height: Fit
                                is_numeric_only: true
                                placeholder: "1"
                            }

                            eval_numeric_unit = <Input> {
                                width: Fill, height: Fit
                                placeholder: "e.g. times, km, pages"
                            }
                        }
                    }

                    // Yes/No view (hidden by default)
                    evaluation_yesno = <View> {
                        width: Fill, height: Fit, visible: false
                        flow: Down, spacing: 10.0

                        <TooltipTrigger> {
                            eval_yesno_reverse_check = <Check> {
                                label: "Reverse behavior"
                            }
                        }
                    }

                    // Time view (hidden by default)
                    evaluation_time = <View> {
                        width: Fill, height: Fit, visible: false
                        flow: Down, spacing: 10.0

                        eval_time_btn = <Btn> {
                            width: Fill, height: Fit
                            text: "Select time"
                        }

                        <TooltipTrigger> {
                            eval_time_limit_check = <Check> {
                                label: "Limit execution to this time"
                            }
                        }
                    }
                }

                // Advanced
                <Group> {
                    <Details> {
                        width: Fill, height: Fit
                        summary: "Advanced"
                        content = <View> {
                            width: Fill, height: Fit
                            flow: Down
                            padding: 10.0
                            <Text> {
                                text: "(Empty for now)"
                                draw_text: { color: (THEME_COLOR_TEXT_TERTIARY), text_style: { font_size: 12.0 } }
                            }
                        }
                    }
                }

                // Create button
                create_btn = <Btn> {
                    width: Fill, height: Fit
                    padding: {top: 10.0, bottom: 10.0}
                    text: "Create"
                    accent: true
                    draw_bg: {
                        border_radius: 8.0
                    }
                    draw_text: {
                        color: #fff
                        text_style: { font_size: 15.0 }
                    }
                }

                // Spacer to prevent content from being hidden under Nav
                <View> { width: Fill, height: 60.0 }
            }
        }

    }
}

// Form state for create_view
#[derive(Clone, Debug)]
struct CreateActivityState {
    activity_name: String,
    start_date: (i32, u32, u32), // (year, month, day)
    goal_date: (i32, u32, u32),   // (year, month, day)

    // Regularity
    regularity_type: usize, // dropdown index: 0=Interval, 1=DaysOfWeek, 2=DaysOfMonth, 3=TargetGoal, 4=OneTime
    interval_step: String,
    interval_unit: usize, // 0=day, 1=week, 2=month, 3=year
    days_of_week_selected: Vec<usize>, // 0=Mon, 1=Tue, etc.
    days_of_month_selected: Vec<u32>,  // 1-31
    dom_carry_over: bool,
    goal_type: usize, // 0=period, 1=specific date
    goal_period_value: String,
    goal_period_unit: usize, // 0=day, 1=week, 2=month, 3=year

    // Evaluation
    evaluation_type: usize, // dropdown index: 0=Numeric, 1=YesNo, 2=Time
    eval_numeric_target: String,
    eval_numeric_unit: String,
    eval_yesno_reverse: bool,
    eval_time_value: String,
    eval_time_limit: bool,
}

impl Default for CreateActivityState {
    fn default() -> Self {
        let today = calendar::today();
        Self {
            activity_name: String::new(),
            start_date: (today.year, today.month, today.day),
            goal_date: (today.year, today.month, today.day),
            regularity_type: 0,
            interval_step: "1".to_string(),
            interval_unit: 0,
            days_of_week_selected: Vec::new(),
            days_of_month_selected: Vec::new(),
            dom_carry_over: false,
            goal_type: 0,
            goal_period_value: "1".to_string(),
            goal_period_unit: 0,
            evaluation_type: 0,
            eval_numeric_target: String::new(),
            eval_numeric_unit: String::new(),
            eval_yesno_reverse: false,
            eval_time_value: "00:00".to_string(),
            eval_time_limit: false,
        }
    }
}

#[derive(Live, LiveHook, Widget)]
pub struct ActivityScreen {
    #[deref]
    view: View,
    #[rust]
    form_state: CreateActivityState,
    #[rust]
    is_start_date: bool,
}

impl Widget for ActivityScreen {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        self.view.handle_event(cx, event, scope);

        if let Event::Actions(actions) = event {
            let mut show_create = None;

            // FAB clicked
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

            // Handle form actions when in create_view
            // Check visibility by checking if area is not Empty
            if self.view.widget(ids!(create_view)).area() != Area::Empty {
                self.handle_form_actions(cx, actions);
            }

            // Switch between list/create views
            if let Some(creating) = show_create {
                self.view.widget(ids!(list_view)).apply_over(cx, live!{ visible: (!creating) });
                self.view.widget(ids!(create_view)).apply_over(cx, live!{ visible: (creating) });

                if creating {
                    // Initialize form state when opening create view
                    self.init_form(cx);
                }

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

impl ActivityScreen {
    fn init_form(&mut self, cx: &mut Cx) {
        // Reset state to defaults
        self.form_state = CreateActivityState::default();

        // Set start date button to today's date
        let date_text = format!("{:02}.{:02}.{:04}",
            self.form_state.start_date.2,
            self.form_state.start_date.1,
            self.form_state.start_date.0
        );
        if let Some(mut btn) = self.view.widget(ids!(start_date_btn)).borrow_mut::<Button>() {
            btn.set_text(cx, &date_text);
        }

        // Clear activity name input
        if let Some(mut input) = self.view.widget(ids!(activity_name)).borrow_mut::<Input>() {
            input.set_text(cx, "");
        }

        // Clear eval numeric unit input
        if let Some(mut input) = self.view.widget(ids!(eval_numeric_unit)).borrow_mut::<Input>() {
            input.set_text(cx, "");
        }
    }

    fn handle_form_actions(&mut self, cx: &mut Cx, actions: &Actions) {
        let uid = self.widget_uid();

        // Start Date button clicked → send action to app
        if let Some(btn) = self.view.widget(ids!(start_date_btn)).borrow::<Button>() {
            if btn.clicked(actions) {
                self.is_start_date = true;
                cx.widget_action(uid, &HeapLiveIdPath::default(), ScreenAction::OpenDatePicker);
            }
        }

        // Goal date button clicked → send action to app
        if let Some(btn) = self.view.widget(ids!(goal_date_btn)).borrow::<Button>() {
            if btn.clicked(actions) {
                self.is_start_date = false;
                cx.widget_action(uid, &HeapLiveIdPath::default(), ScreenAction::OpenDatePicker);
            }
        }

        // Regularity dropdown changed
        let regularity_dd = self.view.widget(ids!(regularity_dropdown));
        if let Some(item) = actions.find_widget_action(regularity_dd.widget_uid()) {
            if let DropDownAction::Select(index, _) = item.cast() {
                // Save to form state
                self.form_state.regularity_type = index;

                // Hide all regularity views
                self.view.widget(ids!(regularity_interval)).apply_over(cx, live!{ visible: false });
                self.view.widget(ids!(regularity_dow)).apply_over(cx, live!{ visible: false });
                self.view.widget(ids!(regularity_dom)).apply_over(cx, live!{ visible: false });
                self.view.widget(ids!(regularity_goal)).apply_over(cx, live!{ visible: false });

                // Show selected view
                match index {
                    0 => self.view.widget(ids!(regularity_interval)).apply_over(cx, live!{ visible: true }),
                    1 => self.view.widget(ids!(regularity_dow)).apply_over(cx, live!{ visible: true }),
                    2 => self.view.widget(ids!(regularity_dom)).apply_over(cx, live!{ visible: true }),
                    3 => self.view.widget(ids!(regularity_goal)).apply_over(cx, live!{ visible: true }),
                    4 => {}, // One-time: no view
                    _ => {}
                }
            }
        }

        // Goal type dropdown changed (inside regularity_goal view)
        let goal_type_dd = self.view.widget(ids!(goal_type_dropdown));
        if let Some(item) = actions.find_widget_action(goal_type_dd.widget_uid()) {
            if let DropDownAction::Select(index, _) = item.cast() {
                // Save to form state
                self.form_state.goal_type = index;

                match index {
                    0 => {
                        // "For a period"
                        self.view.widget(ids!(goal_period_view)).apply_over(cx, live!{ visible: true });
                        self.view.widget(ids!(goal_date_view)).apply_over(cx, live!{ visible: false });
                    }
                    1 => {
                        // "By a specific date"
                        self.view.widget(ids!(goal_period_view)).apply_over(cx, live!{ visible: false });
                        self.view.widget(ids!(goal_date_view)).apply_over(cx, live!{ visible: true });
                    }
                    _ => {}
                }
            }
        }

        // Interval "Set step" checkbox
        let check_widget = self.view.widget(ids!(interval_step_check));
        if let Some(item) = actions.find_widget_action(check_widget.widget_uid()) {
            if let CheckAction::Changed(checked) = item.cast() {
                self.view.widget(ids!(interval_step_input_wrap)).apply_over(cx, live!{ visible: (checked) });
            }
        }

        // Yes/No "Reverse behavior" checkbox
        let reverse_check = self.view.widget(ids!(eval_yesno_reverse_check));
        if let Some(item) = actions.find_widget_action(reverse_check.widget_uid()) {
            if let CheckAction::Changed(checked) = item.cast() {
                self.form_state.eval_yesno_reverse = checked;
            }
        }

        // Time "Limit execution" checkbox
        let limit_check = self.view.widget(ids!(eval_time_limit_check));
        if let Some(item) = actions.find_widget_action(limit_check.widget_uid()) {
            if let CheckAction::Changed(checked) = item.cast() {
                self.form_state.eval_time_limit = checked;
            }
        }

        // Days of Month "Carry over" checkbox
        let carry_check = self.view.widget(ids!(dom_carry_over_check));
        if let Some(item) = actions.find_widget_action(carry_check.widget_uid()) {
            if let CheckAction::Changed(checked) = item.cast() {
                self.form_state.dom_carry_over = checked;
            }
        }

        // Evaluation dropdown changed
        let eval_dd = self.view.widget(ids!(evaluation_dropdown));
        if let Some(item) = actions.find_widget_action(eval_dd.widget_uid()) {
            if let DropDownAction::Select(index, _) = item.cast() {
                // Save to form state
                self.form_state.evaluation_type = index;

                // Hide all evaluation views
                self.view.widget(ids!(evaluation_numeric)).apply_over(cx, live!{ visible: false });
                self.view.widget(ids!(evaluation_yesno)).apply_over(cx, live!{ visible: false });
                self.view.widget(ids!(evaluation_time)).apply_over(cx, live!{ visible: false });

                // Show selected view
                match index {
                    0 => self.view.widget(ids!(evaluation_numeric)).apply_over(cx, live!{ visible: true }),
                    1 => self.view.widget(ids!(evaluation_yesno)).apply_over(cx, live!{ visible: true }),
                    2 => self.view.widget(ids!(evaluation_time)).apply_over(cx, live!{ visible: true }),
                    _ => {}
                }
            }
        }

        // Time evaluation button clicked → send action to app
        if let Some(btn) = self.view.widget(ids!(eval_time_btn)).borrow::<Button>() {
            if btn.clicked(actions) {
                cx.widget_action(uid, &HeapLiveIdPath::default(), ScreenAction::OpenTimePicker);
            }
        }

        // Create button clicked → collect all form data and log as JSON
        if let Some(btn) = self.view.widget(ids!(create_btn)).borrow::<Button>() {
            if btn.clicked(actions) {
                self.collect_and_log_form_data();
            }
        }

        // Tooltip triggers - determine which one by checking visible views
        for action in actions {
            if let TooltipTriggerAction::ShowTooltip = action.as_widget_action().cast() {
                // Check which view is visible to determine tooltip content
                let dom_visible = self.view.widget(ids!(regularity_dom)).area() != Area::Empty;
                let yesno_visible = self.view.widget(ids!(evaluation_yesno)).area() != Area::Empty;
                let time_visible = self.view.widget(ids!(evaluation_time)).area() != Area::Empty;

                if dom_visible {
                    cx.widget_action(uid, &HeapLiveIdPath::default(), ScreenAction::OpenTooltip {
                        title: "Activity Carryover".to_string(),
                        description: "If month has 30 days and you selected 31st, activity moves to 30th when checked. Otherwise skipped.".to_string(),
                    });
                } else if yesno_visible {
                    cx.widget_action(uid, &HeapLiveIdPath::default(), ScreenAction::OpenTooltip {
                        title: "Reverse Behavior".to_string(),
                        description: "When checked, not marking = success. Useful for tracking failures on bad habits.".to_string(),
                    });
                } else if time_visible {
                    cx.widget_action(uid, &HeapLiveIdPath::default(), ScreenAction::OpenTooltip {
                        title: "Time Limit".to_string(),
                        description: "When checked, timer stops at limit with alert. Otherwise runs like stopwatch. Useful for max effort timed activities.".to_string(),
                    });
                }
            }
        }
    }

    pub fn handle_date_selected(&mut self, cx: &mut Cx, year: i32, month: u32, day: u32) {
        let date_text = format!("{:02}.{:02}.{:04}", day, month, year);

        if self.is_start_date {
            self.form_state.start_date = (year, month, day);
            if let Some(mut btn) = self.view.widget(ids!(start_date_btn)).borrow_mut::<Button>() {
                btn.set_text(cx, &date_text);
            }
        } else {
            self.form_state.goal_date = (year, month, day);
            if let Some(mut btn) = self.view.widget(ids!(goal_date_btn)).borrow_mut::<Button>() {
                btn.set_text(cx, &date_text);
            }
        }
    }

    pub fn handle_time_selected(&mut self, cx: &mut Cx, hours: i32, minutes: i32, seconds: Option<i32>) {
        let time_text = if let Some(s) = seconds {
            format!("{:02}:{:02}:{:02}", hours, minutes, s)
        } else {
            format!("{:02}:{:02}", hours, minutes)
        };

        // Save to form state
        self.form_state.eval_time_value = time_text.clone();

        if let Some(mut btn) = self.view.widget(ids!(eval_time_btn)).borrow_mut::<Button>() {
            btn.set_text(cx, &time_text);
        }
    }

    fn collect_and_log_form_data(&mut self) {
        // Collect current widget states before logging
        self.collect_widget_states();

        let (year, month, day) = self.form_state.start_date;
        let (gy, gm, gd) = self.form_state.goal_date;

        let regularity = REGULARITY_TYPES.get(self.form_state.regularity_type).unwrap_or(&"Unknown");

        let interval_unit = TIME_UNITS.get(self.form_state.interval_unit).unwrap_or(&"day");

        let evaluation = EVALUATION_TYPES.get(self.form_state.evaluation_type).unwrap_or(&"Unknown");

        // Build regularity data string based on type
        let regularity_data = match self.form_state.regularity_type {
            0 => format!("each {} {}", self.form_state.interval_step, interval_unit),
            1 => {
                let days: Vec<&str> = self.form_state.days_of_week_selected.iter()
                    .filter_map(|&i| DAY_NAMES.get(i))
                    .copied()
                    .collect();
                format!("days_of_week: {:?}", days)
            },
            2 => format!("days_of_month: {:?}, carry_over: {}", self.form_state.days_of_month_selected, self.form_state.dom_carry_over),
            3 => {
                if self.form_state.goal_type == 0 {
                    // For a period
                    let period_unit = TIME_UNITS.get(self.form_state.goal_period_unit).unwrap_or(&"week");
                    format!("period: {} {}", self.form_state.goal_period_value, period_unit)
                } else {
                    // By a specific date
                    format!("goal_date: {:04}-{:02}-{:02}", gy, gm, gd)
                }
            },
            4 => "one-time".to_string(),
            _ => "unknown".to_string(),
        };

        // Build evaluation data string based on type
        let evaluation_data = match self.form_state.evaluation_type {
            0 => format!("target: {}, unit: {}", self.form_state.eval_numeric_target, self.form_state.eval_numeric_unit),
            1 => format!("reverse: {}", self.form_state.eval_yesno_reverse),
            2 => format!("time: {}, limit: {}", self.form_state.eval_time_value, self.form_state.eval_time_limit),
            _ => "unknown".to_string(),
        };

        log!(
            r#"Create Activity - Form Data:
{{
  "activity_name": "{}",
  "start_date": "{:04}-{:02}-{:02}",
  "regularity": {{
    "type": "{}",
    "data": "{}"
  }},
  "evaluation": {{
    "type": "{}",
    "data": "{}"
  }}
}}"#,
            self.form_state.activity_name,
            year, month, day,
            regularity,
            regularity_data,
            evaluation,
            evaluation_data
        );
    }

    fn collect_widget_states(&mut self) {
        // Collect activity name
        if let Some(input) = self.view.widget(ids!(activity_name)).borrow::<Input>() {
            self.form_state.activity_name = input.text();
        }

        // Collect interval step
        if let Some(input) = self.view.widget(ids!(interval_step_input)).borrow::<Input>() {
            self.form_state.interval_step = input.text();
        }

        // Collect interval unit
        self.form_state.interval_unit = self.view.drop_down(ids!(interval_unit_dropdown)).selected_item();

        // Collect DaysOfWeek selected days
        if let Some(dow) = self.view.widget(ids!(dow_selector)).borrow::<DayOfWeek>() {
            self.form_state.days_of_week_selected = dow.get_selected_days();
        }

        // Collect DaysOfMonth selected days
        if let Some(dom) = self.view.widget(ids!(dom_selector)).borrow::<DaysOfMonth>() {
            let selected = dom.get_selected_days();
            self.form_state.days_of_month_selected = selected.iter().map(|&i| (i + 1) as u32).collect();
        }

        // Note: dom_carry_over is saved via event handler

        // Collect goal period amount
        if let Some(input) = self.view.widget(ids!(goal_period_amount)).borrow::<Input>() {
            self.form_state.goal_period_value = input.text();
        }

        // Collect goal period unit
        self.form_state.goal_period_unit = self.view.drop_down(ids!(goal_period_unit)).selected_item();

        // Collect numeric evaluation fields
        if let Some(input) = self.view.widget(ids!(eval_numeric_target)).borrow::<Input>() {
            self.form_state.eval_numeric_target = input.text();
        }
        if let Some(input) = self.view.widget(ids!(eval_numeric_unit)).borrow::<Input>() {
            self.form_state.eval_numeric_unit = input.text();
        }

        // Note: Checkboxes (reverse, time_limit, carry_over) are saved via event handlers
    }
}
