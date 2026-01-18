use makepad_widgets::*;

live_design! {
    use makepad_widgets::base::*;
    use makepad_widgets::theme_desktop_dark::*;
    use makepad_draw::shader::std::*;
    use crate::widgets::button::Btn;
    use crate::widgets::text::Text;
    
    Card = {{View}} {
        show_bg: true
        draw_bg: {
            color: #2a2a2a
            instance radius: 8.0
            instance border_width: 0.0
            instance inset: vec4(0.0, 0.0, 0.0, 0.0)
            
            fn pixel(self) -> vec4 {
                let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                sdf.box(
                    self.inset.x + self.border_width,
                    self.inset.y + self.border_width,
                    self.rect_size.x - (self.inset.x + self.inset.z + self.border_width * 2.0),
                    self.rect_size.y - (self.inset.y + self.inset.w + self.border_width * 2.0),
                    max(1.0, self.radius)
                );
                sdf.fill_keep(self.color);
                return sdf.result
            }
        }
    }
    
    pub Modal = {{Modal}} {
        width: Fill, height: Fill
        flow: Overlay
        align: {x: 0.5, y: 0.5}
        
        show_bg: true
        grab_key_focus: true
        draw_bg: {
            fn pixel(self) -> vec4 {
                return vec4(0.0, 0.0, 0.0, 0.8) // Dark dimmed background
            }
        }
        
        // The dialog window
        modal_inner = <Card> {
            width: 400.0, height: Fit
            flow: Down
            spacing: 20.0
            padding: {top: 20.0, right: 20.0, bottom: 20.0, left: 20.0}
            
            draw_bg: {
                radius: 8.0
            }
            
            title = <Text> {
                text: "Confirm Action"
                draw_text: {
                    color: #fff
                    text_style: <THEME_FONT_BOLD> { font_size: 16.0 }
                }
            }
            
            text = <Text> {
                width: Fill, height: Fit
                text: "Are you sure you want to proceed?"
                draw_text: {
                    color: #bbb
                    text_style: <THEME_FONT_REGULAR> { font_size: 14.0 }
                }
            }
            
            buttons_wrap = {{View}} {
                width: Fill, height: Fit
                flow: Right
                align: {x: 1.0} // Right align buttons
                spacing: 15.0
                
                cancel_button = <Btn> {
                    width: 100.0
                    text: "Cancel"
                    reset_hover_on_click: true
                    draw_bg: {
                        color: #444 // Gray for cancel
                    }
                }
                
                ok_button = <Btn> {
                    width: 100.0
                    text: "OK"
                    reset_hover_on_click: true
                    draw_bg: {
                         // Default accent? Or custom?
                         // Let's stick to default Btn style for now or customize
                    }
                }
            }
        }
    }
}

#[derive(Live, LiveHook, Widget)]
pub struct Modal {
    #[deref]
    view: View,
}



#[derive(Clone, DefaultNone, Debug)]
pub enum ModalAction {
    None,
    Dismissed,
    Accepted,
}

impl Widget for Modal {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        let uid = self.widget_uid();
        if self.view.visible() {
             self.view.handle_event(cx, event, scope);
             
             // Handle Escape key to dismiss and Return key to accept
             if let Event::KeyDown(ke) = event {
                 if ke.key_code == KeyCode::Escape {
                     cx.widget_action(uid, &scope.path, ModalAction::Dismissed);
                 } else if ke.key_code == KeyCode::ReturnKey {
                     cx.widget_action(uid, &scope.path, ModalAction::Accepted);
                 }
             }
             
             match event.hits(cx, self.view.area()) {
                 Hit::FingerDown(fe) => {
                       // We hit the overlay (Modal)
                       // Check if we hit the inner card?
                       // Since we hit the overlay, we need to know if the point is ALSO inside the inner card.
                       let inner = self.view.widget(ids!(modal_inner));
                       if !inner.area().rect(cx).contains(fe.abs) {
                           cx.widget_action(uid, &scope.path, ModalAction::Dismissed);
                       }
                 }
                 _ => ()
             }
        }
    }
    
    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        self.view.draw_walk(cx, scope, walk)
    }
    
    fn widget(&self, path: &[LiveId]) -> WidgetRef {
        self.view.widget(path)
    }
    

    

}


