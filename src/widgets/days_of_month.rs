use makepad_widgets::*;

live_design! {
    use makepad_widgets::base::*;
    use makepad_widgets::theme_desktop_dark::*;
    use makepad_draw::shader::std::*;
    use link::styling::*;

    pub DaysOfMonth = {{DaysOfMonth}} {
        width: Fill, height: Fit
        flow: Down

        draw_bg: {
            fn pixel(self) -> vec4 { return vec4(0.0, 0.0, 0.0, 0.0); }
        }

        draw_day: {
            instance hover: 0.0
            instance selected: 0.0
            uniform accent: (THEME_COLOR_ACCENT)

            fn pixel(self) -> vec4 {
                let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                let radius = 3.0;
                sdf.box(0., 0., self.rect_size.x, self.rect_size.y, radius);

                let inactive_fill = mix(#444, #333, self.pos.y);
                let inactive_fill_hover = mix(#555, #444, self.pos.y);
                let inactive_stroke = mix(#555, #111, self.pos.y);
                let inactive_stroke_hover = mix(#666, #222, self.pos.y);

                let accent_light = mix(self.accent, #fff, 0.15);
                let accent_dark = mix(self.accent, #000, 0.4);
                let active_fill = mix(accent_light, self.accent, self.pos.y);
                let active_fill_hover = mix(active_fill, #fff, 0.2);
                let active_stroke = mix(accent_light, accent_dark, self.pos.y);

                let inactive_final_fill = mix(inactive_fill, inactive_fill_hover, self.hover);
                let inactive_final_stroke = mix(inactive_stroke, inactive_stroke_hover, self.hover);

                let active_final_fill = mix(active_fill, active_fill_hover, self.hover);
                let active_final_stroke = active_stroke;

                let fill = mix(inactive_final_fill, active_final_fill, self.selected);
                let stroke = mix(inactive_final_stroke, active_final_stroke, self.selected);

                sdf.fill_keep(fill);
                return sdf.stroke(stroke, 1.0);
            }
        }

        draw_text: {
            instance hover: 0.0
            instance selected: 0.0
            text_style: <THEME_FONT_REGULAR>{ font_size: 11.0 }

            fn get_color(self) -> vec4 {
                let inactive_color = mix(#AAA, #DDD, self.hover);
                return mix(inactive_color, #FFF, self.selected);
            }
        }
    }
}

#[derive(Live, LiveHook, Widget)]
pub struct DaysOfMonth {
    #[redraw] #[live] draw_bg: DrawQuad,
    #[live] draw_day: DrawQuad,
    #[live] draw_text: DrawText,

    #[walk] walk: Walk,
    #[layout] layout: Layout,

    #[rust] selected_mask: u32, // 31 bits for days 1-31
    #[rust] hovered_index: Option<usize>,
    #[rust] hover_state: [f32; 31],
    #[rust] next_frame: NextFrame,
    #[rust] area_days: Vec<Rect>,
}

impl Widget for DaysOfMonth {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, _scope: &mut Scope) {
        // Animation frame handling
        if self.next_frame.is_event(event).is_some() {
            let mut any_animating = false;

            for i in 0..31 {
                let target = if self.hovered_index == Some(i) { 1.0 } else { 0.0 };
                let current = self.hover_state[i];

                if (current - target).abs() > 0.01 {
                    let factor = if target > current { 1.0 } else { 0.15 };
                    self.hover_state[i] += (target - current) * factor;
                    any_animating = true;
                } else {
                    self.hover_state[i] = target;
                }
            }

            if any_animating {
                self.next_frame = cx.new_next_frame();
            }
            self.draw_bg.redraw(cx);
        }

        match event.hits(cx, self.draw_bg.area()) {
            Hit::FingerUp(fe) if fe.was_tap() => {
                for (i, rect) in self.area_days.iter().enumerate() {
                    if rect.contains(fe.abs) {
                        self.selected_mask ^= 1 << i;
                        self.draw_bg.redraw(cx);
                        break;
                    }
                }
            }
            Hit::FingerHoverIn(fe) | Hit::FingerHoverOver(fe) => {
                cx.set_cursor(MouseCursor::Hand);
                let mut new_hovered = None;
                for (i, rect) in self.area_days.iter().enumerate() {
                    if rect.contains(fe.abs) {
                        new_hovered = Some(i);
                        break;
                    }
                }
                if new_hovered != self.hovered_index {
                    self.hovered_index = new_hovered;
                    self.next_frame = cx.new_next_frame();
                    self.draw_bg.redraw(cx);
                }
            }
            Hit::FingerHoverOut(_) => {
                cx.set_cursor(MouseCursor::Arrow);
                if self.hovered_index.is_some() {
                    self.hovered_index = None;
                    self.next_frame = cx.new_next_frame();
                    self.draw_bg.redraw(cx);
                }
            }
            _ => ()
        }
    }

    fn draw_walk(&mut self, cx: &mut Cx2d, _scope: &mut Scope, walk: Walk) -> DrawStep {
        self.draw_bg.begin(cx, walk, self.layout);
        self.area_days.clear();

        let cell_size = dvec2(36.0, 30.0);
        let space_x = 5.0;
        let space_y = 5.0;
        let available_width = cx.turtle().rect().size.x;

        // Calculate how many cells fit per row
        let cols = ((available_width + space_x) / (cell_size.x + space_x)).floor().max(1.0) as usize;

        // Pre-apply values for element 0 (will be used by draw 0)
        {
            let hover = self.hover_state[0];
            let selected_f = if (self.selected_mask >> 0) & 1 == 1 { 1.0 } else { 0.0 };
            self.draw_day.apply_over(cx, live!{ hover: (hover), selected: (selected_f) });
            self.draw_text.apply_over(cx, live!{ hover: (hover), selected: (selected_f) });
        }

        let start_pos = cx.turtle().pos();

        // Draw all 31 days with manual wrap
        for day in 1..=31 {
            let i = day - 1;
            let row = i / cols;
            let col = i % cols;

            // Calculate absolute position
            let x = start_pos.x + col as f64 * (cell_size.x + space_x);
            let y = start_pos.y + row as f64 * (cell_size.y + space_y);

            let cell_rect = Rect {
                pos: dvec2(x, y),
                size: cell_size,
            };
            self.area_days.push(cell_rect);

            // Draw with current instance values (set by previous apply_over)
            self.draw_day.draw_abs(cx, cell_rect);

            let label = format!("{}", day);
            let text_offset_x = cell_size.x * 0.35;
            let text_offset_y = cell_size.y * 0.35;
            let text_pos = dvec2(x + text_offset_x, y + text_offset_y);
            self.draw_text.draw_abs(cx, text_pos, &label);

            // Apply values for current element i (will be used by draw i+1, wrapping fixes offset)
            let hover = self.hover_state[i];
            let selected_f = if (self.selected_mask >> i) & 1 == 1 { 1.0 } else { 0.0 };
            self.draw_day.apply_over(cx, live!{ hover: (hover), selected: (selected_f) });
            self.draw_text.apply_over(cx, live!{ hover: (hover), selected: (selected_f) });
        }

        // Calculate total height and walk turtle down
        let rows = (31 + cols - 1) / cols;
        let total_height = rows as f64 * cell_size.y + (rows - 1) as f64 * space_y;
        cx.walk_turtle(Walk::fixed(0.0, total_height));

        self.draw_bg.end(cx);
        DrawStep::done()
    }
}

impl DaysOfMonth {
    pub fn get_selected_days(&self) -> Vec<usize> {
        let mut days = Vec::new();
        for i in 0..31 {
            if (self.selected_mask >> i) & 1 == 1 {
                days.push(i + 1); // Return 1-31, not 0-30
            }
        }
        days
    }

    pub fn set_selected_days(&mut self, cx: &mut Cx, days: &[usize]) {
        self.selected_mask = 0;
        for &day in days {
            if day >= 1 && day <= 31 {
                self.selected_mask |= 1 << (day - 1);
            }
        }
        self.draw_bg.redraw(cx);
    }
}
