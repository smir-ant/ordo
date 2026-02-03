use makepad_widgets::*;

live_design! {
    use makepad_widgets::base::*;
    use makepad_widgets::theme_desktop_dark::*;
    use makepad_draw::shader::std::*;
    use crate::theme::*;

    pub ToggleIconButtonBase = {{ToggleIconButton}} {}

    pub ToggleIconButton = <ToggleIconButtonBase> {
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
            uniform color_active: #fff

            fn get_color(self) -> vec4 {
                return mix(
                    mix(self.color, self.color_hover, self.hover),
                    self.color_down,
                    self.down
                );
            }
        }

        draw_icon_active: {
            instance hover: 0.0
            instance down: 0.0

            uniform color: #fff
            uniform color_hover: #fff
            uniform color_down: #bbb

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
                        draw_icon_active: {hover: 0.0, down: 0.0}
                    }
                }
                on = {
                    from: {all: Forward {duration: 0.1}}
                    apply: {
                        draw_bg: {hover: 1.0, down: 0.0}
                        draw_icon: {hover: 1.0, down: 0.0}
                        draw_icon_active: {hover: 1.0, down: 0.0}
                    }
                }
                down = {
                    from: {all: Forward {duration: 0.05}}
                    apply: {
                        draw_bg: {hover: 1.0, down: 1.0}
                        draw_icon: {hover: 1.0, down: 1.0}
                        draw_icon_active: {hover: 1.0, down: 1.0}
                    }
                }
            }
        }
    }
}

#[derive(Clone, Debug, DefaultNone)]
pub enum ToggleIconButtonAction {
    None,
    Clicked,
    Activated,
    Deactivated,
}

#[derive(Live, LiveHook, Widget)]
pub struct ToggleIconButton {
    #[animator]
    animator: Animator,

    #[redraw]
    #[live]
    draw_bg: DrawQuad,
    #[live]
    draw_icon: DrawIcon,
    #[live]
    draw_icon_active: DrawIcon,
    #[live]
    icon_walk: Walk,
    #[walk]
    walk: Walk,
    #[layout]
    layout: Layout,

    #[rust]
    activated: bool,
}

impl Widget for ToggleIconButton {
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
                    // Only toggle if active icon is specified
                    if self.has_active_icon() {
                        self.activated = !self.activated;
                        let action = if self.activated {
                            ToggleIconButtonAction::Activated
                        } else {
                            ToggleIconButtonAction::Deactivated
                        };
                        cx.widget_action(uid, &scope.path, action);
                    } else {
                        cx.widget_action(uid, &scope.path, ToggleIconButtonAction::Clicked);
                    }
                    self.draw_bg.redraw(cx);
                }
                self.animator_play(cx, ids!(hover.off));
            }
            _ => ()
        }
    }

    fn draw_walk(&mut self, cx: &mut Cx2d, _scope: &mut Scope, walk: Walk) -> DrawStep {
        self.draw_bg.begin(cx, walk, self.layout);
        if self.activated && self.has_active_icon() {
            self.draw_icon_active.draw_walk(cx, self.icon_walk);
        } else {
            self.draw_icon.draw_walk(cx, self.icon_walk);
        }
        self.draw_bg.end(cx);
        DrawStep::done()
    }
}

impl ToggleIconButton {
    fn has_active_icon(&self) -> bool {
        // Check if active icon svg_file is set
        !self.draw_icon_active.svg_file.as_ref().is_empty()
    }

    pub fn is_activated(&self) -> bool {
        self.activated
    }

    pub fn activate(&mut self, cx: &mut Cx) {
        if !self.activated {
            self.activated = true;
            self.draw_bg.redraw(cx);
        }
    }

    pub fn deactivate(&mut self, cx: &mut Cx) {
        if self.activated {
            self.activated = false;
            self.draw_bg.redraw(cx);
        }
    }

    pub fn toggle(&mut self, cx: &mut Cx) {
        self.activated = !self.activated;
        self.draw_bg.redraw(cx);
    }

    pub fn set_activated(&mut self, cx: &mut Cx, active: bool) {
        if self.activated != active {
            self.activated = active;
            self.draw_bg.redraw(cx);
        }
    }

    pub fn clicked(&self, actions: &Actions) -> bool {
        matches!(
            actions.find_widget_action(self.widget_uid()).cast(),
            ToggleIconButtonAction::Clicked
        )
    }

    pub fn toggled(&self, actions: &Actions) -> Option<bool> {
        match actions.find_widget_action(self.widget_uid()).cast() {
            ToggleIconButtonAction::Activated => Some(true),
            ToggleIconButtonAction::Deactivated => Some(false),
            _ => None,
        }
    }
}

impl ToggleIconButtonRef {
    pub fn clicked(&self, actions: &Actions) -> bool {
        self.borrow().is_some_and(|inner| inner.clicked(actions))
    }

    pub fn toggled(&self, actions: &Actions) -> Option<bool> {
        self.borrow().and_then(|inner| inner.toggled(actions))
    }

    pub fn is_activated(&self) -> bool {
        self.borrow().is_some_and(|inner| inner.is_activated())
    }

    pub fn activate(&self, cx: &mut Cx) {
        if let Some(mut inner) = self.borrow_mut() {
            inner.activate(cx);
        }
    }

    pub fn deactivate(&self, cx: &mut Cx) {
        if let Some(mut inner) = self.borrow_mut() {
            inner.deactivate(cx);
        }
    }

    pub fn toggle(&self, cx: &mut Cx) {
        if let Some(mut inner) = self.borrow_mut() {
            inner.toggle(cx);
        }
    }

    pub fn set_activated(&self, cx: &mut Cx, active: bool) {
        if let Some(mut inner) = self.borrow_mut() {
            inner.set_activated(cx, active);
        }
    }
}
