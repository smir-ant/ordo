use makepad_widgets::*;

live_design! {
    use makepad_widgets::base::*;
    
    pub Wrapper = {{Wrapper}} {
        width: Fit, height: Fit
        flow: Down
        show_bg: true
        draw_bg: {
            color: #f0f
        }
    }
}

#[derive(Clone, DefaultNone, Debug)]
pub enum WrapperAction {
    None,
    RightClick,
    LongPress,
    Scroll(Vec2d),
    ShowTooltip{title: String, text: String},
}

#[derive(Live, LiveHook, Widget)]
pub struct Wrapper {
    #[deref] view: View,
    #[rust] last_abs: Option<Vec2d>,
    #[live] blocked: bool,
    #[live] tooltip_text: String,
    #[live] tooltip_title: String,
}

impl Widget for Wrapper {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        if self.blocked {
            match event {
                Event::MouseDown(_) |
                // Event::MouseUp(_) |  
                // Event::MouseMove(_) |
                Event::Scroll(_) |
                Event::TouchUpdate(_) |
                Event::KeyDown(_) |
                Event::KeyUp(_) |
                Event::TextInput(_) => return,
                _ => ()
            }
        }
        self.view.handle_event(cx, event, scope);
        
        match event.hits(cx, self.view.area()) {
            Hit::FingerDown(fe) => {
                self.last_abs = Some(fe.abs);
            }
            Hit::FingerUp(fe) => {
                 self.last_abs = None;
                 if fe.mouse_button() == Some(MouseButton::SECONDARY) {
                     if !self.tooltip_text.is_empty() {
                         let title = if self.tooltip_title.is_empty() { "Info".to_string() } else { self.tooltip_title.clone() };
                         cx.widget_action(self.widget_uid(), &scope.path, WrapperAction::ShowTooltip{title, text: self.tooltip_text.clone()});
                     } else {
                         cx.widget_action(self.widget_uid(), &scope.path, WrapperAction::RightClick);
                     }
                 }
            }
            Hit::FingerLongPress(_fe) => {
                 if !self.tooltip_text.is_empty() {
                     let title = if self.tooltip_title.is_empty() { "Info".to_string() } else { self.tooltip_title.clone() };
                     cx.widget_action(self.widget_uid(), &scope.path, WrapperAction::ShowTooltip{title, text: self.tooltip_text.clone()});
                 } else {
                     cx.widget_action(self.widget_uid(), &scope.path, WrapperAction::LongPress);
                 }
            }
            Hit::FingerMove(fe) => {
                 let last_abs = self.last_abs.unwrap_or(fe.abs);
                 let delta = fe.abs - last_abs;
                 self.last_abs = Some(fe.abs);
                 if delta.x.abs() > 1.0 || delta.y.abs() > 1.0 { // Sensitivity threshold
                    cx.widget_action(self.widget_uid(), &scope.path, WrapperAction::Scroll(delta));
                 }
            }
            _ => ()
        }
    }
    
    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        self.view.draw_walk(cx, scope, walk)
    }
}

impl Wrapper {
    pub fn set_blocked(&mut self, _cx: &mut Cx, blocked: bool) {
        self.blocked = blocked;
    }
}
