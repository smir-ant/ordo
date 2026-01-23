// Details widget - collapsible container like HTML <details> element
// Similar to "Advanced Settings" section in check.html

use makepad_widgets::*;

live_design! {
    use makepad_widgets::base::*;
    use makepad_widgets::theme_desktop_dark::*;
    use makepad_draw::shader::std::*;
    use makepad_widgets::view_ui::View;
    use link::styling::*;
    use link::shaders::*;

    pub Details = {{Details}} {
        width: Fill, height: Fit
        flow: Down, spacing: 0.0
        
        // Arrow shader - changes color based on hover
        draw_arrow: {
            instance open: 0.0
            instance hover: 0.0
            
            fn pixel(self) -> vec4 {
                let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                let c = self.rect_size * 0.5;
                let sz = 4.0;
                
                // Rotate arrow based on open state
                if self.open > 0.5 {
                    // Down arrow (open)
                    sdf.move_to(c.x - sz, c.y - sz * 0.5);
                    sdf.line_to(c.x, c.y + sz * 0.5);
                    sdf.line_to(c.x + sz, c.y - sz * 0.5);
                } else {
                    // Right arrow (closed)
                    sdf.move_to(c.x - sz * 0.5, c.y - sz);
                    sdf.line_to(c.x + sz * 0.5, c.y);
                    sdf.line_to(c.x - sz * 0.5, c.y + sz);
                }
                
                // Brighten on hover
                let color = mix(#888, #CCC, self.hover);
                return sdf.stroke(color, 1.5);
            }
        }
        
        // Title text - normal color, brightens on hover
        draw_title: {
            instance hover: 0.0
            text_style: <THEME_FONT_REGULAR> { font_size: 13.0 }
            fn get_color(self) -> vec4 {
                return mix(#DDD, #FFF, self.hover);
            }
        }
        
        // Separator line
        draw_separator: {
            fn pixel(self) -> vec4 {
                return mix(#333, #444, self.pos.x);
            }
        }
        
        summary: "Details"
        
        // Content placeholder - children go here
        content = <View> {
            width: Fill, height: Fit
            flow: Down, spacing: 8.0
            padding: {left: 24.0, top: 8.0, bottom: 8.0}
            visible: false
        }
    }
}

#[derive(Live, LiveHook, Widget)]
pub struct Details {
    #[redraw] #[live] draw_arrow: DrawQuad,
    #[live] draw_title: DrawText,
    #[live] draw_separator: DrawQuad,
    
    #[deref] view: View,
    
    #[walk] walk: Walk,
    #[layout] layout: Layout,
    
    #[live] summary: ArcStringMut,
    #[live(false)] pub open: bool,
    #[rust] is_hovered: bool,
    #[rust] header_area: Area,
}

impl Widget for Details {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        self.view.handle_event(cx, event, scope);
        
        let uid = self.widget_uid();
        
        // Handle clicks and hover on header area
        match event.hits(cx, self.header_area) {
            Hit::FingerUp(fe) if fe.was_tap() => {
                self.open = !self.open;
                log!("Details: toggled to {}", if self.open { "open" } else { "closed" });
                cx.widget_action(uid, &scope.path, DetailsAction::Toggled(self.open));
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

    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        // Begin main container with full width
        cx.begin_turtle(walk, Layout {
            flow: Flow::Down,
            ..Default::default()
        });
        
        // Draw header row with full width
        cx.begin_turtle(Walk::fill_fit(), Layout {
            flow: Flow::right(),
            align: Align { x: 0.0, y: 0.5 },
            padding: Padding { top: 8.0, bottom: 8.0, left: 0.0, right: 0.0 },
            ..Default::default()
        });
        
        // Update and draw arrow (with hover)
        self.draw_arrow.apply_over(cx, live!{
            open: (if self.open { 1.0 } else { 0.0 }),
            hover: (if self.is_hovered { 1.0 } else { 0.0 })
        });
        let arrow_rect = cx.walk_turtle(Walk::fixed(20.0, 20.0));
        self.draw_arrow.draw_abs(cx, arrow_rect);
        
        // Update and draw title (with hover)
        self.draw_title.apply_over(cx, live!{
            hover: (if self.is_hovered { 1.0 } else { 0.0 })
        });
        self.draw_title.draw_walk(cx, Walk::fit(), Align::default(), self.summary.as_ref());
        
        // End header and capture area
        cx.end_turtle_with_area(&mut self.header_area);
        
        // Draw separator and content if open
        if self.open {
            // Draw separator line
            let sep_rect = cx.walk_turtle(Walk {
                width: Size::fill(),
                height: Size::Fixed(1.0),
                margin: Margin { left: 24.0, top: 0.0, right: 0.0, bottom: 0.0 },
                ..Default::default()
            });
            self.draw_separator.draw_abs(cx, sep_rect);
        }
        
        // Update content visibility and draw view
        self.view.apply_over(cx, live!{
            content = { visible: (self.open) }
        });
        let _ = self.view.draw(cx, scope);
        
        // End main container
        cx.end_turtle();
        
        DrawStep::done()
    }
}

impl Details {
    pub fn is_open(&self) -> bool {
        self.open
    }
    
    pub fn set_open(&mut self, cx: &mut Cx, open: bool) {
        if self.open != open {
            self.open = open;
            self.redraw(cx);
        }
    }
    
    pub fn toggle(&mut self, cx: &mut Cx) {
        self.open = !self.open;
        self.redraw(cx);
    }
}

#[derive(Clone, Debug, DefaultNone)]
pub enum DetailsAction {
    None,
    Toggled(bool),
}

impl DetailsRef {
    pub fn toggled(&self, actions: &Actions) -> Option<bool> {
        if let Some(item) = actions.find_widget_action(self.widget_uid()) {
            if let DetailsAction::Toggled(state) = item.cast() {
                return Some(state);
            }
        }
        None
    }
    
    pub fn is_open(&self) -> bool {
        if let Some(inner) = self.borrow() {
            inner.open
        } else {
            false
        }
    }
    
    pub fn set_open(&self, cx: &mut Cx, open: bool) {
        if let Some(mut inner) = self.borrow_mut() {
            inner.set_open(cx, open);
        }
    }
    
    pub fn toggle(&self, cx: &mut Cx) {
        if let Some(mut inner) = self.borrow_mut() {
            inner.toggle(cx);
        }
    }
}
