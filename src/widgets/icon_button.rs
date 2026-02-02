use makepad_widgets::*;

live_design! {
    use makepad_widgets::base::*;
    use makepad_widgets::theme_desktop_dark::*;
    use makepad_draw::shader::std::*;
    use crate::theme::*;

    pub IconButtonBase = {{IconButton}} {}

    pub IconButton = <IconButtonBase> {
        width: Fit, height: Fit
        align: {x: 0.5, y: 0.5}
        padding: 8.0

        icon_walk: {
            width: 24.0, height: 24.0
        }

        draw_bg: {
            instance hover: 0.0
            instance down: 0.0

            fn pixel(self) -> vec4 {
                let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                let radius = min(self.rect_size.x, self.rect_size.y) * 0.5;

                // Circle background on hover/down
                sdf.circle(self.rect_size.x * 0.5, self.rect_size.y * 0.5, radius);

                let hover_color = mix(#0000, #ffffff10, self.hover);
                let down_color = mix(hover_color, #ffffff20, self.down);

                sdf.fill(down_color);
                return sdf.result;
            }
        }

        draw_icon: {
            instance hover: 0.0
            instance down: 0.0

            uniform color: #888
            uniform color_hover: #bbb
            uniform color_down: #fff

            fn get_color(self) -> vec4 {
                return mix(
                    mix(self.color, self.color_hover, self.hover),
                    self.color_down,
                    self.down
                );
            }
        }

        animator: {
            hover = {
                default: off,
                off = {
                    from: {all: Forward {duration: 0.15}}
                    apply: {
                        draw_bg: {hover: 0.0, down: 0.0}
                        draw_icon: {hover: 0.0, down: 0.0}
                    }
                }
                on = {
                    from: {all: Forward {duration: 0.1}}
                    apply: {
                        draw_bg: {hover: 1.0, down: 0.0}
                        draw_icon: {hover: 1.0, down: 0.0}
                    }
                }
                down = {
                    from: {all: Forward {duration: 0.05}}
                    apply: {
                        draw_bg: {hover: 1.0, down: 1.0}
                        draw_icon: {hover: 1.0, down: 1.0}
                    }
                }
            }
        }
    }
}

#[derive(Clone, Debug, DefaultNone)]
pub enum IconButtonAction {
    None,
    Clicked,
}

#[derive(Live, LiveHook, Widget)]
pub struct IconButton {
    #[animator]
    animator: Animator,

    #[redraw]
    #[live]
    draw_bg: DrawQuad,
    #[live]
    draw_icon: DrawIcon,
    #[live]
    icon_walk: Walk,
    #[walk]
    walk: Walk,
    #[layout]
    layout: Layout,
}

impl Widget for IconButton {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        let uid = self.widget_uid();

        if self.animator_handle_event(cx, event).must_redraw() {
            self.draw_bg.redraw(cx);
        }

        match event.hits(cx, self.draw_bg.area()) {
            Hit::FingerDown(_fe) => {
                self.animator_play(cx, ids!(hover.down));
            }
            Hit::FingerHoverIn(_) => {
                cx.set_cursor(MouseCursor::Hand);
                self.animator_play(cx, ids!(hover.on));
            }
            Hit::FingerHoverOut(_) => {
                cx.set_cursor(MouseCursor::Arrow);
                self.animator_play(cx, ids!(hover.off));
            }
            Hit::FingerUp(fe) => {
                if fe.is_over {
                    cx.widget_action(uid, &scope.path, IconButtonAction::Clicked);
                }
                self.animator_play(cx, ids!(hover.off));
            }
            _ => ()
        }
    }

    fn draw_walk(&mut self, cx: &mut Cx2d, _scope: &mut Scope, walk: Walk) -> DrawStep {
        self.draw_bg.begin(cx, walk, self.layout);
        self.draw_icon.draw_walk(cx, self.icon_walk);
        self.draw_bg.end(cx);
        DrawStep::done()
    }
}

impl IconButton {
    pub fn clicked(&self, actions: &Actions) -> bool {
        matches!(
            actions.find_widget_action(self.widget_uid()).cast(),
            IconButtonAction::Clicked
        )
    }
}

impl IconButtonRef {
    pub fn clicked(&self, actions: &Actions) -> bool {
        self.borrow().is_some_and(|inner| inner.clicked(actions))
    }
}
