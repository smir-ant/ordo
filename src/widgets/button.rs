use makepad_widgets::*;

live_design! {
    use link::shaders::*;
    use makepad_widgets::base::*;
    use makepad_widgets::theme_desktop_dark::*; 
    use crate::theme::*;

    pub Button = {{Button}} {
        width: Fill, height: (THEME_BUTTON_HEIGHT)
        
        draw_text: {
            color: (THEME_BUTTON_TEXT_COLOR)
            text_style: <THEME_FONT_REGULAR> { font_size: (THEME_BUTTON_FONT_SIZE) }
            instance hover: 0.0
            instance down: 0.0
        }
        
        draw_bg: {
            instance hover: 0.0
            instance down: 0.0
            instance border_width: 1.0
            instance border_color: (THEME_BUTTON_BORDER_COLOR)
            instance color: (THEME_BUTTON_BG_COLOR)
            instance radius: (THEME_BUTTON_RADIUS)
            
            fn pixel(self) -> vec4 {
                let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                sdf.box(
                    self.border_width,
                    self.border_width,
                    self.rect_size.x - self.border_width * 2.0,
                    self.rect_size.y - self.border_width * 2.0,
                    self.radius
                );
                
                // Interactive Color Handling
                let fill_color = self.color;
                if self.hover > 0.5 {
                    fill_color = mix(fill_color, #fff, 0.1);
                }
                if self.down > 0.5 {
                    fill_color = mix(fill_color, #000, 0.1);
                }
                
                sdf.fill(fill_color);
                sdf.stroke(self.border_color, self.border_width);
                return sdf.result
            }
        }
        
        // Default behavior configurations
        animator: {
            hover = {
                default: off
                off = {
                    from: {all: Forward {duration: 0.1}}
                    apply: {
                        draw_bg: {hover: 0.0, down: 0.0}
                    }
                }
                on = {
                    from: {all: Forward {duration: 0.1}}
                    apply: {
                        draw_bg: {
                            // Hover effect approximation
                            hover: 1.0, 
                            down: 0.0,
                        }
                    }
                }
                down = {
                    from: {all: Forward {duration: 0.1}}
                    apply: {
                         draw_bg: {hover: 1.0, down: 1.0}
                    }
                }
            }
        }
    }
}

#[derive(Live, LiveHook, Widget)]
pub struct Button {
    #[redraw] #[live] draw_bg: DrawQuad,
    #[live] draw_text: DrawText,
    #[live] layout: Layout,
    #[walk] walk: Walk,
    
    #[animator] animator: Animator,
    
    #[live] text: ArcStringMut,
}

impl Widget for Button {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, _scope: &mut Scope) {
        self.animator_handle_event(cx, event);
        
        match event.hits(cx, self.draw_bg.area()) {
            Hit::FingerDown(_fe) => {
                self.animator_play(cx, ids!(hover.down));
            },
            Hit::FingerHoverIn(_) => {
                cx.set_cursor(MouseCursor::Hand);
                self.animator_play(cx, ids!(hover.on));
            }
            Hit::FingerHoverOut(_) => {
                self.animator_play(cx, ids!(hover.off));
            }
            Hit::FingerUp(fe) => {
                if fe.is_over {
                    if fe.was_tap() {
                        // Clicked!
                        // For now just logging or doing nothing, later emit action
                        // log!("Button Clicked: {}", self.text.as_ref());
                        self.animator_play(cx, ids!(hover.on));
                    }
                } else {
                    self.animator_play(cx, ids!(hover.off));
                }
            }
            _ => ()
        }
    }
    
    fn draw_walk(&mut self, cx: &mut Cx2d, _scope: &mut Scope, walk: Walk) -> DrawStep {
        self.draw_bg.begin(cx, walk, self.layout);
        self.draw_text.draw_walk(cx, self.walk, Align::default(), self.text.as_ref());
        self.draw_bg.end(cx);
        DrawStep::done()
    }
}
