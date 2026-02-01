//! Modal date picker with year/month wheels and calendar grid.
//!
//! # Architecture
//! - `displayed_year/month`: Currently visible in wheels and calendar
//! - `selected_year/month/day`: Actually chosen date (updated on day click)
//! - Year wheel: finite range (2001-2051)
//! - Month wheel: infinite (wraps Dec→Jan)
//!
//! # Swipe Sync Mechanism
//! Calendar swipe → WheelH.apply_scroll_delta() → get_visual_value() → update calendar
//! On release → WheelH.trigger_snap() → emits Changed → final calendar update

use makepad_widgets::*;
use crate::widgets::modal::{Modal, ModalAction};
use crate::widgets::button::{Button as Btn, ButtonAction};
use crate::widgets::wheel::{Wheel, WheelAction};
use crate::utils::calendar::{self, WeekStart, SimpleDate, MONTH_NAMES_SHORT};

live_design! {
    use makepad_widgets::base::*;
    use makepad_widgets::theme_desktop_dark::*;
    use makepad_widgets::view_ui::View;
    use makepad_draw::shader::std::*;
    use crate::styling::*;
    use crate::widgets::text::Text;
    use crate::widgets::button::Btn;
    use crate::widgets::modal::Modal;
    use crate::widgets::modal::DialogStyle;
    use crate::widgets::wheel::WheelH;

    DayCell = {{DayCell}} {
        width: 36.0, height: 36.0
        align: {x: 0.5, y: 0.5}

        draw_bg: {
            instance color: #0000
            instance selected: 0.0
            instance hover: 0.0
            uniform accent_color: (THEME_COLOR_ACCENT)
            uniform hover_tint: (THEME_COLOR_TEXT_PRIMARY)

            fn pixel(self) -> vec4 {
                let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                let radius = min(self.rect_size.x, self.rect_size.y) * 0.45;
                let center = self.rect_size * 0.5;
                sdf.circle(center.x, center.y, radius);
                let bg = mix(self.color, self.accent_color, self.selected);
                let bg = mix(bg, mix(self.color, self.hover_tint, 0.1), self.hover * (1.0 - self.selected));
                sdf.fill(bg);
                return sdf.result;
            }
        }

        draw_text: {
            text_style: <THEME_FONT_REGULAR> { font_size: 14.0 }
            color: (THEME_COLOR_TEXT_PRIMARY)
            instance selected: 0.0
            instance is_today: 0.0
            uniform accent_color: (THEME_COLOR_ACCENT)

            fn get_color(self) -> vec4 {
                return mix(self.color, self.accent_color, self.is_today * (1.0 - self.selected));
            }
        }
    }

    DayHeader = <View> {
        width: 36.0, height: 28.0
        align: {x: 0.5, y: 0.5}
        label = <Text> {
            width: Fit, height: Fit
            draw_text: { text_style: <THEME_FONT_REGULAR> { font_size: 12.0 }, color: (THEME_COLOR_TEXT_SECONDARY) }
        }
    }

    // Calendar row template (7 cells per row)
    CalendarRow = <View> {
        width: Fill, height: Fit, flow: Right, spacing: 2.0
    }

    pub DatePicker = {{DatePicker}} {
        width: Fill, height: Fill
        flow: Overlay
        week_start: Monday

        modal = <Modal> {
            content = <DialogStyle> {
                width: Fit
                spacing: 12.0

                title = <Text> {
                    width: Fit, height: Fit
                    draw_text: { text_style: <THEME_FONT_BOLD> { font_size: 14.0 }, color: (THEME_COLOR_TEXT_PRIMARY) }
                    text: "Select Date"
                }

                year_picker = <WheelH> {
                    width: 280.0, height: 40.0
                    step_size: 80.0
                    range_min: 2001, range_max: 2051
                    initial_value: 2026
                    is_infinite: false
                }

                month_picker = <WheelH> {
                    width: 280.0, height: 40.0
                    step_size: 80.0
                    is_infinite: true
                }

                calendar_wrap = <View> {
                    width: 280.0, height: Fit
                    flow: Down, spacing: 2.0

                    header_row = <View> {
                        width: Fill, height: Fit, flow: Right, spacing: 2.0
                        d0 = <DayHeader> {} d1 = <DayHeader> {} d2 = <DayHeader> {}
                        d3 = <DayHeader> {} d4 = <DayHeader> {} d5 = <DayHeader> {} d6 = <DayHeader> {}
                    }

                    row0 = <CalendarRow> { c0_0 = <DayCell> {} c0_1 = <DayCell> {} c0_2 = <DayCell> {} c0_3 = <DayCell> {} c0_4 = <DayCell> {} c0_5 = <DayCell> {} c0_6 = <DayCell> {} }
                    row1 = <CalendarRow> { c1_0 = <DayCell> {} c1_1 = <DayCell> {} c1_2 = <DayCell> {} c1_3 = <DayCell> {} c1_4 = <DayCell> {} c1_5 = <DayCell> {} c1_6 = <DayCell> {} }
                    row2 = <CalendarRow> { c2_0 = <DayCell> {} c2_1 = <DayCell> {} c2_2 = <DayCell> {} c2_3 = <DayCell> {} c2_4 = <DayCell> {} c2_5 = <DayCell> {} c2_6 = <DayCell> {} }
                    row3 = <CalendarRow> { c3_0 = <DayCell> {} c3_1 = <DayCell> {} c3_2 = <DayCell> {} c3_3 = <DayCell> {} c3_4 = <DayCell> {} c3_5 = <DayCell> {} c3_6 = <DayCell> {} }
                    row4 = <CalendarRow> { c4_0 = <DayCell> {} c4_1 = <DayCell> {} c4_2 = <DayCell> {} c4_3 = <DayCell> {} c4_4 = <DayCell> {} c4_5 = <DayCell> {} c4_6 = <DayCell> {} }
                    row5 = <CalendarRow> { c5_0 = <DayCell> {} c5_1 = <DayCell> {} c5_2 = <DayCell> {} c5_3 = <DayCell> {} c5_4 = <DayCell> {} c5_5 = <DayCell> {} c5_6 = <DayCell> {} }
                }

                buttons_wrap = <View> {
                    width: Fill, height: Fit, flow: Right, spacing: 10.0, margin: {top: 5.0}
                    today_button = <Btn> { text: "Today" }
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

#[derive(Clone, Debug, DefaultNone)]
pub enum DayCellAction {
    None,
    Clicked(u32),
}

// --- WeekStart Live traits ---

impl LiveHook for WeekStart {}

impl LiveRead for WeekStart {
    fn live_read_to(&self, id: LiveId, out: &mut Vec<LiveNode>) {
        let atom = match self { WeekStart::Monday => live_id!(Monday), WeekStart::Sunday => live_id!(Sunday) };
        out.push(LiveNode::from_id_value(id, LiveValue::Id(atom)));
    }
}

impl LiveApply for WeekStart {
    fn apply(&mut self, _cx: &mut Cx, _apply: &mut Apply, index: usize, nodes: &[LiveNode]) -> usize {
        if let LiveValue::Id(id) = &nodes[index].value {
            *self = if *id == live_id!(Sunday) { WeekStart::Sunday } else { WeekStart::Monday };
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

// --- DayCell ---

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
            Hit::FingerHoverIn(_) => { self.is_hovered = true; self.draw_bg.redraw(cx); }
            Hit::FingerHoverOut(_) => { self.is_hovered = false; self.draw_bg.redraw(cx); }
            Hit::FingerUp(fe) if fe.is_over && self.day > 0 => {
                cx.widget_action(self.widget_uid(), &scope.path, DayCellAction::Clicked(self.day));
            }
            _ => {}
        }
    }

    fn draw_walk(&mut self, cx: &mut Cx2d, _scope: &mut Scope, walk: Walk) -> DrawStep {
        self.draw_bg.apply_over(cx, live!{
            selected: (if self.is_selected && self.is_visible { 1.0 } else { 0.0 })
            hover: (if self.is_hovered && self.is_visible { 1.0 } else { 0.0 })
        });

        self.draw_bg.begin(cx, walk, self.layout);
        if self.is_visible {
            self.draw_text.apply_over(cx, live!{
                selected: (if self.is_selected { 1.0 } else { 0.0 })
                is_today: (if self.is_today { 1.0 } else { 0.0 })
            });
            self.draw_text.draw_walk(cx, Walk::fit(), Align::default(), &format!("{}", self.day));
        }
        self.draw_bg.end(cx);
        DrawStep::done()
    }
}

// --- DatePicker ---

#[derive(Live, LiveHook, Widget)]
pub struct DatePicker {
    #[deref] view: View,
    #[live] week_start: WeekStart,

    #[rust] displayed_year: i32,
    #[rust] displayed_month: u32,
    #[rust] selected_year: i32,
    #[rust] selected_month: u32,
    #[rust] selected_day: u32,

    #[rust] today: SimpleDate,
    #[rust] initialized: bool,

    // Cached UIDs to avoid borrow conflicts
    #[rust] year_picker_uid: Option<WidgetUid>,
    #[rust] month_picker_uid: Option<WidgetUid>,
    #[rust] today_button_uid: Option<WidgetUid>,

    // Swipe state
    #[rust] swipe_start_x: f64,
    #[rust] last_swipe_x: f64,
    #[rust] is_swiping: bool,
    #[rust] swipe_was_significant: bool,
}

impl DatePicker {
    fn modal_ref(&self) -> WidgetRef { self.view.widget(ids!(modal)) }
    fn content_ref(&self) -> WidgetRef { self.modal_ref().widget(ids!(content)) }

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

        self.displayed_year = self.selected_year;
        self.displayed_month = self.selected_month;

        // Cache widget UIDs
        let content = self.content_ref();
        self.year_picker_uid = content.widget(ids!(year_picker)).borrow::<Wheel>().map(|w| w.widget_uid());
        self.month_picker_uid = content.widget(ids!(month_picker)).borrow::<Wheel>().map(|w| w.widget_uid());
        self.today_button_uid = content.widget(ids!(buttons_wrap)).widget(ids!(today_button)).borrow::<Btn>().map(|b| b.widget_uid());

        // Setup month labels
        if let Some(mut picker) = content.widget(ids!(month_picker)).borrow_mut::<Wheel>() {
            picker.set_labels(MONTH_NAMES_SHORT.iter().map(|s| s.to_string()).collect());
        }

        self.sync_pickers(cx);
        self.update_calendar(cx);

        if let Some(mut modal) = self.modal_ref().borrow_mut::<Modal>() {
            modal.open(cx);
        }
    }

    pub fn close(&mut self, cx: &mut Cx) {
        if let Some(mut modal) = self.modal_ref().borrow_mut::<Modal>() {
            modal.close(cx);
        }
    }

    /// Sync wheel pickers to displayed_year/month
    fn sync_pickers(&mut self, cx: &mut Cx) {
        let content = self.content_ref();
        if let Some(mut p) = content.widget(ids!(year_picker)).borrow_mut::<Wheel>() {
            p.set_value(cx, self.displayed_year);
        }
        if let Some(mut p) = content.widget(ids!(month_picker)).borrow_mut::<Wheel>() {
            p.set_value(cx, (self.displayed_month - 1) as i32);
        }
    }

    /// Update calendar grid for displayed month/year
    fn update_calendar(&mut self, cx: &mut Cx) {
        let content = self.content_ref();
        let calendar_wrap = content.widget(ids!(calendar_wrap));

        // Update day headers
        let day_names = calendar::day_names_short(self.week_start);
        let header = calendar_wrap.widget(ids!(header_row));
        for (i, name) in day_names.iter().enumerate() {
            let id = [ids!(d0), ids!(d1), ids!(d2), ids!(d3), ids!(d4), ids!(d5), ids!(d6)][i];
            if let Some(mut t) = header.widget(id).widget(ids!(label)).borrow_mut::<crate::widgets::text::Text>() {
                t.set_text(cx, name);
            }
        }

        // Build day map
        let grid = calendar::calendar_grid(self.displayed_year, self.displayed_month, self.week_start);
        let mut day_map = [[0u32; 7]; 6];
        for &(day, row, col) in &grid {
            day_map[row as usize][col as usize] = day;
        }

        let is_sel_month = self.displayed_year == self.selected_year && self.displayed_month == self.selected_month;
        let is_today_month = self.displayed_year == self.today.year && self.displayed_month == self.today.month;

        // Update cells
        let row_ids = [ids!(row0), ids!(row1), ids!(row2), ids!(row3), ids!(row4), ids!(row5)];
        for row in 0..6 {
            let row_widget = calendar_wrap.widget(row_ids[row]);
            for col in 0..7 {
                let cell_id = Self::cell_id(row, col);
                if let Some(mut cell) = row_widget.widget(cell_id).borrow_mut::<DayCell>() {
                    let day = day_map[row][col];
                    cell.day = day;
                    cell.is_visible = day > 0;
                    cell.is_selected = is_sel_month && day == self.selected_day && day > 0;
                    cell.is_today = is_today_month && day == self.today.day && day > 0;
                    cell.is_hovered = false;
                }
            }
        }

        self.view.redraw(cx);
    }

    fn cell_id(row: usize, col: usize) -> &'static [LiveId] {
        const IDS: [[&[LiveId]; 7]; 6] = [
            [ids!(c0_0), ids!(c0_1), ids!(c0_2), ids!(c0_3), ids!(c0_4), ids!(c0_5), ids!(c0_6)],
            [ids!(c1_0), ids!(c1_1), ids!(c1_2), ids!(c1_3), ids!(c1_4), ids!(c1_5), ids!(c1_6)],
            [ids!(c2_0), ids!(c2_1), ids!(c2_2), ids!(c2_3), ids!(c2_4), ids!(c2_5), ids!(c2_6)],
            [ids!(c3_0), ids!(c3_1), ids!(c3_2), ids!(c3_3), ids!(c3_4), ids!(c3_5), ids!(c3_6)],
            [ids!(c4_0), ids!(c4_1), ids!(c4_2), ids!(c4_3), ids!(c4_4), ids!(c4_5), ids!(c4_6)],
            [ids!(c5_0), ids!(c5_1), ids!(c5_2), ids!(c5_3), ids!(c5_4), ids!(c5_5), ids!(c5_6)],
        ];
        IDS[row][col]
    }

    /// Handle calendar area swipe → sync to month wheel
    fn handle_swipe(&mut self, cx: &mut Cx, event: &Event) {
        let calendar_rect = self.content_ref().widget(ids!(calendar_wrap)).area().rect(cx);
        let threshold = 25.0;

        let (start, mv, end) = match event {
            Event::MouseDown(e) if calendar_rect.contains(e.abs) => (Some(e.abs.x), None, false),
            Event::MouseMove(e) if self.is_swiping => (None, Some(e.abs.x), false),
            Event::MouseUp(_) if self.is_swiping => (None, None, true),
            Event::TouchUpdate(tu) => {
                let mut result = (None, None, false);
                for t in &tu.touches {
                    match t.state {
                        makepad_widgets::event::TouchState::Start if calendar_rect.contains(t.abs) => result.0 = Some(t.abs.x),
                        makepad_widgets::event::TouchState::Move if self.is_swiping => result.1 = Some(t.abs.x),
                        makepad_widgets::event::TouchState::Stop if self.is_swiping => result.2 = true,
                        _ => {}
                    }
                }
                result
            }
            _ => (None, None, false),
        };

        if let Some(x) = start {
            self.is_swiping = true;
            self.swipe_start_x = x;
            self.last_swipe_x = x;
            self.swipe_was_significant = false;
        }

        if let Some(x) = mv {
            let delta = x - self.last_swipe_x;
            self.last_swipe_x = x;

            // Apply to wheel and check month change (must drop borrow before update_calendar)
            let visual_month = if let Some(mut p) = self.content_ref().widget(ids!(month_picker)).borrow_mut::<Wheel>() {
                p.apply_scroll_delta(cx, delta);
                Some((p.get_visual_value() + 1) as u32)
            } else { None };

            if let Some(m) = visual_month {
                if m != self.displayed_month {
                    self.displayed_month = m;
                    self.update_calendar(cx);
                }
            }

            if (x - self.swipe_start_x).abs() >= threshold {
                self.swipe_was_significant = true;
            }
        }

        if end {
            if let Some(mut p) = self.content_ref().widget(ids!(month_picker)).borrow_mut::<Wheel>() {
                p.trigger_snap(cx);
            }
            self.is_swiping = false;
        }
    }

    pub fn set_date(&mut self, cx: &mut Cx, year: i32, month: u32, day: u32) {
        self.selected_year = year;
        self.selected_month = month.clamp(1, 12);
        self.selected_day = day.clamp(1, calendar::days_in_month(year, self.selected_month));
        self.displayed_year = self.selected_year;
        self.displayed_month = self.selected_month;
        if self.is_open() {
            self.sync_pickers(cx);
            self.update_calendar(cx);
        }
    }
}

impl Widget for DatePicker {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        let uid = self.widget_uid();
        let modal_uid = self.modal_ref().borrow::<Modal>().map(|m| m.widget_uid());

        self.handle_swipe(cx, event);
        self.view.handle_event(cx, event, scope);

        if let Event::Actions(actions) = event {
            // Modal actions
            if let Some(modal_uid) = modal_uid {
                if let Some(action) = actions.find_widget_action(modal_uid) {
                    match action.cast() {
                        ModalAction::Accepted => {
                            let (y, m, d) = (self.selected_year, self.selected_month, self.selected_day);
                            cx.widget_action(uid, &scope.path, DatePickerAction::Accepted { year: y, month: m, day: d });
                        }
                        ModalAction::Dismissed => {
                            cx.widget_action(uid, &scope.path, DatePickerAction::Dismissed);
                        }
                        _ => {}
                    }
                }
            }

            for action in actions {
                let Some(wa) = action.as_widget_action() else { continue };

                // Wheel changes
                if let WheelAction::Changed(val) = wa.cast() {
                    if Some(wa.widget_uid) == self.year_picker_uid {
                        self.displayed_year = val;
                        self.update_calendar(cx);
                    } else if Some(wa.widget_uid) == self.month_picker_uid {
                        self.displayed_month = (val + 1) as u32;
                        self.update_calendar(cx);
                    }
                }

                // Day click (skip if swipe was significant)
                if let DayCellAction::Clicked(day) = wa.cast() {
                    if !self.swipe_was_significant {
                        self.selected_year = self.displayed_year;
                        self.selected_month = self.displayed_month;
                        self.selected_day = day;
                        self.update_calendar(cx);
                    }
                    self.swipe_was_significant = false;
                }

                // Today button
                if let ButtonAction::Clicked(_) = wa.cast() {
                    if Some(wa.widget_uid) == self.today_button_uid {
                        self.today = calendar::today();
                        self.displayed_year = self.today.year;
                        self.displayed_month = self.today.month;
                        self.selected_year = self.today.year;
                        self.selected_month = self.today.month;
                        self.selected_day = self.today.day;
                        self.sync_pickers(cx);
                        self.update_calendar(cx);
                    }
                }
            }
        }
    }

    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        self.view.draw_walk(cx, scope, walk)
    }
}
