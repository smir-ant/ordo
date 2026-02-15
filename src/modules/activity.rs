use makepad_widgets::*;
use makepad_widgets::drop_down::DropDownAction;
use crate::widgets::button::Button;
use crate::widgets::check::{Check, CheckAction};
use crate::widgets::modal::TooltipTriggerAction;
use crate::header::{HeaderAction, MenuItem, AppAction};
use crate::widgets::input::Input;
use crate::utils::calendar;

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
    use makepad_widgets::drop_down::DropDown;
    use makepad_draw::shader::std::*;
    use link::styling::*;

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
                        empty_text: "Morning routine"
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
                    regularity_dropdown = <DropDown> {
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
                                    text: "1"
                                }
                            }

                            interval_unit_dropdown = <DropDown> {
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

                        goal_type_dropdown = <DropDown> {
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
                                text: "7"
                            }

                            goal_period_unit = <DropDown> {
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
            }
        }

    }
}

// Form state for create_view
#[derive(Clone, Debug)]
struct CreateActivityState {
    activity_name: String,
    start_date: (i32, u32, u32), // (year, month, day)
}

impl Default for CreateActivityState {
    fn default() -> Self {
        let today = calendar::today();
        Self {
            activity_name: String::new(),
            start_date: (today.year, today.month, today.day),
        }
    }
}

#[derive(Clone, Copy, Debug)]
enum DatePickerTarget {
    StartDate,
    GoalDate,
}

impl Default for DatePickerTarget {
    fn default() -> Self {
        Self::StartDate
    }
}

#[derive(Live, LiveHook, Widget)]
pub struct ActivityScreen {
    #[deref]
    view: View,
    #[rust]
    form_state: CreateActivityState,
    #[rust]
    date_picker_target: DatePickerTarget,
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
    }

    fn handle_form_actions(&mut self, cx: &mut Cx, actions: &Actions) {
        let uid = self.widget_uid();

        // Start Date button clicked → send action to app
        if let Some(btn) = self.view.widget(ids!(start_date_btn)).borrow::<Button>() {
            if btn.clicked(actions) {
                self.date_picker_target = DatePickerTarget::StartDate;
                cx.widget_action(uid, &HeapLiveIdPath::default(), AppAction::OpenDatePicker);
            }
        }

        // Goal date button clicked → send action to app
        if let Some(btn) = self.view.widget(ids!(goal_date_btn)).borrow::<Button>() {
            if btn.clicked(actions) {
                self.date_picker_target = DatePickerTarget::GoalDate;
                cx.widget_action(uid, &HeapLiveIdPath::default(), AppAction::OpenDatePicker);
            }
        }

        // Regularity dropdown changed
        let regularity_dd = self.view.widget(ids!(regularity_dropdown));
        if let Some(item) = actions.find_widget_action(regularity_dd.widget_uid()) {
            if let DropDownAction::Select(index, _) = item.cast() {
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

        // Tooltip trigger for "Carry over" checkbox
        for action in actions {
            if let TooltipTriggerAction::ShowTooltip = action.as_widget_action().cast() {
                cx.widget_action(uid, &HeapLiveIdPath::default(), AppAction::OpenTooltip {
                    title: "Activity Carryover".to_string(),
                    description: "If month has 30 days and you selected 31st, activity moves to 30th when checked. Otherwise skipped.".to_string(),
                });
            }
        }
    }

    pub fn handle_date_selected(&mut self, cx: &mut Cx, year: i32, month: u32, day: u32) {
        let date_text = format!("{:02}.{:02}.{:04}", day, month, year);

        match self.date_picker_target {
            DatePickerTarget::StartDate => {
                self.form_state.start_date = (year, month, day);
                if let Some(mut btn) = self.view.widget(ids!(start_date_btn)).borrow_mut::<Button>() {
                    btn.set_text(cx, &date_text);
                }
            }
            DatePickerTarget::GoalDate => {
                // TODO: save goal date to form state when we expand CreateActivityState
                if let Some(mut btn) = self.view.widget(ids!(goal_date_btn)).borrow_mut::<Button>() {
                    btn.set_text(cx, &date_text);
                }
            }
        }
    }
}
