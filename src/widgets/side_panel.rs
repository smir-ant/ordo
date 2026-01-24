use makepad_widgets::*;

live_design! {
    use makepad_widgets::base::*;
    use makepad_widgets::theme_desktop_dark::*;
    use makepad_widgets::view_ui::View;
    use makepad_widgets::scroll_bars::ScrollBars;

    pub SidePanel = {{SidePanel}} {
        width: Fill, height: Fill
        flow: Overlay
        visible: false
        
        show_bg: true
        draw_bg: {
            fn pixel(self) -> vec4 {
                 return vec4(0.0, 0.0, 0.0, 0.6)
            }
        }
        
        // This is the container for user content
        panel = <View> {
            width: 350.0, height: Fill
            flow: Down
            
            show_bg: true
            draw_bg: { color: #232323 }
            
            scroll_bars: <ScrollBars> {}
        }
    }
}

#[derive(Clone, DefaultNone, Debug)]
pub enum SidePanelAction {
    None,
    Close,
}

#[derive(Live, LiveHook, Widget)]
pub struct SidePanel {
    #[deref]
    view: View,
}

impl Widget for SidePanel {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        let uid = self.widget_uid();
        
        if self.view.visible() {
            // 1. Forward events to content
            self.view.handle_event(cx, event, scope);
            
            // 2. Handle Escape key
            if let Event::KeyDown(ke) = event {
                if ke.key_code == KeyCode::Escape {
                    self.close(cx);
                    cx.widget_action(uid, &scope.path, SidePanelAction::Close);
                }
            }
            
            // 3. Handle Click Outside
            // We check if the click hit the SidePanel (which covers the screen) 
            // but NOT the 'panel' (which is the content).
            match event.hits(cx, self.view.area()) {
                Hit::FingerDown(fe) => {
                    // Find the panel widget to check its rect
                    // We assume the user didn't rename 'panel' or we can access it via specific logic.
                    // Since 'panel' is a child in DSL, we can try to find it.
                    // But 'panel' doesn't have a fixed ID unless assigned. 
                    // However, SidePanel.panel is a property in the DSL, usually it gets an ID?
                    // Let's assume the user content is inside the widget with id 'panel'.
                    
                    let panel = self.view.widget(ids!(panel));
                    if panel.area() != Area::Empty {
                         if !panel.area().rect(cx).contains(fe.abs) {
                             self.close(cx);
                             cx.widget_action(uid, &scope.path, SidePanelAction::Close);
                         }
                    } else {
                        // If panel not found (maybe renamed), we might fail to detect click outside correctly.
                        // Fallback: If the hit was on self.view.area() directly?
                        // But self.view.area() covers everything.
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

impl SidePanel {
    pub fn open(&mut self, cx: &mut Cx) {
        self.view.set_visible(cx, true);
        self.view.redraw(cx);
    }
    
    pub fn close(&mut self, cx: &mut Cx) {
        self.view.set_visible(cx, false);
        self.view.redraw(cx);
    }
    
    pub fn toggle(&mut self, cx: &mut Cx) {
        if self.view.visible() {
            self.close(cx);
        } else {
            self.open(cx);
        }
    }
}
