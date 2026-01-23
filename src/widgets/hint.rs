use makepad_widgets::*;

live_design! {
    use makepad_widgets::base::*;
    
    pub Hint = {{Hint}} {
        width: Fit, height: Fit
        flow: Down
        show_bg: true
        draw_bg: {
            color: #f0f
        }
    }
}

#[derive(Clone, DefaultNone, Debug)]
pub enum HintAction {
    None,
    RightClick,
    LongPress,
    Scroll(Vec2d),
    ShowTooltip{title: String, text: String},
}

#[derive(Live, LiveHook, Widget)]
pub struct Hint {
    #[deref] view: View,
    #[rust] last_abs: Option<Vec2d>,
    #[live] blocked: bool,
    #[live] tooltip_text: String,
    #[live] tooltip_title: String,
}

impl Hint {
    /// Emit ShowTooltip action with configured title/text
    fn emit_tooltip(&self, cx: &mut Cx, scope: &Scope) {
        if !self.tooltip_text.is_empty() {
            let title = if self.tooltip_title.is_empty() { 
                "Info".to_string() 
            } else { 
                self.tooltip_title.clone() 
            };
            cx.widget_action(
                self.widget_uid(), 
                &scope.path, 
                HintAction::ShowTooltip { title, text: self.tooltip_text.clone() }
            );
        }
    }
    
    /// Check if position is inside hint area
    fn contains_pos(&self, cx: &Cx, pos: DVec2) -> bool {
        self.view.area().rect(cx).contains(pos)
    }
    
    pub fn set_blocked(&mut self, _cx: &mut Cx, blocked: bool) {
        self.blocked = blocked;
    }
}

impl Widget for Hint {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        if self.blocked {
            match event {
                Event::MouseDown(_) |
                Event::Scroll(_) |
                Event::TouchUpdate(_) |
                Event::KeyDown(_) |
                Event::KeyUp(_) |
                Event::TextInput(_) => return,
                _ => ()
            }
        }
        
        // Intercept long press and right click BEFORE passing to children
        // This ensures tooltip works even when clicking on child widgets (buttons, inputs, etc.)
        let mut intercepted = false;
        
        match event {
            // Mobile: Long press anywhere in hint area
            Event::LongPress(lp) => {
                if self.contains_pos(cx, lp.abs) {
                    self.emit_tooltip(cx, scope);
                    intercepted = true;
                }
            }
            // Desktop: Right click (secondary button) anywhere in hint area
            Event::MouseUp(mu) => {
                if mu.button == MouseButton::SECONDARY && self.contains_pos(cx, mu.abs) {
                    self.emit_tooltip(cx, scope);
                    intercepted = true;
                }
            }
            _ => ()
        }
        
        // Pass event to children (they handle their own clicks, hovers, etc.)
        self.view.handle_event(cx, event, scope);
        
        // Handle scroll gesture through hits (for scroll propagation)
        if !intercepted {
            match event.hits(cx, self.view.area()) {
                Hit::FingerDown(fe) => {
                    self.last_abs = Some(fe.abs);
                }
                Hit::FingerUp(_fe) => {
                    self.last_abs = None;
                }
                Hit::FingerMove(fe) => {
                    let last_abs = self.last_abs.unwrap_or(fe.abs);
                    let delta = fe.abs - last_abs;
                    self.last_abs = Some(fe.abs);
                    if delta.x.abs() > 1.0 || delta.y.abs() > 1.0 {
                        cx.widget_action(self.widget_uid(), &scope.path, HintAction::Scroll(delta));
                    }
                }
                _ => ()
            }
        }
    }
    
    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        self.view.draw_walk(cx, scope, walk)
    }
}
