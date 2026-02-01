//! Unified wheel picker widget (horizontal and vertical).
//!
//! # Architecture
//! - `scroll_pos`: Current scroll position in pixels (continuous)
//! - `current_value`: Discrete value derived from scroll_pos
//! - `is_infinite`: Whether values wrap around (e.g., months) or clamp (e.g., years)
//!
//! # Interaction Flow
//! 1. Drag/scroll → updates scroll_pos → emits Changed(value)
//! 2. Release → triggers snap animation to nearest grid position
//! 3. External control via apply_scroll_delta() + trigger_snap()

use makepad_widgets::*;

#[derive(Clone, Debug, DefaultNone)]
pub enum WheelAction {
    None,
    Changed(i32),
}

// Keep old action types for backwards compatibility
pub type WheelHAction = WheelAction;
pub type WheelVAction = WheelAction;

/// Wheel orientation
#[derive(Clone, Copy, Debug, PartialEq, Default)]
pub enum Orientation {
    #[default]
    Horizontal,
    Vertical,
}

impl LiveHook for Orientation {}

impl LiveRead for Orientation {
    fn live_read_to(&self, id: LiveId, out: &mut Vec<LiveNode>) {
        let atom = match self {
            Orientation::Horizontal => live_id!(Horizontal),
            Orientation::Vertical => live_id!(Vertical),
        };
        out.push(LiveNode::from_id_value(id, LiveValue::Id(atom)));
    }
}

impl LiveApply for Orientation {
    fn apply(&mut self, _cx: &mut Cx, _apply: &mut Apply, index: usize, nodes: &[LiveNode]) -> usize {
        if let LiveValue::Id(id) = &nodes[index].value {
            *self = if *id == live_id!(Vertical) { Orientation::Vertical } else { Orientation::Horizontal };
        }
        index + 1
    }
}

impl LiveNew for Orientation {
    fn live_type_info(_cx: &mut Cx) -> LiveTypeInfo {
        LiveTypeInfo {
            module_id: LiveModuleId::from_str(&module_path!()).unwrap(),
            live_type: std::any::TypeId::of::<Self>(),
            live_ignore: true,
            fields: Vec::new(),
            type_name: live_id!(Orientation),
        }
    }
    fn new(_cx: &mut Cx) -> Self { Orientation::Horizontal }
}

live_design! {
    use link::theme::*;
    use link::shaders::*;
    use crate::styling::*;

    // Horizontal wheel
    pub WheelH = {{Wheel}} {
        width: 280.0, height: 40.0
        orientation: Horizontal
        step_size: 60.0
        range_min: 0
        range_max: 11
        is_infinite: true

        draw_text: {
            text_style: <THEME_FONT_BOLD> { font_size: 16.0 }
            color: #FFF
        }

        draw_bg: {
            instance color: #0000
            instance border_color: #FFF3
            instance border_width: 1.0
            instance border_radius: 2.0

            fn pixel(self) -> vec4 {
                let sdf = Sdf2d::viewport(self.pos * self.rect_size)
                sdf.box(
                    self.border_width,
                    self.border_width,
                    self.rect_size.x - self.border_width * 2.0,
                    self.rect_size.y - self.border_width * 2.0,
                    self.border_radius
                )
                sdf.fill_keep(self.color)
                return sdf.stroke(self.border_color, self.border_width)
            }
        }

        draw_selection: {
            instance color: (THEME_COLOR_ACCENT)
            instance border_radius: 2.0

            fn pixel(self) -> vec4 {
                let sdf = Sdf2d::viewport(self.pos * self.rect_size)
                sdf.box(0.0, 0.0, self.rect_size.x, self.rect_size.y, self.border_radius)
                sdf.fill_keep(vec4(self.color.xyz, 1.0));
                let stroke_color = mix(#fff5, #0005, self.pos.x);
                return sdf.stroke(stroke_color, 1.0);
            }
        }
    }

    // Vertical wheel
    pub WheelV = {{Wheel}} {
        width: 100.0, height: 160.0
        orientation: Vertical
        step_size: 40.0
        range_min: 0
        range_max: 23
        is_infinite: true

        draw_text: {
            text_style: <THEME_FONT_BOLD> { font_size: 20.0 }
            color: #FFF
        }

        draw_bg: {
            instance color: #0000
            instance border_color: #FFF3
            instance border_width: 1.0
            instance border_radius: 2.0

            fn pixel(self) -> vec4 {
                let sdf = Sdf2d::viewport(self.pos * self.rect_size)
                sdf.box(
                    self.border_width,
                    self.border_width,
                    self.rect_size.x - self.border_width * 2.0,
                    self.rect_size.y - self.border_width * 2.0,
                    self.border_radius
                )
                sdf.fill_keep(self.color)
                return sdf.stroke(self.border_color, self.border_width)
            }
        }

        draw_selection: {
            instance color: (THEME_COLOR_ACCENT)
            instance border_radius: 2.0

            fn pixel(self) -> vec4 {
                let sdf = Sdf2d::viewport(self.pos * self.rect_size)
                sdf.box(0.0, 0.0, self.rect_size.x, self.rect_size.y, self.border_radius)
                sdf.fill_keep(vec4(self.color.xyz, 1.0));
                let stroke_color = mix(#fff5, #0005, self.pos.y);
                return sdf.stroke(stroke_color, 1.0);
            }
        }
    }
}

/// Scroll speed factor (lower = slower, more controlled feel)
const SCROLL_SPEED: f64 = 1.0 / 1.5;

#[derive(Live, LiveHook, Widget)]
pub struct Wheel {
    #[redraw] #[live] draw_text: DrawText,
    #[live] draw_bg: DrawQuad,
    #[live] draw_selection: DrawQuad,

    #[live] orientation: Orientation,
    #[live] step_size: f64,
    #[live] range_min: i32,
    #[live] range_max: i32,
    #[live(true)] is_infinite: bool,
    #[live(0)] initial_value: i32,

    #[rust] labels: Vec<String>,
    #[layout] layout: Layout,
    #[walk] walk: Walk,

    #[rust] scroll_pos: f64,
    #[rust] current_value: i32,
    #[rust] is_dragging: bool,
    #[rust] is_outside_bounds: bool,
    #[rust] last_abs: f64,
    #[rust] drag_start: f64,

    #[rust] next_frame: NextFrame,
    #[rust] scroll_target: Option<f64>,
    #[rust] scroll_cooldown: Option<i32>,
    #[rust] initialized: bool,
}

// Type aliases for backwards compatibility
pub type WheelH = Wheel;
pub type WheelV = Wheel;

impl Wheel {
    /// Get primary axis value from position
    #[inline]
    fn axis(&self, pos: DVec2) -> f64 {
        match self.orientation {
            Orientation::Horizontal => pos.x,
            Orientation::Vertical => pos.y,
        }
    }

    /// Get primary axis value from scroll event
    #[inline]
    fn scroll_axis(&self, scroll: DVec2) -> f64 {
        match self.orientation {
            Orientation::Horizontal => {
                if scroll.x.abs() > scroll.y.abs() { scroll.x } else { scroll.y }
            }
            Orientation::Vertical => scroll.y,
        }
    }

    /// Initialize from initial_value if not yet set
    fn ensure_initialized(&mut self) {
        if self.initialized { return; }
        self.initialized = true;
        let val = self.initial_value.clamp(self.range_min, self.range_max);
        self.current_value = val;
        self.scroll_pos = (val - self.range_min) as f64 * self.step_size;
    }

    /// Apply scroll delta and update display
    fn apply_delta(&mut self, cx: &mut Cx, delta: f64, path: &HeapLiveIdPath) {
        self.scroll_pos -= delta * SCROLL_SPEED;
        if !self.is_infinite { self.clamp_scroll(); }
        self.redraw(cx);
        self.update_value(cx, path);
    }

    /// Start drag interaction
    fn start_drag(&mut self, cx: &mut Cx, abs: f64) {
        cx.sweep_lock(self.draw_bg.area());
        self.is_dragging = true;
        self.is_outside_bounds = false;
        self.last_abs = abs;
        self.drag_start = abs;
        if let Some(target) = self.scroll_target.take() {
            self.scroll_pos = target;
        }
        self.scroll_cooldown = None;
    }

    /// Finish drag - snap to grid or handle tap
    fn finish_drag(&mut self, cx: &mut Cx) {
        cx.sweep_unlock(self.draw_bg.area());
        self.is_dragging = false;
        self.is_outside_bounds = false;

        let snapped_pos = self.get_snapped_pos();

        // Tap detection
        if (self.last_abs - self.drag_start).abs() < 10.0 {
            let rect = self.draw_bg.area().rect(cx);
            let center = self.axis(rect.pos) + self.axis(rect.size) * 0.5;
            let steps_offset = ((self.last_abs - center) / self.step_size).round();
            if steps_offset != 0.0 {
                self.scroll_pos = snapped_pos;
                self.scroll_target = Some(snapped_pos + steps_offset * self.step_size);
                self.next_frame = cx.new_next_frame();
                self.redraw(cx);
                return;
            }
        }

        if (self.scroll_pos - snapped_pos).abs() > 0.5 {
            self.scroll_target = Some(snapped_pos);
            self.next_frame = cx.new_next_frame();
        } else {
            self.scroll_pos = snapped_pos;
        }
        self.redraw(cx);
    }

    fn get_snapped_pos(&self) -> f64 {
        let mut idx = (self.scroll_pos / self.step_size).round();
        if !self.is_infinite {
            idx = idx.clamp(0.0, (self.range_max - self.range_min) as f64);
        }
        idx * self.step_size
    }

    fn clamp_scroll(&mut self) {
        if !self.is_infinite {
            let max_pos = (self.range_max - self.range_min) as f64 * self.step_size;
            self.scroll_pos = self.scroll_pos.clamp(0.0, max_pos);
        }
    }

    fn snap_to_grid(&mut self, cx: &mut Cx) {
        self.scroll_target = Some(self.get_snapped_pos());
        self.next_frame = cx.new_next_frame();
    }

    fn update_value(&mut self, cx: &mut Cx, path: &HeapLiveIdPath) {
        let raw_idx = (self.scroll_pos / self.step_size).round() as i32;
        let range_len = self.range_max - self.range_min + 1;
        self.current_value = if self.is_infinite {
            raw_idx.rem_euclid(range_len) + self.range_min
        } else {
            raw_idx.clamp(0, range_len - 1) + self.range_min
        };
        cx.widget_action(self.widget_uid(), path, WheelAction::Changed(self.current_value));
    }

    fn redraw(&mut self, cx: &mut Cx) {
        self.draw_bg.redraw(cx);
        self.draw_selection.redraw(cx);
    }

    fn get_label(&self, val: i32) -> String {
        if self.labels.is_empty() {
            match self.orientation {
                Orientation::Horizontal => format!("{}", val),
                Orientation::Vertical => format!("{:02}", val),
            }
        } else {
            let idx = (val - self.range_min) as usize;
            self.labels.get(idx).cloned().unwrap_or_else(|| format!("{}", val))
        }
    }

    // --- Public API ---

    pub fn get_value(&self) -> i32 { self.current_value }

    pub fn set_range(&mut self, min: i32, max: i32) {
        self.range_min = min;
        self.range_max = max;
    }

    pub fn set_labels(&mut self, labels: Vec<String>) {
        if !labels.is_empty() {
            self.range_min = 0;
            self.range_max = (labels.len() - 1) as i32;
        }
        self.labels = labels;
    }

    pub fn set_value(&mut self, cx: &mut Cx, val: i32) {
        self.current_value = val.clamp(self.range_min, self.range_max);
        self.scroll_pos = (self.current_value - self.range_min) as f64 * self.step_size;
        self.initialized = true;
        self.redraw(cx);
    }

    /// Apply external scroll delta (e.g., from calendar swipe)
    pub fn apply_scroll_delta(&mut self, cx: &mut Cx, delta: f64) {
        self.scroll_pos -= delta * SCROLL_SPEED;
        if !self.is_infinite { self.clamp_scroll(); }
        self.redraw(cx);
    }

    /// Trigger async snap animation
    pub fn trigger_snap(&mut self, cx: &mut Cx) {
        self.scroll_cooldown = Some(0);
        self.next_frame = cx.new_next_frame();
    }

    /// Get current visual value during scroll
    pub fn get_visual_value(&self) -> i32 {
        let raw_idx = (self.scroll_pos / self.step_size).round() as i32;
        let range_len = self.range_max - self.range_min + 1;
        if self.is_infinite {
            raw_idx.rem_euclid(range_len) + self.range_min
        } else {
            raw_idx.clamp(0, range_len - 1) + self.range_min
        }
    }
}

impl Widget for Wheel {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        self.ensure_initialized();

        // Snap animation
        if self.next_frame.is_event(event).is_some() {
            if let Some(target) = self.scroll_target {
                let delta = target - self.scroll_pos;
                if delta.abs() < 1.0 {
                    self.scroll_pos = target;
                    self.scroll_target = None;
                } else {
                    self.scroll_pos += delta * 0.2;
                    self.next_frame = cx.new_next_frame();
                }
                if !self.is_infinite { self.clamp_scroll(); }
                self.redraw(cx);
                self.update_value(cx, &scope.path);
            } else if let Some(cooldown) = self.scroll_cooldown {
                if cooldown > 0 {
                    self.scroll_cooldown = Some(cooldown - 1);
                    self.next_frame = cx.new_next_frame();
                } else {
                    self.scroll_cooldown = None;
                    self.snap_to_grid(cx);
                }
            }
        }

        // Mouse wheel scroll
        if let Event::Scroll(e) = event {
            if self.draw_bg.area().rect(cx).contains(e.abs) {
                e.handled_x.set(true);
                e.handled_y.set(true);
                self.scroll_target = None;
                let delta = self.scroll_axis(e.scroll);
                self.scroll_pos -= delta * SCROLL_SPEED;
                if !self.is_infinite { self.clamp_scroll(); }
                self.redraw(cx);
                self.update_value(cx, &scope.path);
                self.scroll_cooldown = Some(10);
                self.next_frame = cx.new_next_frame();
            }
        }

        // Outside bounds tracking
        if self.is_outside_bounds {
            match event {
                Event::MouseMove(e) => {
                    let abs = self.axis(e.abs);
                    self.apply_delta(cx, abs - self.last_abs, &scope.path);
                    self.last_abs = abs;
                }
                Event::MouseUp(_) => self.finish_drag(cx),
                Event::TouchUpdate(tu) => {
                    for touch in &tu.touches {
                        touch.handled.set(self.draw_bg.area());
                        let abs = self.axis(touch.abs);
                        match touch.state {
                            makepad_widgets::event::TouchState::Move => {
                                self.apply_delta(cx, abs - self.last_abs, &scope.path);
                                self.last_abs = abs;
                            }
                            makepad_widgets::event::TouchState::Stop => self.finish_drag(cx),
                            _ => ()
                        }
                    }
                }
                _ => ()
            }
            return;
        }

        // Touch handling
        if let Event::TouchUpdate(tu) = event {
            for touch in &tu.touches {
                let in_bounds = self.draw_bg.area().rect(cx).contains(touch.abs);
                let abs = self.axis(touch.abs);
                match touch.state {
                    makepad_widgets::event::TouchState::Start if in_bounds => {
                        touch.handled.set(self.draw_bg.area());
                        self.start_drag(cx, abs);
                    }
                    makepad_widgets::event::TouchState::Move if self.is_dragging => {
                        touch.handled.set(self.draw_bg.area());
                        self.apply_delta(cx, abs - self.last_abs, &scope.path);
                        self.last_abs = abs;
                    }
                    makepad_widgets::event::TouchState::Stop if self.is_dragging => {
                        touch.handled.set(self.draw_bg.area());
                        self.last_abs = abs;
                        self.finish_drag(cx);
                    }
                    _ => ()
                }
            }
            return;
        }

        // Mouse handling
        if let Event::MouseDown(e) = event {
            if self.draw_bg.area().rect(cx).contains(e.abs) {
                cx.sweep_lock(self.draw_bg.area());
            }
        }

        match event.hits_with_sweep_area(cx, self.draw_bg.area(), self.draw_bg.area()) {
            Hit::FingerDown(fe) => self.start_drag(cx, self.axis(fe.abs)),
            Hit::FingerMove(fe) if self.is_dragging => {
                let abs = self.axis(fe.abs);
                self.apply_delta(cx, abs - self.last_abs, &scope.path);
                self.last_abs = abs;
            }
            Hit::FingerUp(fe) if self.is_dragging => {
                if fe.is_sweep {
                    self.is_outside_bounds = true;
                    self.last_abs = self.axis(fe.abs);
                } else {
                    self.finish_drag(cx);
                }
            }
            _ => ()
        }
    }

    fn draw_walk(&mut self, cx: &mut Cx2d, _scope: &mut Scope, walk: Walk) -> DrawStep {
        self.ensure_initialized();
        self.draw_bg.begin(cx, walk, self.layout);

        let rect = cx.turtle().rect();

        // Auto-detect orientation from dimensions
        let orientation = if rect.size.y > rect.size.x {
            Orientation::Vertical
        } else {
            Orientation::Horizontal
        };
        self.orientation = orientation;

        let (center, size) = match orientation {
            Orientation::Horizontal => (rect.size.x * 0.5, rect.size.x),
            Orientation::Vertical => (rect.size.y * 0.5, rect.size.y),
        };

        // Selection indicator
        let selection_rect = match self.orientation {
            Orientation::Horizontal => Rect {
                pos: dvec2(rect.pos.x + center - self.step_size * 0.5, rect.pos.y),
                size: dvec2(self.step_size, rect.size.y),
            },
            Orientation::Vertical => Rect {
                pos: dvec2(rect.pos.x, rect.pos.y + center - self.step_size * 0.5),
                size: dvec2(rect.size.x, self.step_size),
            },
        };
        self.draw_selection.draw_abs(cx, selection_rect);

        // Visible items
        let center_idx = (self.scroll_pos / self.step_size).round() as i32;
        let window = (center / self.step_size).ceil() as i32 + 1;
        let range_len = self.range_max - self.range_min + 1;
        let max_dist = match self.orientation {
            Orientation::Horizontal => center,
            Orientation::Vertical => size * 0.6,
        };

        for i in (center_idx - window)..=(center_idx + window) {
            let offset = i as f64 * self.step_size - self.scroll_pos;
            let dist = offset.abs();

            if dist >= max_dist { continue; }

            let val = if self.is_infinite {
                i.rem_euclid(range_len) + self.range_min
            } else {
                if i < 0 || i >= range_len { continue; }
                i + self.range_min
            };

            let opacity = 1.0 - (dist / max_dist).powf(1.5);
            self.draw_text.color.w = opacity as f32;

            // Vertical has scale effect
            if self.orientation == Orientation::Vertical {
                let scale = 1.0 - (dist / max_dist) * 0.3;
                self.draw_text.text_style.font_size = (20.0 * scale) as f32;
            }

            let text = self.get_label(val);
            let layout = self.draw_text.layout(cx, 0.0, 0.0, None, false, Align::default(), &text);
            let text_size = dvec2(
                layout.size_in_lpxs.width as f64 * self.draw_text.font_scale as f64,
                layout.size_in_lpxs.height as f64 * self.draw_text.font_scale as f64,
            );

            let pos = match self.orientation {
                Orientation::Horizontal => {
                    rect.pos + dvec2(center + offset, rect.size.y * 0.5) - text_size * 0.5
                }
                Orientation::Vertical => {
                    dvec2(
                        rect.pos.x + (rect.size.x - text_size.x) * 0.5,
                        rect.pos.y + center + offset - text_size.y * 0.5
                    )
                }
            };

            self.draw_text.draw_abs(cx, pos, &text);
        }

        self.draw_bg.end(cx);
        DrawStep::done()
    }
}
