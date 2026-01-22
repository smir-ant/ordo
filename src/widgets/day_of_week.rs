use makepad_widgets::*;

live_design! {
    use makepad_widgets::base::*;
    use makepad_widgets::theme_desktop_dark::*; 
    use makepad_draw::shader::std::*;
    use link::styling::*;

    pub DayOfWeek = {{DayOfWeek}} {
        width: Fill, height: Fit
        spacing: 5.0
        
        // Container background (transparent, for internal hit area tracking)
        draw_bg: {
            fn pixel(self) -> vec4 { return vec4(0.0, 0.0, 0.0, 0.0); }
        }
        
        draw_bg_active: {
            uniform accent_light: (THEME_COLOR_ACCENT_LIGHT)
            uniform accent: (THEME_COLOR_ACCENT)
            uniform accent_stroke_top: (THEME_COLOR_ACCENT_STROKE_TOP)
            uniform accent_dark: (THEME_COLOR_ACCENT_DARK)
            
            fn pixel(self) -> vec4 {
                let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                sdf.circle(self.rect_size.x * 0.5, self.rect_size.y * 0.5, self.rect_size.x * 0.5 - 2.0);
                
                let fill_grad = mix(self.accent_light, self.accent, self.pos.y); 
                sdf.fill_keep(fill_grad);
                
                let stroke_grad = mix(self.accent_stroke_top, self.accent_dark, self.pos.y);
                return sdf.stroke(stroke_grad, 1);
            }
        }
        
        // Hover state for active (selected) days - 20% lighter
        draw_bg_active_hover: {
            uniform accent_light: (THEME_COLOR_ACCENT_LIGHT)
            uniform accent: (THEME_COLOR_ACCENT)
            uniform accent_stroke_top: (THEME_COLOR_ACCENT_STROKE_TOP)
            uniform accent_dark: (THEME_COLOR_ACCENT_DARK)
            
            fn pixel(self) -> vec4 {
                let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                sdf.circle(self.rect_size.x * 0.5, self.rect_size.y * 0.5, self.rect_size.x * 0.5 - 2.0);
                
                // 20% lighter fill on hover
                let base_fill = mix(self.accent_light, self.accent, self.pos.y);
                let fill_grad = mix(base_fill, #fff, 0.2);
                sdf.fill_keep(fill_grad);
                
                let stroke_grad = mix(self.accent_stroke_top, self.accent_dark, self.pos.y);
                return sdf.stroke(stroke_grad, 1);
            }
        }
        
        draw_bg_hover: {
            fn pixel(self) -> vec4 {
                let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                sdf.circle(self.rect_size.x * 0.5, self.rect_size.y * 0.5, self.rect_size.x * 0.5 - 2.0);
                
                let fill_grad = mix(#555, #444, self.pos.y);
                sdf.fill_keep(fill_grad);
                
                let stroke_grad = mix(#666, #222, self.pos.y); 
                return sdf.stroke(stroke_grad, 1);
            }
        }
        
        draw_bg_inactive: {
            fn pixel(self) -> vec4 {
                let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                sdf.circle(self.rect_size.x * 0.5, self.rect_size.y * 0.5, self.rect_size.x * 0.5 - 2.0);
                
                let fill_grad = mix(#444, #333, self.pos.y);
                sdf.fill_keep(fill_grad);
                
                let stroke_grad = mix(#555, #111, self.pos.y); 
                return sdf.stroke(stroke_grad, 1);
            }
        }
        
        draw_text_active: {
            text_style: <THEME_FONT_REGULAR>{ font_size: 9.0 }
            color: #FFF
        }
        
        draw_text_hover: {
            text_style: <THEME_FONT_REGULAR>{ font_size: 9.0 }
            color: #DDD
        }
        
        draw_text_inactive: {
            text_style: <THEME_FONT_REGULAR>{ font_size: 9.0 }
            color: #AAA
        }
    }
}

#[derive(Live, LiveHook, Widget)]
pub struct DayOfWeek {
    #[redraw] #[live] draw_bg: DrawQuad,
    #[live] draw_bg_active: DrawQuad,
    #[live] draw_bg_active_hover: DrawQuad,
    #[live] draw_bg_hover: DrawQuad,
    #[live] draw_bg_inactive: DrawQuad,
    
    #[live] draw_text_active: DrawText,
    #[live] draw_text_hover: DrawText,
    #[live] draw_text_inactive: DrawText,
    
    #[walk] walk: Walk,
    #[layout] layout: Layout,
    
    #[rust] selected_mask: u8,
    #[rust] hovered_index: Option<usize>,
    #[rust] area_days: Vec<Rect>,
}

impl Widget for DayOfWeek {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, _scope: &mut Scope) {
        match event.hits(cx, self.draw_bg.area()) {
            Hit::FingerDown(fe) => {
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
                    self.draw_bg.redraw(cx);
                }
            }
            Hit::FingerHoverOut(_) => {
                if self.hovered_index.is_some() {
                    self.hovered_index = None;
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
        
        for (i, label) in labels.iter().enumerate() {
            let is_selected = (self.selected_mask >> i) & 1 == 1;
            let is_hovered = self.hovered_index == Some(i);
            
            cx.begin_turtle(Walk::fixed(circle_size.x, circle_size.y), Layout {
                align: Align {x: 0.5, y: 0.5},
                ..Layout::default()
            });
            
            let rect = cx.turtle().rect();
            self.area_days.push(rect);
            
            if is_selected && is_hovered {
                // Selected day with hover - use brighter variant
                self.draw_bg_active_hover.draw_abs(cx, rect);
                self.draw_text_active.draw_walk(cx, Walk::fit(), Align::default(), label);
            } else if is_selected {
                self.draw_bg_active.draw_abs(cx, rect);
                self.draw_text_active.draw_walk(cx, Walk::fit(), Align::default(), label);
            } else if is_hovered {
                self.draw_bg_hover.draw_abs(cx, rect);
                self.draw_text_hover.draw_walk(cx, Walk::fit(), Align::default(), label);
            } else {
                self.draw_bg_inactive.draw_abs(cx, rect);
                self.draw_text_inactive.draw_walk(cx, Walk::fit(), Align::default(), label);
            }
            
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
