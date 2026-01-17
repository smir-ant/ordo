use makepad_widgets::*;

live_design! {
    use makepad_widgets::base::*;
    use makepad_widgets::theme_desktop_dark::*;
    use crate::widgets::button::Btn;
    use crate::widgets::text::Text;
    
    pub Modal = {{Modal}} {
        width: Fill, height: Fill
        flow: Overlay
        align: {x: 0.5, y: 0.5}
        
        show_bg: true
        draw_bg: {
            fn pixel(self) -> vec4 {
                return vec4(0.0, 0.0, 0.0, 0.8) // Dark dimmed background
            }
        }
        
        // The dialog window
        modal_inner = {{View}} {
            width: 400.0, height: Fit
            flow: Down
            spacing: 20.0
            padding: 20.0
            
            show_bg: true
            draw_bg: {
                color: #2a2a2a
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



impl Widget for Modal {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        if self.view.visible() {
             self.view.handle_event(cx, event, scope);
        }
    }
    
    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        self.view.draw_walk(cx, scope, walk)
    }
    
    fn widget(&self, path: &[LiveId]) -> WidgetRef {
        self.view.widget(path)
    }
    

    

}

impl Modal {
    pub fn ok_clicked(&self, actions: &Actions) -> bool {
        // Helper to check OK button inside content
        // This requires knowing the ID path.
        // Assuming user assigns `ok_button` id in usage or we traverse.
        // Better to let the user do `self.ui.widget(ids!(my_modal, ok_button))`?
        // For now, let's keep it simple.
        false
    }
}
