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
    DoubleTap,
}

#[derive(Live, LiveHook, Widget)]
pub struct Wrapper {
    #[deref] view: View,
}

impl Widget for Wrapper {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        self.view.handle_event(cx, event, scope);
        
        match event.hits(cx, self.view.area()) {
            Hit::FingerDown(_fe) => {
                 // log!("Wrapper: FingerDown Detected!");
            }
            Hit::FingerUp(fe) => {
                 if fe.mouse_button() == Some(MouseButton::SECONDARY) {
                     log!("Wrapper: Right Click Detected! Emitting Action.");
                     cx.widget_action(self.widget_uid(), &scope.path, WrapperAction::RightClick);
                 }
                 if fe.tap_count > 1 {
                     log!("Wrapper: Double Tap Detected! Emitting Action.");
                     cx.widget_action(self.widget_uid(), &scope.path, WrapperAction::DoubleTap);
                 }
            }
            _ => ()
        }
    }
    
    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        self.view.draw_walk(cx, scope, walk)
    }
}
