use makepad_widgets::*;

live_design! {
    use makepad_widgets::base::*;
    use makepad_widgets::theme_desktop_dark::*;
    use makepad_draw::shader::std::*;
    use link::styling::*;

    pub Tabs = {{Tabs}} {
        width: Fill, height: Fit
        
        // Container background (dark inset)
        draw_bg: {
            fn pixel(self) -> vec4 {
                let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                sdf.box(0., 0., self.rect_size.x, self.rect_size.y, 4.0);
                sdf.fill_keep(#2A2A2A);
                let stroke = mix(#222, #333, self.pos.y);
                return sdf.stroke(stroke, 1.0);
            }
        }
        
        // Active tab background (accent colored)
        draw_tab_active: {
            uniform accent: (THEME_COLOR_ACCENT)
            
            fn pixel(self) -> vec4 {
                let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                sdf.box(0., 0., self.rect_size.x, self.rect_size.y, 3.0);
                
                // Derive variants from single accent color
                let accent_light = mix(self.accent, #fff, 0.15);
                let accent_dark = mix(self.accent, #000, 0.3);
                
                let fill = mix(accent_light, self.accent, self.pos.y);
                sdf.fill_keep(fill);
                
                let stroke = mix(accent_light, accent_dark, self.pos.y);
                return sdf.stroke(stroke, 1.0);
            }
        }
        
        // Inactive tab background (transparent)
        draw_tab_inactive: {
            fn pixel(self) -> vec4 {
                return vec4(0.0, 0.0, 0.0, 0.0);
            }
        }
        
        // Active tab hover (20% lighter)
        draw_tab_active_hover: {
            uniform accent: (THEME_COLOR_ACCENT)
            
            fn pixel(self) -> vec4 {
                let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                sdf.box(0., 0., self.rect_size.x, self.rect_size.y, 3.0);
                
                let accent_light = mix(self.accent, #fff, 0.15);
                let accent_dark = mix(self.accent, #000, 0.3);
                
                // 20% lighter on hover
                let base_fill = mix(accent_light, self.accent, self.pos.y);
                let fill = mix(base_fill, #fff, 0.2);
                sdf.fill_keep(fill);
                
                let stroke = mix(accent_light, accent_dark, self.pos.y);
                return sdf.stroke(stroke, 1.0);
            }
        }
        
        // Inactive tab hover (subtle gray highlight)
        draw_tab_hover: {
            fn pixel(self) -> vec4 {
                let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                sdf.box(0., 0., self.rect_size.x, self.rect_size.y, 3.0);
                let fill = mix(#3A3A3A, #333, self.pos.y);
                sdf.fill_keep(fill);
                let stroke = mix(#444, #222, self.pos.y);
                return sdf.stroke(stroke, 1.0);
            }
        }
        
        // Text styles
        draw_text_active: {
            text_style: <THEME_FONT_REGULAR>{ font_size: 11.0 }
            color: #FFF
        }
        
        draw_text_inactive: {
            text_style: <THEME_FONT_REGULAR>{ font_size: 11.0 }
            color: #888
        }
        
        draw_text_hover: {
            text_style: <THEME_FONT_REGULAR>{ font_size: 11.0 }
            color: #AAA
        }
    }
}

#[derive(Live, LiveHook, Widget)]
pub struct Tabs {
    #[redraw] #[live] draw_bg: DrawQuad,
    #[live] draw_tab_active: DrawQuad,
    #[live] draw_tab_active_hover: DrawQuad,
    #[live] draw_tab_inactive: DrawQuad,
    #[live] draw_tab_hover: DrawQuad,
    
    #[live] draw_text_active: DrawText,
    #[live] draw_text_inactive: DrawText,
    #[live] draw_text_hover: DrawText,
    
    #[walk] walk: Walk,
    #[layout] layout: Layout,
    
    #[live] labels: Vec<String>,
    #[rust] selected_index: usize,
    #[rust] hovered_index: Option<usize>,
    #[rust] area_tabs: Vec<Rect>,
}

impl Widget for Tabs {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        let uid = self.widget_uid();
        match event.hits(cx, self.draw_bg.area()) {
            Hit::FingerDown(fe) => {
                for (i, rect) in self.area_tabs.iter().enumerate() {
                    if rect.contains(fe.abs) {
                        if self.selected_index != i {
                            self.selected_index = i;
                            self.draw_bg.redraw(cx);
                            log!("Tabs: changed to index {}", i);
                            cx.widget_action(uid, &scope.path, TabsAction::Changed(i));
                        }
                        break;
                    }
                }
            }
            Hit::FingerHoverIn(fe) | Hit::FingerHoverOver(fe) => {
                cx.set_cursor(MouseCursor::Hand);
                let mut new_hovered = None;
                for (i, rect) in self.area_tabs.iter().enumerate() {
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
        // Begin container with padding
        self.draw_bg.begin(cx, walk, Layout {
            padding: Padding { left: 2.0, top: 2.0, right: 2.0, bottom: 2.0 },
            flow: Flow::right(),
            ..Layout::default()
        });
        
        self.area_tabs.clear();
        let tab_count = self.labels.len().max(1);
        
        // Calculate available width for tabs
        let container_rect = cx.turtle().rect();
        let available_width = container_rect.size.x - 4.0; // minus padding
        let tab_width = available_width / tab_count as f64;
        let tab_height = 24.0;
        
        for (i, label) in self.labels.iter().enumerate() {
            let is_selected = i == self.selected_index;
            let is_hovered = self.hovered_index == Some(i);
            
            cx.begin_turtle(Walk::fixed(tab_width, tab_height), Layout {
                align: Align { x: 0.5, y: 0.5 },
                ..Layout::default()
            });
            
            let rect = cx.turtle().rect();
            self.area_tabs.push(rect);
            
            if is_selected && is_hovered {
                self.draw_tab_active_hover.draw_abs(cx, rect);
                self.draw_text_active.draw_walk(cx, Walk::fit(), Align::default(), label);
            } else if is_selected {
                self.draw_tab_active.draw_abs(cx, rect);
                self.draw_text_active.draw_walk(cx, Walk::fit(), Align::default(), label);
            } else if is_hovered {
                self.draw_tab_hover.draw_abs(cx, rect);
                self.draw_text_hover.draw_walk(cx, Walk::fit(), Align::default(), label);
            } else {
                self.draw_tab_inactive.draw_abs(cx, rect);
                self.draw_text_inactive.draw_walk(cx, Walk::fit(), Align::default(), label);
            }
            
            cx.end_turtle();
        }
        
        self.draw_bg.end(cx);
        DrawStep::done()
    }
}

impl Tabs {
    pub fn selected_index(&self) -> usize {
        self.selected_index
    }
    
    pub fn set_selected_index(&mut self, cx: &mut Cx, index: usize) {
        if index < self.labels.len() && self.selected_index != index {
            self.selected_index = index;
            self.draw_bg.redraw(cx);
        }
    }
    
    pub fn labels(&self) -> &[String] {
        &self.labels
    }
}

#[derive(Clone, Debug, DefaultNone)]
pub enum TabsAction {
    None,
    Changed(usize),
}

impl TabsRef {
    /// Get selected index if tabs changed, returns Some(new_index) when Changed action occurred
    pub fn changed(&self, actions: &Actions) -> Option<usize> {
        if let Some(item) = actions.find_widget_action(self.widget_uid()) {
            if let TabsAction::Changed(i) = item.cast() {
                return Some(i);
            }
        }
        None
    }
    
    /// Get the current selected index
    pub fn selected_index(&self) -> usize {
        if let Some(inner) = self.borrow() {
            inner.selected_index
        } else {
            0
        }
    }
    
    /// Set selected index programmatically
    pub fn set_selected_index(&self, cx: &mut Cx, index: usize) {
        if let Some(mut inner) = self.borrow_mut() {
            inner.set_selected_index(cx, index);
        }
    }
}
