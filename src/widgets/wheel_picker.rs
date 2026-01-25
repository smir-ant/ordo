use makepad_widgets::*;

#[derive(Clone, Debug, DefaultNone)]
pub enum WheelPickerAction {
    None,
    Changed(i32),
}

live_design! {
    use link::theme::*;
    use link::shaders::*;
    use crate::styling::*;
    
    pub WheelPicker = {{WheelPicker}} {
        width: 100.0, height: 160.0
        
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
                sdf.box(
                    0.0,
                    0.0,
                    self.rect_size.x,
                    self.rect_size.y,
                    self.border_radius
                )
                sdf.fill_keep(vec4(self.color.xyz, 1));
                
                let stroke_top = #fff5;
                let stroke_bottom = #0005; 
                let stroke_color = mix(stroke_top, stroke_bottom, self.pos.y);
                
                return sdf.stroke(stroke_color, 1.0);
            }
        }
        
        step_height: 40.0
        range_min: 0
        range_max: 23
    }
}

#[derive(Live, LiveHook, Widget)]
pub struct WheelPicker {
    #[redraw] #[live] draw_text: DrawText,
    #[live] draw_bg: DrawQuad,
    #[live] draw_selection: DrawQuad,
    
    #[live] step_height: f64,
    #[live] range_min: i32,
    #[live] range_max: i32,
    #[live(true)] is_infinite: bool,
    
    #[layout] layout: Layout,
    #[walk] walk: Walk,
    
    #[rust] scroll_pos: f64,
    #[rust] current_value: i32,
    #[rust] is_dragging: bool,
    #[rust] last_abs_y: f64,
    #[rust] drag_start_y: f64,
    
    #[rust] next_frame: NextFrame,
    #[rust] scroll_target: Option<f64>,
    #[rust] scroll_cooldown: Option<i32>,
}

impl Widget for WheelPicker {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        // Animation handling
        if let Some(ne) = self.next_frame.is_event(event) {
             if let Some(target) = self.scroll_target {
                 let delta = target - self.scroll_pos;
                 if delta.abs() < 1.0 {
                     self.scroll_pos = target;
                     self.scroll_target = None;
                 } else {
                     self.scroll_pos += delta * 0.2; // Smoothness factor
                     self.next_frame = cx.new_next_frame();
                 }
                if !self.is_infinite { self.clamp_scroll(); }
                 self.draw_bg.redraw(cx);
                 self.draw_selection.redraw(cx);
                 self.update_value(cx, &scope.path); // Update value during animation
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
        
        // Global Scroll Handler to consume events
        if let Event::Scroll(e) = event {
            if self.draw_bg.area().rect(cx).contains(e.abs) {
                // Consume event to prevent parent scrolling
                e.handled_x.set(true);
                e.handled_y.set(true);
                
                self.scroll_target = None; // Cancel animation
                self.scroll_pos += e.scroll.y;
                if !self.is_infinite { self.clamp_scroll(); }
                self.draw_bg.redraw(cx);
                self.draw_selection.redraw(cx);
                self.update_value(cx, &scope.path);
                
                // Trigger cooldown for snap
                self.scroll_cooldown = Some(10);
                self.next_frame = cx.new_next_frame();
            }
        }

        match event.hits_with_sweep_area(cx, self.draw_bg.area(), self.draw_bg.area()) {
            Hit::FingerDown(fe) => {
                cx.sweep_lock(self.draw_bg.area());
                self.is_dragging = true;
                self.last_abs_y = fe.abs.y;
                self.drag_start_y = fe.abs.y;
                self.scroll_target = None; // Cancel animation
                self.scroll_cooldown = None;
            }
            Hit::FingerMove(fe) => {
                if self.is_dragging {
                    let delta = fe.abs.y - self.last_abs_y;
                    self.scroll_pos -= delta;
                    if !self.is_infinite { self.clamp_scroll(); }
                    self.last_abs_y = fe.abs.y;
                    self.draw_bg.redraw(cx);
                    self.draw_selection.redraw(cx);
                    self.update_value(cx, &scope.path); // Update value while dragging
                }
            }
            Hit::FingerUp(fe) => {
                cx.sweep_unlock(self.draw_bg.area());
                self.is_dragging = false;
                
                // Detect Tap
                if (fe.abs.y - self.drag_start_y).abs() < 10.0 {
                    let rect = fe.rect;
                    let center_y = rect.pos.y + rect.size.y * 0.5;
                    let touch_offset = fe.abs.y - center_y;
                    let steps_offset = (touch_offset / self.step_height).round();
                    
                    if steps_offset != 0.0 {
                         let target_pos = self.scroll_pos + steps_offset * self.step_height;
                         self.scroll_target = Some(target_pos);
                         self.next_frame = cx.new_next_frame();
                         self.draw_bg.redraw(cx);
                         self.draw_selection.redraw(cx);
                         return; 
                    }
                }
                
                self.snap_to_grid(cx);
                self.draw_bg.redraw(cx);
                self.draw_selection.redraw(cx);
            }
            _ => ()
        }
    }

    fn draw_walk(&mut self, cx: &mut Cx2d, _scope: &mut Scope, walk: Walk) -> DrawStep {
        self.draw_bg.begin(cx, walk, self.layout);
        
        let rect = cx.turtle().rect();
        let center_y = rect.size.y * 0.5;
        
        // Draw Selection Indicator (Center)
        let selection_rect = Rect {
            pos: dvec2(rect.pos.x, rect.pos.y + center_y - self.step_height * 0.5),
            size: dvec2(rect.size.x, self.step_height),
        };
        self.draw_selection.draw_abs(cx, selection_rect);
        
        // Infinite Scroll Rendering
        // Calculate the index of the item currently at the center
        let center_idx = (self.scroll_pos / self.step_height).round() as i32;
        
        // Render a window of items around the center
        // +/- 10 items covers the screen amply
        let window = 10;
        
        for i in (center_idx - window)..=(center_idx + window) {
             let offset = (i as f64) * self.step_height - self.scroll_pos;
             
             // Simple culling
             if offset < -rect.size.y || offset > rect.size.y {
                 continue;
             }
             
             let dist = offset.abs();
             let max_dist = rect.size.y * 0.6;
             
             if dist < max_dist {
                 let range_len = self.range_max - self.range_min + 1;
                 
                 // Value calculation depending on mode
                 let val = if self.is_infinite {
                     (i.rem_euclid(range_len)) + self.range_min
                 } else {
                     if i < 0 || i >= range_len { continue; } // Don't draw outside range
                     i + self.range_min
                 };
                 
                 let opacity = 1.0 - (dist / max_dist).powf(1.5);
                 let scale = 1.0 - (dist / max_dist) * 0.3;
                 
                 self.draw_text.color.w = opacity as f32;
                 self.draw_text.text_style.font_size = (20.0 * scale) as f32;
                 
                 let text = format!("{:02}", val);
                 
                 let laidout = self.draw_text.layout(
                        cx,
                        0.0, 0.0, None, false, Align::default(),
                        &text
                 );
                 let width = laidout.size_in_lpxs.width as f64 * self.draw_text.font_scale as f64;
                 let height = laidout.size_in_lpxs.height as f64 * self.draw_text.font_scale as f64;
                 
                 let pos = dvec2(
                     rect.pos.x + (rect.size.x - width) * 0.5,
                     rect.pos.y + center_y + offset - height * 0.5
                 );
                 
                 self.draw_text.draw_abs(cx, pos, &text);
             }
        }
        
        self.draw_bg.end(cx);
        DrawStep::done()
    }
}

impl WheelPicker {
    fn clamp_scroll(&mut self) {
        if !self.is_infinite {
            let max_idx = (self.range_max - self.range_min) as f64;
            let max_pos = max_idx * self.step_height;
            if self.scroll_pos < 0.0 { self.scroll_pos = 0.0; }
            if self.scroll_pos > max_pos { self.scroll_pos = max_pos; }
        }
    }

    fn snap_to_grid(&mut self, cx: &mut Cx) {
        let exact_idx = self.scroll_pos / self.step_height;
        let mut snapped_idx = exact_idx.round();
        
        if !self.is_infinite {
             let max_idx = (self.range_max - self.range_min) as f64;
             if snapped_idx < 0.0 { snapped_idx = 0.0; }
             if snapped_idx > max_idx { snapped_idx = max_idx; }
        }
        
        let target = snapped_idx * self.step_height;
        
        self.scroll_target = Some(target);
        self.next_frame = cx.new_next_frame();
    }
    
    fn update_value(&mut self, cx: &mut Cx, path: &HeapLiveIdPath) {
        let raw_idx = (self.scroll_pos / self.step_height).round() as i32;
        let range_len = self.range_max - self.range_min + 1;
        
        if self.is_infinite {
            self.current_value = (raw_idx.rem_euclid(range_len)) + self.range_min;
        } else {
             // Clamped value
             let mut idx = raw_idx;
             if idx < 0 { idx = 0; }
             if idx >= range_len { idx = range_len - 1; }
             self.current_value = idx + self.range_min;
        }
        cx.widget_action(self.widget_uid(), &path, WheelPickerAction::Changed(self.current_value));
    }
    
    pub fn get_value(&self) -> i32 {
        self.current_value
    }
    
    pub fn set_range(&mut self, min: i32, max: i32) {
        self.range_min = min;
        self.range_max = max;
    }
    
    pub fn set_value(&mut self, cx: &mut Cx, val: i32) {
        let val = val.clamp(self.range_min, self.range_max);
        self.current_value = val;
        let idx = (val - self.range_min) as f64;
        self.scroll_pos = idx * self.step_height;
        self.draw_bg.redraw(cx);
        self.draw_selection.redraw(cx);
    }
}