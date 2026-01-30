use makepad_widgets::*;
use crate::widgets::button::Button;

live_design! {
    use makepad_widgets::base::*;
    use makepad_widgets::theme_desktop_dark::*;
    use makepad_draw::shader::std::*;
    use makepad_widgets::view_ui::View;
    use makepad_widgets::scroll_bars::ScrollBars;

    // --- Style Templates ---

    pub TooltipStyle = <View> {
        width: 300.0, height: Fit
        flow: Down
        spacing: 15.0
        padding: 20.0
        show_bg: true
        draw_bg: {
            color: #333
            instance radius: 6.0
            fn pixel(self) -> vec4 {
                let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                sdf.box(0., 0., self.rect_size.x, self.rect_size.y, max(1.0, self.radius));
                sdf.fill_keep(self.color);
                return sdf.result
            }
        }
        // Scroll support
        scroll_bars: <ScrollBars> {
            show_scroll_y: true
            scroll_bar_y: { drag_scrolling: true }
        }
    }

    pub DialogStyle = <View> {
        width: 400.0, height: Fit
        flow: Down
        spacing: 20.0
        padding: 24.0
        show_bg: true
        draw_bg: {
            color: #2a2a2a
            instance radius: 8.0
            fn pixel(self) -> vec4 {
                let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                sdf.box(0., 0., self.rect_size.x, self.rect_size.y, max(1.0, self.radius));
                sdf.fill_keep(self.color);
                return sdf.result
            }
        }
        // Scroll support
        scroll_bars: <ScrollBars> {
            show_scroll_y: true
            scroll_bar_y: { drag_scrolling: true }
        }
    }

    pub SidePanelStyle = <View> {
        width: 350.0, height: Fill
        flow: Down
        padding: 16.0
        show_bg: true
        draw_bg: { color: #232323 }
    }

    // --- Main Modal Widget ---

    pub Modal = {{Modal}} {
        width: Fill, height: Fill
        flow: Overlay
        visible: false

        align: {x: 0.5, y: 0.5}

        show_bg: true
        draw_bg: {
            fn pixel(self) -> vec4 {
                return vec4(0.0, 0.0, 0.0, 0.8)
            }
        }
    }

    // --- TooltipTrigger (for RMB / LongPress) ---

    pub TooltipTrigger = {{TooltipTrigger}} {
        width: Fit, height: Fit
        flow: Down

        draw_underline: {
            instance hover: 0.0

            fn pixel(self) -> vec4 {
                // Dotted pattern: 2px dot + 2px gap
                let phase = mod(self.pos.x * self.rect_size.x, 4.0);
                if phase < 2.0 {
                    return mix(#666, #999, self.hover);
                }
                return vec4(0.0, 0.0, 0.0, 0.0);
            }
        }
    }
}

#[derive(Clone, DefaultNone, Debug)]
pub enum ModalAction {
    None,
    Dismissed,
    Accepted,
}

#[derive(Clone, DefaultNone, Debug)]
pub enum TooltipTriggerAction {
    None,
    ShowTooltip,
}

// --- TooltipTrigger ---

#[derive(Live, LiveHook, Widget)]
pub struct TooltipTrigger {
    #[live] draw_underline: DrawQuad,
    #[deref] view: View,
    #[rust] is_hovered: bool,
}

impl TooltipTrigger {
    fn contains_pos(&self, cx: &Cx, pos: DVec2) -> bool {
        self.view.area().rect(cx).contains(pos)
    }
}

impl Widget for TooltipTrigger {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        let uid = self.widget_uid();

        match event {
            Event::LongPress(lp) => {
                if self.contains_pos(cx, lp.abs) {
                    // Reset cursor when opening tooltip
                    self.is_hovered = false;
                    cx.set_cursor(MouseCursor::Arrow);
                    self.view.redraw(cx);
                    cx.widget_action(uid, &scope.path, TooltipTriggerAction::ShowTooltip);
                }
            }
            Event::MouseUp(mu) => {
                if mu.button.is_secondary() && self.contains_pos(cx, mu.abs) {
                    // Reset cursor when opening tooltip
                    self.is_hovered = false;
                    cx.set_cursor(MouseCursor::Arrow);
                    self.view.redraw(cx);
                    cx.widget_action(uid, &scope.path, TooltipTriggerAction::ShowTooltip);
                }
            }
            Event::MouseMove(mm) => {
                let is_over = self.contains_pos(cx, mm.abs);
                if is_over && !self.is_hovered {
                    self.is_hovered = true;
                    cx.set_cursor(MouseCursor::Help);
                    self.view.redraw(cx);
                } else if !is_over && self.is_hovered {
                    self.is_hovered = false;
                    cx.set_cursor(MouseCursor::Arrow);
                    self.view.redraw(cx);
                }
            }
            _ => ()
        }

        self.view.handle_event(cx, event, scope);
    }

    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        // Begin container
        cx.begin_turtle(walk, Layout::flow_down());

        // Draw view content
        let _ = self.view.draw_walk(cx, scope, Walk::fit());

        // Get the used rect (content bounds)
        let used = cx.turtle().used();

        // Draw underline at bottom of content
        let hover_val: f64 = if self.is_hovered { 1.0 } else { 0.0 };
        self.draw_underline.apply_over(cx, live!{ hover: (hover_val) });

        let underline_rect = Rect {
            pos: dvec2(cx.turtle().pos().x, cx.turtle().pos().y),
            size: dvec2(used.x, 1.0),
        };
        self.draw_underline.draw_abs(cx, underline_rect);

        // Walk past the underline
        cx.walk_turtle(Walk::fixed(0.0, 1.0));

        cx.end_turtle();
        DrawStep::done()
    }
}

// --- Main Modal ---

#[derive(Live, LiveHook, Widget)]
pub struct Modal {
    #[deref] view: View,
    #[rust] is_open: bool,
}

impl Modal {
    pub fn is_open(&self) -> bool {
        self.is_open
    }

    pub fn open(&mut self, cx: &mut Cx) {
        self.is_open = true;
        self.visible = true;
        self.view.redraw(cx);
    }

    pub fn close(&mut self, cx: &mut Cx) {
        self.is_open = false;
        self.visible = false;
        self.view.redraw(cx);
    }

    /// Recursively search for a button by id in content
    fn find_button_clicked(&self, content: &WidgetRef, button_id: &[LiveId], actions: &Actions) -> bool {
        // Try direct child
        let btn = content.widget(button_id);
        if let Some(b) = btn.borrow::<Button>() {
            if b.clicked(actions) {
                return true;
            }
        }

        // Try nested in common wrapper patterns
        for wrapper_id in [ids!(buttons_wrap), ids!(footer), ids!(actions)] {
            let wrapper = content.widget(wrapper_id);
            if wrapper.area() != Area::Empty {
                let btn = wrapper.widget(button_id);
                if let Some(b) = btn.borrow::<Button>() {
                    if b.clicked(actions) {
                        return true;
                    }
                }
            }
        }

        false
    }
}

impl Widget for Modal {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        if !self.is_open {
            return;
        }

        let uid = self.widget_uid();

        self.view.handle_event(cx, event, scope);

        // Handle button clicks in content
        if let Event::Actions(actions) = event {
            let content = self.view.widget(ids!(content));

            // Check ok_button → Accepted
            if self.find_button_clicked(&content, ids!(ok_button), actions) {
                self.close(cx);
                cx.widget_action(uid, &scope.path, ModalAction::Accepted);
                return;
            }

            // Check cancel_button → Dismissed
            if self.find_button_clicked(&content, ids!(cancel_button), actions) {
                self.close(cx);
                cx.widget_action(uid, &scope.path, ModalAction::Dismissed);
                return;
            }
        }

        // Escape key
        if let Event::KeyDown(ke) = event {
            if ke.key_code == KeyCode::Escape {
                self.close(cx);
                cx.widget_action(uid, &scope.path, ModalAction::Dismissed);
                return;
            }
        }

        // Click outside content
        match event.hits(cx, self.view.area()) {
            Hit::FingerUp(fe) => {
                let content = self.view.widget(ids!(content));
                if content.area() != Area::Empty {
                    if !content.area().rect(cx).contains(fe.abs) {
                        self.close(cx);
                        cx.widget_action(uid, &scope.path, ModalAction::Dismissed);
                    }
                }
            }
            _ => ()
        }
    }

    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        self.view.draw_walk(cx, scope, walk)
    }
}
