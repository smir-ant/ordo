use makepad_widgets::*;

#[derive(Clone, Debug, DefaultNone)]
pub enum WheelHAction {
    None,
    Changed(i32),
}

live_design! {
    use link::theme::*;
    use link::shaders::*;
    use crate::styling::*;

    pub WheelH = {{WheelH}} {
        width: 280.0, height: 40.0

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
                sdf.fill_keep(vec4(self.color.xyz, 1));

                let stroke_left = #fff5;
                let stroke_right = #0005;
                let stroke_color = mix(stroke_left, stroke_right, self.pos.x);

                return sdf.stroke(stroke_color, 1.0);
            }
        }

        step_width: 60.0
        range_min: 0
        range_max: 11
        is_infinite: true
        initial_value: 0
    }
}

#[derive(Live, LiveHook, Widget)]
pub struct WheelH {
    #[redraw] #[live] draw_text: DrawText,
    #[live] draw_bg: DrawQuad,
    #[live] draw_selection: DrawQuad,

    #[live] step_width: f64,
    #[live] range_min: i32,
    #[live] range_max: i32,
    #[live(true)] is_infinite: bool,
    #[live(0)] initial_value: i32,

    /// Labels for string mode (e.g., month names). If empty, uses range numbers.
    #[rust] labels: Vec<String>,

    #[layout] layout: Layout,
    #[walk] walk: Walk,

    #[rust] scroll_pos: f64,
    #[rust] current_value: i32,
    #[rust] is_dragging: bool,
    #[rust] is_outside_bounds: bool,
    #[rust] last_abs_x: f64,
    #[rust] drag_start_x: f64,

    #[rust] next_frame: NextFrame,
    #[rust] scroll_target: Option<f64>,
    #[rust] scroll_cooldown: Option<i32>,
    #[rust] initialized: bool,
}

impl Widget for WheelH {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        // Initialize scroll position on first draw
        if !self.initialized {
            self.initialized = true;
            let val = self.initial_value.clamp(self.range_min, self.range_max);
            self.current_value = val;
            let idx = (val - self.range_min) as f64;
            self.scroll_pos = idx * self.step_width;
        }

        // Animation
        if let Some(_) = self.next_frame.is_event(event) {
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
                self.draw_bg.redraw(cx);
                self.draw_selection.redraw(cx);
                self.update_value(cx, &scope.path);
            } else if let Some(cooldown) = self.scroll_cooldown {
                if cooldown > 0 {
                    self.scroll_cooldown = Some(cooldown - 1);
                    self.next_frame = cx.new_next_frame();
                } else {
                    self.scroll_cooldown = None;
                    self.snap_to_grid(cx);
                    self.draw_bg.redraw(cx);
                    self.draw_selection.redraw(cx);
                }
            }
        }

        // Scroll (mouse wheel) - use X for horizontal, but also accept Y for convenience
        if let Event::Scroll(e) = event {
            if self.draw_bg.area().rect(cx).contains(e.abs) {
                e.handled_x.set(true);
                e.handled_y.set(true);
                self.scroll_target = None;
                // Use horizontal scroll if available, otherwise use vertical
                let scroll_delta = if e.scroll.x.abs() > e.scroll.y.abs() {
                    e.scroll.x
                } else {
                    e.scroll.y
                };
                self.scroll_pos -= scroll_delta / 1.5;
                if !self.is_infinite { self.clamp_scroll(); }
                self.draw_bg.redraw(cx);
                self.draw_selection.redraw(cx);
                self.update_value(cx, &scope.path);
                self.scroll_cooldown = Some(10);
                self.next_frame = cx.new_next_frame();
            }
        }

        // Track finger outside widget bounds via raw events
        if self.is_outside_bounds {
            match event {
                Event::MouseMove(e) => {
                    let delta = e.abs.x - self.last_abs_x;
                    self.scroll_pos -= delta / 1.5;
                    if !self.is_infinite { self.clamp_scroll(); }
                    self.last_abs_x = e.abs.x;
                    self.draw_bg.redraw(cx);
                    self.draw_selection.redraw(cx);
                    self.update_value(cx, &scope.path);
                }
                Event::MouseUp(_) => {
                    self.finish_drag(cx);
                }
                Event::TouchUpdate(tu) => {
                    for touch in &tu.touches {
                        touch.handled.set(self.draw_bg.area());
                        match touch.state {
                            makepad_widgets::event::TouchState::Move => {
                                let delta = touch.abs.x - self.last_abs_x;
                                self.scroll_pos -= delta / 1.5;
                                if !self.is_infinite { self.clamp_scroll(); }
                                self.last_abs_x = touch.abs.x;
                                self.draw_bg.redraw(cx);
                                self.draw_selection.redraw(cx);
                                self.update_value(cx, &scope.path);
                            }
                            makepad_widgets::event::TouchState::Stop => {
                                self.finish_drag(cx);
                            }
                            _ => ()
                        }
                    }
                }
                _ => ()
            }
            return;
        }

        // Handle touch events directly for reliable mobile response
        if let Event::TouchUpdate(tu) = event {
            for touch in &tu.touches {
                let dominated = self.draw_bg.area().rect(cx).contains(touch.abs);
                match touch.state {
                    makepad_widgets::event::TouchState::Start => {
                        if dominated {
                            cx.sweep_lock(self.draw_bg.area());
                            touch.handled.set(self.draw_bg.area());
                            self.is_dragging = true;
                            self.is_outside_bounds = false;
                            self.last_abs_x = touch.abs.x;
                            self.drag_start_x = touch.abs.x;
                            // Complete pending animation before starting new interaction
                            if let Some(target) = self.scroll_target.take() {
                                self.scroll_pos = target;
                            }
                            self.scroll_cooldown = None;
                        }
                    }
                    makepad_widgets::event::TouchState::Move => {
                        if self.is_dragging {
                            touch.handled.set(self.draw_bg.area());
                            let delta = touch.abs.x - self.last_abs_x;
                            self.scroll_pos -= delta / 1.5;
                            if !self.is_infinite { self.clamp_scroll(); }
                            self.last_abs_x = touch.abs.x;
                            self.draw_bg.redraw(cx);
                            self.draw_selection.redraw(cx);
                            self.update_value(cx, &scope.path);
                        }
                    }
                    makepad_widgets::event::TouchState::Stop => {
                        if self.is_dragging {
                            touch.handled.set(self.draw_bg.area());
                            self.last_abs_x = touch.abs.x;  // Update before finish for tap detection
                            self.finish_drag(cx);
                        }
                    }
                    _ => ()
                }
            }
            return;
        }

        // sweep_lock for mouse
        if let Event::MouseDown(e) = event {
            if self.draw_bg.area().rect(cx).contains(e.abs) {
                cx.sweep_lock(self.draw_bg.area());
            }
        }

        // Track mouse inside widget
        match event.hits_with_sweep_area(cx, self.draw_bg.area(), self.draw_bg.area()) {
            Hit::FingerDown(fe) => {
                self.is_dragging = true;
                self.is_outside_bounds = false;
                self.last_abs_x = fe.abs.x;
                self.drag_start_x = fe.abs.x;
                // Complete pending animation before starting new interaction
                if let Some(target) = self.scroll_target.take() {
                    self.scroll_pos = target;
                }
                self.scroll_cooldown = None;
            }
            Hit::FingerMove(fe) => {
                if self.is_dragging {
                    let delta = fe.abs.x - self.last_abs_x;
                    self.scroll_pos -= delta / 1.5;
                    if !self.is_infinite { self.clamp_scroll(); }
                    self.last_abs_x = fe.abs.x;
                    self.draw_bg.redraw(cx);
                    self.draw_selection.redraw(cx);
                    self.update_value(cx, &scope.path);
                }
            }
            Hit::FingerUp(fe) => {
                if self.is_dragging {
                    if fe.is_sweep {
                        self.is_outside_bounds = true;
                        self.last_abs_x = fe.abs.x;
                    } else {
                        self.finish_drag(cx);
                    }
                }
            }
            _ => ()
        }
    }

    fn draw_walk(&mut self, cx: &mut Cx2d, _scope: &mut Scope, walk: Walk) -> DrawStep {
        // Initialize on first draw if not yet done
        if !self.initialized {
            self.initialized = true;
            let val = self.initial_value.clamp(self.range_min, self.range_max);
            self.current_value = val;
            let idx = (val - self.range_min) as f64;
            self.scroll_pos = idx * self.step_width;
        }

        self.draw_bg.begin(cx, walk, self.layout);

        let rect = cx.turtle().rect();
        let center_x = rect.size.x * 0.5;
        let center_y = rect.size.y * 0.5;

        // Selection indicator (centered)
        let selection_rect = Rect {
            pos: dvec2(rect.pos.x + center_x - self.step_width * 0.5, rect.pos.y),
            size: dvec2(self.step_width, rect.size.y),
        };
        self.draw_selection.draw_abs(cx, selection_rect);

        let center_idx = (self.scroll_pos / self.step_width).round() as i32;
        // Calculate window dynamically based on widget width and step
        let window = ((rect.size.x * 0.5) / self.step_width).ceil() as i32 + 1;
        let range_len = self.range_max - self.range_min + 1;

        for i in (center_idx - window)..=(center_idx + window) {
            let offset = (i as f64) * self.step_width - self.scroll_pos;

            if offset < -rect.size.x || offset > rect.size.x { continue; }

            let dist = offset.abs();
            let max_dist = rect.size.x * 0.5;

            if dist < max_dist {
                let val = if self.is_infinite {
                    (i.rem_euclid(range_len)) + self.range_min
                } else {
                    if i < 0 || i >= range_len { continue; }
                    i + self.range_min
                };

                let opacity = 1.0 - (dist / max_dist).powf(1.5);

                self.draw_text.color.w = opacity as f32;
                self.draw_text.text_style.font_size = 16.0;

                // Get text: either from labels or format as number
                let text = self.get_label(val);

                let laidout = self.draw_text.layout(cx, 0.0, 0.0, None, false, Align::default(), &text);
                let width = laidout.size_in_lpxs.width as f64 * self.draw_text.font_scale as f64;
                let height = laidout.size_in_lpxs.height as f64 * self.draw_text.font_scale as f64;

                let pos = dvec2(
                    rect.pos.x + center_x + offset - width * 0.5,
                    rect.pos.y + center_y - height * 0.5
                );

                self.draw_text.draw_abs(cx, pos, &text);
            }
        }

        self.draw_bg.end(cx);
        DrawStep::done()
    }
}

impl WheelH {
    fn finish_drag(&mut self, cx: &mut Cx) {
        cx.sweep_unlock(self.draw_bg.area());
        self.is_dragging = false;
        self.is_outside_bounds = false;

        // Always snap scroll_pos to grid first
        let mut snapped_idx = (self.scroll_pos / self.step_width).round();
        if !self.is_infinite {
            let max_idx = (self.range_max - self.range_min) as f64;
            snapped_idx = snapped_idx.clamp(0.0, max_idx);
        }
        let snapped_pos = snapped_idx * self.step_width;

        let rect = self.draw_bg.area().rect(cx);
        // Tap detection - select item by position
        if (self.last_abs_x - self.drag_start_x).abs() < 10.0 {
            let center_x = rect.pos.x + rect.size.x * 0.5;
            let touch_offset = self.last_abs_x - center_x;
            let steps_offset = (touch_offset / self.step_width).round();
            if steps_offset != 0.0 {
                // Animate to tapped element
                self.scroll_pos = snapped_pos;
                let target_pos = snapped_pos + steps_offset * self.step_width;
                self.scroll_target = Some(target_pos);
                self.next_frame = cx.new_next_frame();
                self.draw_bg.redraw(cx);
                self.draw_selection.redraw(cx);
                return;
            }
        }

        // For swipe: animate to snapped position
        if (self.scroll_pos - snapped_pos).abs() > 0.5 {
            self.scroll_target = Some(snapped_pos);
            self.next_frame = cx.new_next_frame();
        } else {
            self.scroll_pos = snapped_pos;
        }
        self.draw_bg.redraw(cx);
        self.draw_selection.redraw(cx);
    }

    fn clamp_scroll(&mut self) {
        if !self.is_infinite {
            let max_idx = (self.range_max - self.range_min) as f64;
            let max_pos = max_idx * self.step_width;
            self.scroll_pos = self.scroll_pos.clamp(0.0, max_pos);
        }
    }

    fn snap_to_grid(&mut self, cx: &mut Cx) {
        let mut snapped_idx = (self.scroll_pos / self.step_width).round();
        if !self.is_infinite {
            let max_idx = (self.range_max - self.range_min) as f64;
            snapped_idx = snapped_idx.clamp(0.0, max_idx);
        }
        self.scroll_target = Some(snapped_idx * self.step_width);
        self.next_frame = cx.new_next_frame();
    }

    fn update_value(&mut self, cx: &mut Cx, path: &HeapLiveIdPath) {
        let raw_idx = (self.scroll_pos / self.step_width).round() as i32;
        let range_len = self.range_max - self.range_min + 1;

        self.current_value = if self.is_infinite {
            (raw_idx.rem_euclid(range_len)) + self.range_min
        } else {
            raw_idx.clamp(0, range_len - 1) + self.range_min
        };
        cx.widget_action(self.widget_uid(), path, WheelHAction::Changed(self.current_value));
    }

    fn get_label(&self, val: i32) -> String {
        if self.labels.is_empty() {
            format!("{}", val)
        } else {
            let idx = (val - self.range_min) as usize;
            if idx < self.labels.len() {
                self.labels[idx].clone()
            } else {
                format!("{}", val)
            }
        }
    }

    pub fn get_value(&self) -> i32 {
        self.current_value
    }

    pub fn set_range(&mut self, min: i32, max: i32) {
        self.range_min = min;
        self.range_max = max;
    }

    pub fn set_labels(&mut self, labels: Vec<String>) {
        self.labels = labels.clone();
        if !labels.is_empty() {
            self.range_min = 0;
            self.range_max = (labels.len() - 1) as i32;
        }
    }

    pub fn set_value(&mut self, cx: &mut Cx, val: i32) {
        self.current_value = val.clamp(self.range_min, self.range_max);
        let idx = (self.current_value - self.range_min) as f64;
        self.scroll_pos = idx * self.step_width;
        self.initialized = true; // Prevent draw_walk from resetting to initial_value
        self.draw_bg.redraw(cx);
        self.draw_selection.redraw(cx);
    }

    /// Apply external scroll delta (e.g., from calendar swipe)
    /// Does NOT emit Changed action - caller should call trigger_snap() when done
    pub fn apply_scroll_delta(&mut self, cx: &mut Cx, delta: f64) {
        self.scroll_pos -= delta / 1.5;
        if !self.is_infinite {
            self.clamp_scroll();
        }
        self.draw_bg.redraw(cx);
        self.draw_selection.redraw(cx);
    }

    /// Trigger async snap-to-grid after external scroll
    /// Uses NextFrame animation, then emits WheelHAction::Changed when done
    pub fn trigger_snap(&mut self, cx: &mut Cx) {
        // Set cooldown to 0 so snap happens on next frame
        self.scroll_cooldown = Some(0);
        self.next_frame = cx.new_next_frame();
    }

    /// Get the step width for delta calculations
    pub fn get_step_width(&self) -> f64 {
        self.step_width
    }

    /// Get current visual value based on scroll position (without emitting action)
    /// Useful for tracking value during external scroll
    pub fn get_visual_value(&self) -> i32 {
        let raw_idx = (self.scroll_pos / self.step_width).round() as i32;
        let range_len = self.range_max - self.range_min + 1;

        if self.is_infinite {
            (raw_idx.rem_euclid(range_len)) + self.range_min
        } else {
            raw_idx.clamp(0, range_len - 1) + self.range_min
        }
    }
}
