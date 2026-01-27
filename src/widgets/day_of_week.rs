use makepad_widgets::*;

live_design! {
    use makepad_widgets::base::*;
    use makepad_widgets::theme_desktop_dark::*;
    use makepad_draw::shader::std::*;
    use link::styling::*;

    pub DayOfWeek = {{DayOfWeek}} {
        width: Fill, height: Fit
        spacing: 5.0

        // Container (transparent, for hit area)
        draw_bg: {
            fn pixel(self) -> vec4 { return vec4(0.0, 0.0, 0.0, 0.0); }
        }

        // Single unified day background shader
        draw_day: {
            instance hover: 0.0
            instance selected: 0.0
            uniform accent: (THEME_COLOR_ACCENT)

            fn pixel(self) -> vec4 {
                let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                let radius = self.rect_size.x * 0.5 - 2.0;
                sdf.circle(self.rect_size.x * 0.5, self.rect_size.y * 0.5, radius);

                // Inactive colors
                let inactive_fill = mix(#444, #333, self.pos.y);
                let inactive_fill_hover = mix(#555, #444, self.pos.y);
                let inactive_stroke = mix(#555, #111, self.pos.y);
                let inactive_stroke_hover = mix(#666, #222, self.pos.y);

                // Active (accent) colors
                let accent_light = mix(self.accent, #fff, 0.15);
                let accent_dark = mix(self.accent, #000, 0.4);
                let active_fill = mix(accent_light, self.accent, self.pos.y);
                let active_fill_hover = mix(active_fill, #fff, 0.2);
                let active_stroke = mix(accent_light, accent_dark, self.pos.y);

                // Mix inactive with hover
                let inactive_final_fill = mix(inactive_fill, inactive_fill_hover, self.hover);
                let inactive_final_stroke = mix(inactive_stroke, inactive_stroke_hover, self.hover);

                // Mix active with hover
                let active_final_fill = mix(active_fill, active_fill_hover, self.hover);
                let active_final_stroke = active_stroke; // stroke doesn't change on hover for active

                // Final mix based on selected
                let fill = mix(inactive_final_fill, active_final_fill, self.selected);
                let stroke = mix(inactive_final_stroke, active_final_stroke, self.selected);

                sdf.fill_keep(fill);
                return sdf.stroke(stroke, 1.0);
            }
        }

        draw_text: {
            instance hover: 0.0
            instance selected: 0.0
            text_style: <THEME_FONT_REGULAR>{ font_size: 9.0 }

            fn get_color(self) -> vec4 {
                // Inactive: #AAA -> #DDD on hover
                // Active: #FFF always
                let inactive_color = mix(#AAA, #DDD, self.hover);
                return mix(inactive_color, #FFF, self.selected);
            }
        }
    }
}

#[derive(Live, LiveHook, Widget)]
pub struct DayOfWeek {
    #[redraw] #[live] draw_bg: DrawQuad,
    #[live] draw_day: DrawQuad,
    #[live] draw_text: DrawText,

    #[walk] walk: Walk,
    #[layout] layout: Layout,

    #[rust] selected_mask: u8,
    #[rust] hovered_index: Option<usize>,
    #[rust] hover_state: [f32; 7],
    #[rust] next_frame: NextFrame,
    #[rust] area_days: Vec<Rect>,
}

impl Widget for DayOfWeek {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, _scope: &mut Scope) {
        // Animation frame handling
        if self.next_frame.is_event(event).is_some() {
            let mut any_animating = false;

            for i in 0..7 {
                let target = if self.hovered_index == Some(i) { 1.0 } else { 0.0 };
                let current = self.hover_state[i];

                if (current - target).abs() > 0.01 {
                    // Lerp towards target (0.15 factor = ~0.1-0.2s feel at 60fps)
                    self.hover_state[i] += (target - current) * 0.15;
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

        let labels = ["Mo", "Tu", "We", "Th", "Fr", "Sa", "Su"];
        self.area_days.clear();

        let circle_size = dvec2(34.0, 34.0);
        let space = 3.0;

        // Pre-apply values for element 0 (will be used by draw 0)
        {
            let hover = self.hover_state[0];
            let selected_f = if (self.selected_mask >> 0) & 1 == 1 { 1.0 } else { 0.0 };
            self.draw_day.apply_over(cx, live!{ hover: (hover), selected: (selected_f) });
            self.draw_text.apply_over(cx, live!{ hover: (hover), selected: (selected_f) });
        }

        for (i, label) in labels.iter().enumerate() {
            cx.begin_turtle(Walk::fixed(circle_size.x, circle_size.y), Layout {
                align: Align {x: 0.5, y: 0.5},
                ..Layout::default()
            });

            let rect = cx.turtle().rect();
            self.area_days.push(rect);

            // Draw with current instance values (set by previous apply_over)
            self.draw_day.draw_abs(cx, rect);
            self.draw_text.draw_walk(cx, Walk::fit(), Align::default(), label);

            // Apply values for current element i (will be used by draw i+1, wrapping fixes offset)
            let hover = self.hover_state[i];
            let selected_f = if (self.selected_mask >> i) & 1 == 1 { 1.0 } else { 0.0 };
            self.draw_day.apply_over(cx, live!{ hover: (hover), selected: (selected_f) });
            self.draw_text.apply_over(cx, live!{ hover: (hover), selected: (selected_f) });

            cx.end_turtle();

            if i < 6 {
                cx.walk_turtle(Walk::fixed(space, 0.0));
            }
        }

        self.draw_bg.end(cx);
        DrawStep::done()
    }
}

impl DayOfWeek {
    pub fn get_selected_days(&self) -> Vec<usize> {
        let mut days = Vec::new();
        for i in 0..7 {
            if (self.selected_mask >> i) & 1 == 1 {
                days.push(i);
            }
        }
        days
    }
}
