// Note: "CheckBox" name is taken by makepad_widgets standard widget.
// We use "Check" as our custom implementation.

use makepad_widgets::*;

live_design! {
    use makepad_widgets::base::*;
    use makepad_widgets::theme_desktop_dark::*;
    use makepad_draw::shader::std::*;
    use link::styling::*;
    use link::shaders::*;

    pub Check = {{Check}} {
        width: Fit, height: Fit
        flow: Right, align: {y: 0.5}, spacing: 8.0
        
        draw_bg: {
            instance checked: 0.0
            instance hover: 0.0
            uniform accent: (THEME_COLOR_ACCENT)
            
            fn pixel(self) -> vec4 {
                let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                // Inset by 1px to prevent stroke clipping, use 2px radius for slight rounding
                sdf.box(1., 1., self.rect_size.x - 2., self.rect_size.y - 2., 2.0);
                
                // Inactive colors
                let inactive_fill = mix(#2A2A2A, #3A3A3A, self.hover);
                let inactive_stroke = mix(#444, #555, self.hover);
                
                // Active colors (accent)
                let accent_light = mix(self.accent, #fff, 0.15);
                let accent_dark = mix(self.accent, #000, 0.3);
                let active_fill = mix(accent_light, self.accent, self.pos.y);
                let active_fill_hover = mix(active_fill, #fff, 0.2 * self.hover);
                let active_stroke = mix(accent_light, accent_dark, self.pos.y);
                
                // Mix based on checked state
                let fill = mix(inactive_fill, active_fill_hover, self.checked);
                let stroke = mix(inactive_stroke, active_stroke, self.checked);
                
                sdf.fill_keep(fill);
                return sdf.stroke(stroke, 1.0);
            }
        }
        
        draw_label: {
            instance hover: 0.0
            text_style: <THEME_FONT_REGULAR> { font_size: 11.0 }
            fn get_color(self) -> vec4 {
                return mix(#BFBFBF, #FFFFFF, self.hover);
            }
        }
        
        label: ""
    }
}

#[derive(Live, LiveHook, Widget)]
pub struct Check {
    #[redraw] #[live] draw_bg: DrawQuad,
    #[live] draw_label: DrawText,
    
    #[walk] walk: Walk,
    #[layout] layout: Layout,
    
    #[live] label: ArcStringMut,
    #[live(false)] pub checked: bool,
    #[rust] is_hovered: bool,
    #[rust] area: Area,

}

impl Widget for Check {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        let uid = self.widget_uid();
        // Use combined area for hit testing (checkbox + label)
        match event.hits(cx, self.area) {
            Hit::FingerUp(fe) if fe.was_tap() => {
                self.checked = !self.checked;
                log!("Check: toggled to {}", self.checked);
                cx.widget_action(uid, &scope.path, CheckAction::Changed(self.checked));
                self.redraw(cx);
            }
            Hit::FingerHoverIn(_) => {
                cx.set_cursor(MouseCursor::Hand);
                self.is_hovered = true;
                self.redraw(cx);
            }
            Hit::FingerHoverOut(_) => {
                self.is_hovered = false;
                self.redraw(cx);
            }
            _ => ()
        }
    }

    fn draw_walk(&mut self, cx: &mut Cx2d, _scope: &mut Scope, walk: Walk) -> DrawStep {
        let box_size = 16.0;
        
        // Begin the widget container
        cx.begin_turtle(walk, self.layout);
        
        // Set instance values for checkbox
        self.draw_bg.apply_over(cx, live!{
            checked: (if self.checked { 1.0 } else { 0.0 }),
            hover: (if self.is_hovered { 1.0 } else { 0.0 })
        });
        
        // Draw checkbox
        let box_rect = cx.walk_turtle(Walk::fixed(box_size, box_size));
        self.draw_bg.draw_abs(cx, box_rect);
        
        // Draw label if present
        let label = self.label.as_ref();
        if !label.is_empty() {
            self.draw_label.apply_over(cx, live!{
                hover: (if self.is_hovered { 1.0 } else { 0.0 })
            });
            self.draw_label.draw_walk(cx, Walk::fit(), Align::default(), label);
        }
        
        // End container and store combined area for hit testing
        cx.end_turtle_with_area(&mut self.area);
        
        DrawStep::done()
    }
    
    fn text(&self) -> String {
        self.label.as_ref().to_string()
    }
    
    fn set_text(&mut self, _cx: &mut Cx, v: &str) {
        self.label.as_mut_empty().push_str(v);
    }
}

impl Check {
    pub fn checked(&self) -> bool {
        self.checked
    }
    
    pub fn set_checked(&mut self, cx: &mut Cx, checked: bool) {
        if self.checked != checked {
            self.checked = checked;
            self.redraw(cx);
        }
    }
}

#[derive(Clone, Debug, DefaultNone)]
pub enum CheckAction {
    None,
    Changed(bool),
}

impl CheckRef {
    /// Get new checked state if changed, returns Some(new_state) when Changed action occurred
    pub fn changed(&self, actions: &Actions) -> Option<bool> {
        if let Some(item) = actions.find_widget_action(self.widget_uid()) {
            if let CheckAction::Changed(state) = item.cast() {
                return Some(state);
            }
        }
        None
    }
    
    /// Get the current checked state
    pub fn checked(&self) -> bool {
        if let Some(inner) = self.borrow() {
            inner.checked
        } else {
            false
        }
    }
    
    /// Set checked state programmatically
    pub fn set_checked(&self, cx: &mut Cx, checked: bool) {
        if let Some(mut inner) = self.borrow_mut() {
            inner.set_checked(cx, checked);
        }
    }
}
