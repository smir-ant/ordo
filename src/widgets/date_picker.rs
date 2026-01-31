use makepad_widgets::*;
use crate::widgets::modal::{Modal, ModalAction};
use crate::widgets::button::{Button as Btn, ButtonAction};
use crate::widgets::wheel_h::{WheelH, WheelHAction};
use crate::utils::calendar::{self, WeekStart, SimpleDate, MONTH_NAMES_SHORT};

live_design! {
    use makepad_widgets::base::*;
    use makepad_widgets::theme_desktop_dark::*;
    use makepad_widgets::view_ui::View;
    use makepad_draw::shader::std::*;
    use link::styling::*;
    use crate::widgets::text::Text;
    use crate::widgets::button::Btn;
    use crate::widgets::modal::Modal;
    use crate::widgets::modal::DialogStyle;
    use crate::widgets::wheel_h::WheelH;

    // Day cell in calendar grid
    DayCell = {{DayCell}} {
        width: 36.0, height: 36.0
        align: {x: 0.5, y: 0.5}

        draw_bg: {
            instance color: #0000
            instance selected: 0.0
            instance hover: 0.0
            uniform accent_color: (THEME_COLOR_ACCENT)

            fn pixel(self) -> vec4 {
                let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                let radius = min(self.rect_size.x, self.rect_size.y) * 0.45;
                let center = self.rect_size * 0.5;

                sdf.circle(center.x, center.y, radius);

                // Selected state uses accent color
                let bg = mix(self.color, self.accent_color, self.selected);
                // Hover state (subtle)
                let bg = mix(bg, mix(self.color, #fff, 0.1), self.hover * (1.0 - self.selected));

                sdf.fill(bg);
                return sdf.result;
            }
        }

        draw_text: {
            text_style: <THEME_FONT_REGULAR> { font_size: 14.0 }
            color: #fff
            instance selected: 0.0
            instance is_today: 0.0
            uniform accent_color: (THEME_COLOR_ACCENT)

            fn get_color(self) -> vec4 {
                // Today but not selected - accent text
                let today_color = mix(self.color, self.accent_color, self.is_today * (1.0 - self.selected));
                return today_color;
            }
        }
    }

    // Day of week header cell
    DayHeader = <View> {
        width: 36.0, height: 28.0
        align: {x: 0.5, y: 0.5}

        label = <Text> {
            width: Fit, height: Fit
            draw_text: {
                text_style: <THEME_FONT_REGULAR> { font_size: 12.0 }
                color: #888
            }
        }
    }

    pub DatePicker = {{DatePicker}} {
        width: Fill, height: Fill
        flow: Overlay

        // Configuration
        week_start: Monday

        modal = <Modal> {
            content = <DialogStyle> {
                width: Fit
                spacing: 12.0

                title = <Text> {
                    width: Fit, height: Fit
                    draw_text: {
                        text_style: <THEME_FONT_BOLD> { font_size: 14.0 }
                        color: #fff
                    }
                    text: "Select Date"
                }

                // Year picker
                year_picker = <WheelH> {
                    width: 280.0, height: 40.0
                    step_width: 80.0
                    range_min: 2001
                    range_max: 2051
                    initial_value: 2026
                    is_infinite: false
                }

                // Month picker
                month_picker = <WheelH> {
                    width: 280.0, height: 40.0
                    step_width: 80.0
                    is_infinite: true
                }

                // Calendar grid container
                calendar_wrap = <View> {
                    width: 280.0, height: Fit
                    flow: Down
                    spacing: 2.0

                    // Day of week headers
                    header_row = <View> {
                        width: Fill, height: Fit
                        flow: Right
                        spacing: 2.0

                        d0 = <DayHeader> {}
                        d1 = <DayHeader> {}
                        d2 = <DayHeader> {}
                        d3 = <DayHeader> {}
                        d4 = <DayHeader> {}
                        d5 = <DayHeader> {}
                        d6 = <DayHeader> {}
                    }

                    // Calendar rows (max 6 rows needed for any month)
                    row0 = <View> {
                        width: Fill, height: Fit
                        flow: Right
                        spacing: 2.0
                        c0_0 = <DayCell> {} c0_1 = <DayCell> {} c0_2 = <DayCell> {}
                        c0_3 = <DayCell> {} c0_4 = <DayCell> {} c0_5 = <DayCell> {} c0_6 = <DayCell> {}
                    }
                    row1 = <View> {
                        width: Fill, height: Fit
                        flow: Right
                        spacing: 2.0
                        c1_0 = <DayCell> {} c1_1 = <DayCell> {} c1_2 = <DayCell> {}
                        c1_3 = <DayCell> {} c1_4 = <DayCell> {} c1_5 = <DayCell> {} c1_6 = <DayCell> {}
                    }
                    row2 = <View> {
                        width: Fill, height: Fit
                        flow: Right
                        spacing: 2.0
                        c2_0 = <DayCell> {} c2_1 = <DayCell> {} c2_2 = <DayCell> {}
                        c2_3 = <DayCell> {} c2_4 = <DayCell> {} c2_5 = <DayCell> {} c2_6 = <DayCell> {}
                    }
                    row3 = <View> {
                        width: Fill, height: Fit
                        flow: Right
                        spacing: 2.0
                        c3_0 = <DayCell> {} c3_1 = <DayCell> {} c3_2 = <DayCell> {}
                        c3_3 = <DayCell> {} c3_4 = <DayCell> {} c3_5 = <DayCell> {} c3_6 = <DayCell> {}
                    }
                    row4 = <View> {
                        width: Fill, height: Fit
                        flow: Right
                        spacing: 2.0
                        c4_0 = <DayCell> {} c4_1 = <DayCell> {} c4_2 = <DayCell> {}
                        c4_3 = <DayCell> {} c4_4 = <DayCell> {} c4_5 = <DayCell> {} c4_6 = <DayCell> {}
                    }
                    row5 = <View> {
                        width: Fill, height: Fit
                        flow: Right
                        spacing: 2.0
                        c5_0 = <DayCell> {} c5_1 = <DayCell> {} c5_2 = <DayCell> {}
                        c5_3 = <DayCell> {} c5_4 = <DayCell> {} c5_5 = <DayCell> {} c5_6 = <DayCell> {}
                    }
                }

                buttons_wrap = <View> {
                    width: Fill, height: Fit
                    flow: Right
                    spacing: 10.0
                    margin: {top: 5.0}

                    today_button = <Btn> { text: "Today" }

                    // Spacer pushes cancel/ok to the right
                    <View> { width: Fill, height: 1.0 }

                    cancel_button = <Btn> { text: "Cancel" }
                    ok_button = <Btn> { text: "OK", accent: true }
                }
            }
        }
    }
}

#[derive(Clone, Debug, DefaultNone)]
pub enum DatePickerAction {
    None,
    Dismissed,
    Accepted { year: i32, month: u32, day: u32 },
}

#[derive(Live, LiveHook, Widget)]
pub struct DatePicker {
    #[deref] view: View,
    #[live] week_start: WeekStart,

    // Currently displayed month/year (changes on wheel scroll)
    #[rust] displayed_year: i32,
    #[rust] displayed_month: u32,  // 1-12

    // Actually selected date (changes only on day click)
    #[rust] selected_year: i32,
    #[rust] selected_month: u32,  // 1-12
    #[rust] selected_day: u32,    // 1-31

    #[rust] today: SimpleDate,
    #[rust] initialized: bool,
    #[rust] year_picker_uid: Option<WidgetUid>,
    #[rust] month_picker_uid: Option<WidgetUid>,
    #[rust] today_button_uid: Option<WidgetUid>,

    // Calendar swipe tracking
    #[rust] swipe_start_x: f64,
    #[rust] is_swiping: bool,
}

// Make WeekStart work with live_design
impl LiveHook for WeekStart {}
impl LiveRead for WeekStart {
    fn live_read_to(&self, id: LiveId, out: &mut Vec<LiveNode>) {
        let atom = match self {
            WeekStart::Monday => live_id!(Monday),
            WeekStart::Sunday => live_id!(Sunday),
        };
        out.push(LiveNode::from_id_value(id, LiveValue::Id(atom)));
    }
}
impl LiveApply for WeekStart {
    fn apply(&mut self, _cx: &mut Cx, _apply: &mut Apply, index: usize, nodes: &[LiveNode]) -> usize {
        if let LiveValue::Id(id) = &nodes[index].value {
            *self = if *id == live_id!(Sunday) {
                WeekStart::Sunday
            } else {
                WeekStart::Monday
            };
        }
        index + 1
    }
}
impl LiveNew for WeekStart {
    fn live_type_info(_cx: &mut Cx) -> LiveTypeInfo {
        LiveTypeInfo {
            module_id: LiveModuleId::from_str(&module_path!()).unwrap(),
            live_type: std::any::TypeId::of::<Self>(),
            live_ignore: true,
            fields: Vec::new(),
            type_name: live_id!(WeekStart),
        }
    }
    fn new(_cx: &mut Cx) -> Self { WeekStart::Monday }
}

// DayCell widget for calendar days
#[derive(Live, LiveHook, Widget)]
pub struct DayCell {
    #[redraw] #[live] draw_bg: DrawQuad,
    #[redraw] #[live] draw_text: DrawText,
    #[layout] layout: Layout,
    #[walk] walk: Walk,

    #[rust] day: u32,
    #[rust] is_selected: bool,
    #[rust] is_today: bool,
    #[rust] is_visible: bool,
    #[rust] is_hovered: bool,
}

impl Widget for DayCell {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        if !self.is_visible { return; }

        match event.hits(cx, self.draw_bg.area()) {
            Hit::FingerHoverIn(_) => {
                self.is_hovered = true;
                self.draw_bg.redraw(cx);
            }
            Hit::FingerHoverOut(_) => {
                self.is_hovered = false;
                self.draw_bg.redraw(cx);
            }
            Hit::FingerUp(fe) => {
                if fe.is_over && self.day > 0 {
                    cx.widget_action(self.widget_uid(), &scope.path, DayCellAction::Clicked(self.day));
                }
            }
            _ => {}
        }
    }

    fn draw_walk(&mut self, cx: &mut Cx2d, _scope: &mut Scope, walk: Walk) -> DrawStep {
        // Always reset shader instance values to current state
        self.draw_bg.apply_over(cx, live!{
            selected: (if self.is_selected && self.is_visible { 1.0 } else { 0.0 })
            hover: (if self.is_hovered && self.is_visible { 1.0 } else { 0.0 })
        });

        if !self.is_visible {
            // Draw invisible placeholder to maintain grid
            self.draw_bg.begin(cx, walk, self.layout);
            self.draw_bg.end(cx);
            return DrawStep::done();
        }

        self.draw_text.apply_over(cx, live!{
            selected: (if self.is_selected { 1.0 } else { 0.0 })
            is_today: (if self.is_today { 1.0 } else { 0.0 })
        });

        self.draw_bg.begin(cx, walk, self.layout);

        let text = format!("{}", self.day);
        self.draw_text.draw_walk(cx, Walk::fit(), Align::default(), &text);

        self.draw_bg.end(cx);
        DrawStep::done()
    }
}

#[derive(Clone, Debug, DefaultNone)]
pub enum DayCellAction {
    None,
    Clicked(u32),
}

impl DatePicker {
    fn modal_ref(&self) -> WidgetRef {
        self.view.widget(ids!(modal))
    }

    pub fn is_open(&self) -> bool {
        self.modal_ref().borrow::<Modal>().map(|m| m.is_open()).unwrap_or(false)
    }

    pub fn open(&mut self, cx: &mut Cx) {
        // Initialize to today on first open
        if !self.initialized {
            self.today = calendar::today();
            self.selected_year = self.today.year;
            self.selected_month = self.today.month;
            self.selected_day = self.today.day;
            self.initialized = true;
        }

        // Set displayed to selected date
        self.displayed_year = self.selected_year;
        self.displayed_month = self.selected_month;

        // Cache widget UIDs to avoid borrow conflicts in handle_event
        let content = self.modal_ref().widget(ids!(content));
        if let Some(picker) = content.widget(ids!(year_picker)).borrow::<WheelH>() {
            self.year_picker_uid = Some(picker.widget_uid());
        }
        if let Some(picker) = content.widget(ids!(month_picker)).borrow::<WheelH>() {
            self.month_picker_uid = Some(picker.widget_uid());
        }
        if let Some(btn) = content.widget(ids!(buttons_wrap)).widget(ids!(today_button)).borrow::<Btn>() {
            self.today_button_uid = Some(btn.widget_uid());
        }

        // Setup pickers
        self.setup_month_picker(cx);
        self.sync_pickers_to_displayed(cx);
        self.update_calendar_grid(cx);

        if let Some(mut modal) = self.modal_ref().borrow_mut::<Modal>() {
            modal.open(cx);
        }
    }

    pub fn close(&mut self, cx: &mut Cx) {
        if let Some(mut modal) = self.modal_ref().borrow_mut::<Modal>() {
            modal.close(cx);
        }
    }

    fn setup_month_picker(&mut self, _cx: &mut Cx) {
        let month_labels: Vec<String> = MONTH_NAMES_SHORT.iter().map(|s| s.to_string()).collect();
        let content = self.modal_ref().widget(ids!(content));
        if let Some(mut picker) = content.widget(ids!(month_picker)).borrow_mut::<WheelH>() {
            picker.set_labels(month_labels);
        }
    }

    fn sync_pickers_to_displayed(&mut self, cx: &mut Cx) {
        let content = self.modal_ref().widget(ids!(content));

        if let Some(mut picker) = content.widget(ids!(year_picker)).borrow_mut::<WheelH>() {
            picker.set_value(cx, self.displayed_year);
        }

        if let Some(mut picker) = content.widget(ids!(month_picker)).borrow_mut::<WheelH>() {
            picker.set_value(cx, (self.displayed_month - 1) as i32); // 0-indexed
        }
    }

    fn update_calendar_grid(&mut self, cx: &mut Cx) {
        let content = self.modal_ref().widget(ids!(content));
        let calendar_wrap = content.widget(ids!(calendar_wrap));

        // Update header labels
        let day_names = calendar::day_names_short(self.week_start);
        let header_row = calendar_wrap.widget(ids!(header_row));
        for (i, name) in day_names.iter().enumerate() {
            let header_id = match i {
                0 => ids!(d0), 1 => ids!(d1), 2 => ids!(d2), 3 => ids!(d3),
                4 => ids!(d4), 5 => ids!(d5), _ => ids!(d6),
            };
            if let Some(mut text) = header_row.widget(header_id).widget(ids!(label)).borrow_mut::<crate::widgets::text::Text>() {
                text.set_text(cx, name);
            }
        }

        // Get calendar grid data for DISPLAYED month/year
        let grid = calendar::calendar_grid(self.displayed_year, self.displayed_month, self.week_start);

        // Check if displayed month matches selected month (for showing selection)
        let is_selected_month = self.displayed_year == self.selected_year
            && self.displayed_month == self.selected_month;

        // Check if displayed month is today's month (for showing today marker)
        let is_today_month = self.displayed_year == self.today.year
            && self.displayed_month == self.today.month;

        // Create a lookup: (row, col) -> day
        let mut day_map = [[0u32; 7]; 6];
        for &(day, row, col) in &grid {
            day_map[row as usize][col as usize] = day;
        }

        // Update each cell (always show all 6 rows for consistent height)
        for row in 0..6u32 {
            let row_id = match row {
                0 => ids!(row0), 1 => ids!(row1), 2 => ids!(row2),
                3 => ids!(row3), 4 => ids!(row4), _ => ids!(row5),
            };

            let row_widget = calendar_wrap.widget(row_id);

            for col in 0..7u32 {
                let cell_id = match (row, col) {
                    (0, 0) => ids!(c0_0), (0, 1) => ids!(c0_1), (0, 2) => ids!(c0_2),
                    (0, 3) => ids!(c0_3), (0, 4) => ids!(c0_4), (0, 5) => ids!(c0_5), (0, 6) => ids!(c0_6),
                    (1, 0) => ids!(c1_0), (1, 1) => ids!(c1_1), (1, 2) => ids!(c1_2),
                    (1, 3) => ids!(c1_3), (1, 4) => ids!(c1_4), (1, 5) => ids!(c1_5), (1, 6) => ids!(c1_6),
                    (2, 0) => ids!(c2_0), (2, 1) => ids!(c2_1), (2, 2) => ids!(c2_2),
                    (2, 3) => ids!(c2_3), (2, 4) => ids!(c2_4), (2, 5) => ids!(c2_5), (2, 6) => ids!(c2_6),
                    (3, 0) => ids!(c3_0), (3, 1) => ids!(c3_1), (3, 2) => ids!(c3_2),
                    (3, 3) => ids!(c3_3), (3, 4) => ids!(c3_4), (3, 5) => ids!(c3_5), (3, 6) => ids!(c3_6),
                    (4, 0) => ids!(c4_0), (4, 1) => ids!(c4_1), (4, 2) => ids!(c4_2),
                    (4, 3) => ids!(c4_3), (4, 4) => ids!(c4_4), (4, 5) => ids!(c4_5), (4, 6) => ids!(c4_6),
                    (5, 0) => ids!(c5_0), (5, 1) => ids!(c5_1), (5, 2) => ids!(c5_2),
                    (5, 3) => ids!(c5_3), (5, 4) => ids!(c5_4), (5, 5) => ids!(c5_5), _ => ids!(c5_6),
                };

                if let Some(mut cell) = row_widget.widget(cell_id).borrow_mut::<DayCell>() {
                    let day = day_map[row as usize][col as usize];
                    cell.day = day;
                    cell.is_visible = day > 0;
                    // Only show selection if this is the selected month/year
                    cell.is_selected = is_selected_month && day == self.selected_day && day > 0;
                    // Only show today marker if this is today's month/year
                    cell.is_today = is_today_month && day == self.today.day && day > 0;
                    cell.is_hovered = false;
                }
            }
        }

        self.view.redraw(cx);
    }

    fn get_selected_date(&self) -> (i32, u32, u32) {
        (self.selected_year, self.selected_month, self.selected_day)
    }

    fn handle_calendar_swipe(&mut self, cx: &mut Cx, event: &Event) {
        let content = self.modal_ref().widget(ids!(content));
        let calendar_rect = content.widget(ids!(calendar_wrap)).area().rect(cx);
        let swipe_threshold = 50.0; // Minimum horizontal distance to trigger month change

        // Handle touch events (track without consuming)
        if let Event::TouchUpdate(tu) = event {
            for touch in &tu.touches {
                match touch.state {
                    makepad_widgets::event::TouchState::Start => {
                        if calendar_rect.contains(touch.abs) {
                            self.is_swiping = true;
                            self.swipe_start_x = touch.abs.x;
                        }
                    }
                    makepad_widgets::event::TouchState::Stop => {
                        if self.is_swiping {
                            let delta = touch.abs.x - self.swipe_start_x;
                            if delta.abs() >= swipe_threshold {
                                self.finish_swipe(cx, delta, swipe_threshold);
                            }
                            self.is_swiping = false;
                        }
                    }
                    _ => {}
                }
            }
        }

        // Handle mouse events (track without consuming - don't use hits())
        if let Event::MouseDown(e) = event {
            if calendar_rect.contains(e.abs) {
                self.is_swiping = true;
                self.swipe_start_x = e.abs.x;
            }
        }

        if let Event::MouseUp(e) = event {
            if self.is_swiping {
                let delta = e.abs.x - self.swipe_start_x;
                if delta.abs() >= swipe_threshold {
                    self.finish_swipe(cx, delta, swipe_threshold);
                }
                self.is_swiping = false;
            }
        }
    }

    fn finish_swipe(&mut self, cx: &mut Cx, delta: f64, threshold: f64) {
        if delta.abs() < threshold {
            return;
        }

        if delta > 0.0 {
            // Swipe right = previous month
            self.go_to_previous_month(cx);
        } else {
            // Swipe left = next month
            self.go_to_next_month(cx);
        }
    }

    fn go_to_previous_month(&mut self, cx: &mut Cx) {
        if self.displayed_month == 1 {
            self.displayed_month = 12;
            self.displayed_year -= 1;
        } else {
            self.displayed_month -= 1;
        }
        self.sync_pickers_to_displayed(cx);
        self.update_calendar_grid(cx);
    }

    fn go_to_next_month(&mut self, cx: &mut Cx) {
        if self.displayed_month == 12 {
            self.displayed_month = 1;
            self.displayed_year += 1;
        } else {
            self.displayed_month += 1;
        }
        self.sync_pickers_to_displayed(cx);
        self.update_calendar_grid(cx);
    }

    pub fn set_date(&mut self, cx: &mut Cx, year: i32, month: u32, day: u32) {
        self.selected_year = year;
        self.selected_month = month.clamp(1, 12);
        self.selected_day = day.clamp(1, calendar::days_in_month(year, self.selected_month));

        // Also update displayed to match
        self.displayed_year = self.selected_year;
        self.displayed_month = self.selected_month;

        if self.is_open() {
            self.sync_pickers_to_displayed(cx);
            self.update_calendar_grid(cx);
        }
    }
}

impl Widget for DatePicker {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        let uid = self.widget_uid();
        let modal_uid = self.modal_ref().borrow::<Modal>().map(|m| m.widget_uid());

        // Handle calendar swipe for month navigation
        self.handle_calendar_swipe(cx, event);

        self.view.handle_event(cx, event, scope);

        if let Event::Actions(actions) = event {
            // Handle modal accept/dismiss
            if let Some(modal_uid) = modal_uid {
                if let Some(action) = actions.find_widget_action(modal_uid) {
                    match action.cast() {
                        ModalAction::Accepted => {
                            let (year, month, day) = self.get_selected_date();
                            log!("Date: {}.{}.{}", day, month, year);
                            cx.widget_action(uid, &scope.path, DatePickerAction::Accepted { year, month, day });
                        }
                        ModalAction::Dismissed => {
                            cx.widget_action(uid, &scope.path, DatePickerAction::Dismissed);
                        }
                        _ => {}
                    }
                }
            }

            // Handle WheelH changes and button clicks using cached UIDs
            for action in actions {
                if let WheelHAction::Changed(val) = action.as_widget_action().cast() {
                    let Some(widget_action) = action.as_widget_action() else { continue };
                    let action_uid = widget_action.widget_uid;

                    // Check year picker - changes displayed year
                    if Some(action_uid) == self.year_picker_uid {
                        self.displayed_year = val;
                        self.update_calendar_grid(cx);
                    }

                    // Check month picker - changes displayed month
                    if Some(action_uid) == self.month_picker_uid {
                        self.displayed_month = (val + 1) as u32; // Convert 0-indexed to 1-indexed
                        self.update_calendar_grid(cx);
                    }
                }

                // Handle day cell clicks - sets SELECTED date to displayed month/year + clicked day
                if let DayCellAction::Clicked(day) = action.as_widget_action().cast() {
                    self.selected_year = self.displayed_year;
                    self.selected_month = self.displayed_month;
                    self.selected_day = day;
                    self.update_calendar_grid(cx);
                }

                // Handle Today button click
                if let ButtonAction::Clicked(_) = action.as_widget_action().cast() {
                    let Some(widget_action) = action.as_widget_action() else { continue };
                    if Some(widget_action.widget_uid) == self.today_button_uid {
                        // Update today in case date changed
                        self.today = calendar::today();
                        // Set both displayed and selected to today
                        self.displayed_year = self.today.year;
                        self.displayed_month = self.today.month;
                        self.selected_year = self.today.year;
                        self.selected_month = self.today.month;
                        self.selected_day = self.today.day;
                        // Update pickers and grid
                        self.sync_pickers_to_displayed(cx);
                        self.update_calendar_grid(cx);
                    }
                }
            }
        }
    }

    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        self.view.draw_walk(cx, scope, walk)
    }
}
