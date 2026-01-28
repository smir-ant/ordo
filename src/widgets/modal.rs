use makepad_widgets::*;
use crate::widgets::button::Button;

live_design! {
    use makepad_widgets::base::*;
    use makepad_widgets::theme_desktop_dark::*;
    use makepad_widgets::scroll_bars::ScrollBars;
    use makepad_draw::shader::std::*;
    use makepad_widgets::view_ui::View; 

    use crate::widgets::button::Btn;
    use crate::widgets::text::Text;

    // --- Content Wrappers ---

    pub TooltipContent = {{TooltipContent}} {
        width: 300.0, height: Fit
        flow: Down
        spacing: 15.0
        padding: 15.0
        
        show_bg: true
        draw_bg: {
            color: #333
            instance radius: 6.0
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
        
        title = <Text> {
            text: "Info"
            draw_text: {
                color: #fff
                text_style: <THEME_FONT_BOLD> { font_size: 14.0 }
            }
        }
        
        text = <Text> {
            width: Fill, height: Fit
            text: "Tooltip text goes here."
            draw_text: {
                color: #ccc
                text_style: <THEME_FONT_REGULAR> { font_size: 12.0 }
            }
        }
        
        ok_button = <Btn> {
            width: Fill
            text: "Got it"
            reset_hover_on_click: true
        }
    }

    pub DialogContent = {{DialogContent}} {
        width: 400.0, height: Fit
        flow: Down
        spacing: 20.0
        padding: 20.0
        
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
        
        title = <Text> {
            text: "Confirm Action"
            draw_text: {
                color: #fff
                text_style: <THEME_FONT_BOLD> { font_size: 16.0 }
            }
        }
        
        text = <Text> {
            width: Fill, height: Fit
            text: "Are you sure?"
            draw_text: {
                color: #bbb
                text_style: <THEME_FONT_REGULAR> { font_size: 14.0 }
            }
        }
        
        buttons_wrap = <View> {
            width: Fill, height: Fit
            flow: Right
            align: {x: 1.0} 
            spacing: 15.0
            
            cancel_button = <Btn> {
                width: 100.0
                text: "Cancel"
                draw_bg: { color: #444 }
            }
            
            ok_button = <Btn> {
                width: 100.0
                text: "OK"
                accent: true
            }
        }
    }

    pub SidePanelContent = {{SidePanelContent}} {
        width: 350.0, height: Fill
        flow: Down
        spacing: 0.0
        
        show_bg: true
        draw_bg: {
            color: #232323
        }

        header = <View> {
            width: Fill, height: Fit
            flow: Right
            padding: 20.0
            align: {y: 0.5}
            
            title = <Text> {
                text: "Side Panel"
                draw_text: {
                    color: #fff
                    text_style: <THEME_FONT_BOLD> { font_size: 18.0 }
                }
            }
        }

        // Scrollable content area
        body = <View> {
            width: Fill, height: Fill
            flow: Down
            scroll_bars: <ScrollBars> {}
            padding: 20.0
            spacing: 15.0
            
            // Content will be injected here by the user
        }
    }

    // --- Unified Modal Widget ---

    pub Modal = {{Modal}} {
        width: Fill, height: Fill
        flow: Overlay
        visible: false
        
        // Default alignment is center (for Dialogs/Tooltips)
        align: {x: 0.5, y: 0.5} 
        
        show_bg: true
        draw_bg: {
            fn pixel(self) -> vec4 {
                return vec4(0.0, 0.0, 0.0, 0.8) // Dark dimmed background
            }
        }
    }

    pub TooltipTrigger = {{TooltipTrigger}} {
        width: Fit, height: Fit
        flow: Down
        show_bg: true
        draw_bg: {
            color: #f0f // Pink default as requested context
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

// --- Content Wrappers Implementations ---

// --- Content Wrappers Implementations ---

#[derive(Live, LiveHook, Widget)]
pub struct TooltipTrigger {
    #[deref] view: View,
}

impl TooltipTrigger {
    fn contains_pos(&self, cx: &Cx, pos: DVec2) -> bool {
        self.view.area().rect(cx).contains(pos)
    }
}

impl Widget for TooltipTrigger {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        let uid = self.widget_uid();
        
        // Intercept triggers BEFORE passing to children, similar to Hint pattern
        match event {
            Event::LongPress(lp) => {
                 if self.contains_pos(cx, lp.abs) {
                     cx.widget_action(uid, &scope.path, TooltipTriggerAction::ShowTooltip);
                 }
            }
            Event::MouseUp(mu) => {
                // Secondary button (Right Click)
                if mu.button.is_secondary() && self.contains_pos(cx, mu.abs) {
                     cx.widget_action(uid, &scope.path, TooltipTriggerAction::ShowTooltip);
                }
            }
            _ => ()
        }
        
        // Forward generic events to children (like Hover, Click on inner buttons)
        self.view.handle_event(cx, event, scope);
    }

    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        self.view.draw_walk(cx, scope, walk)
    }
}

// --- Content Wrappers Implementations ---

#[derive(Live, LiveHook, Widget)]
pub struct TooltipContent {
    #[deref] view: View,
}

impl Widget for TooltipContent {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        let uid = self.widget_uid();
        self.view.handle_event(cx, event, scope);
        
        if let Event::Actions(actions) = event {
            if self.view.widget(ids!(ok_button)).borrow::<Button>().map(|b| b.clicked(actions)).unwrap_or(false) {
                log!("TooltipContent: 'Got it' clicked");
                cx.widget_action(uid, &scope.path, ModalAction::Accepted);
            }
        }
    }
    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        self.view.draw_walk(cx, scope, walk)
    }
}

#[derive(Live, LiveHook, Widget)]
pub struct DialogContent {
    #[deref] view: View,
}

impl Widget for DialogContent {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        let uid = self.widget_uid();
        self.view.handle_event(cx, event, scope);
        
        if let Event::Actions(actions) = event {
            if self.view.widget(ids!(buttons_wrap)).widget(ids!(ok_button)).borrow::<Button>().map(|b| b.clicked(actions)).unwrap_or(false) {
                log!("DialogContent: 'OK' clicked");
                cx.widget_action(uid, &scope.path, ModalAction::Accepted);
            }
            if self.view.widget(ids!(buttons_wrap)).widget(ids!(cancel_button)).borrow::<Button>().map(|b| b.clicked(actions)).unwrap_or(false) {
                log!("DialogContent: 'Cancel'");
                cx.widget_action(uid, &scope.path, ModalAction::Dismissed);
            }
        }
    }
    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        self.view.draw_walk(cx, scope, walk)
    }
}

// --- SidePanelContent Implementation ---
#[derive(Live, LiveHook, Widget)]
pub struct SidePanelContent {
    #[deref] view: View,
}

impl Widget for SidePanelContent {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        self.view.handle_event(cx, event, scope);
    }
    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        self.view.draw_walk(cx, scope, walk)
    }
}

// --- Main Modal Implementation ---

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
        log!("Modal::open called, is_open set to true");
        self.is_open = true;
        // Note: visible must be set via apply_over on WidgetRef from outside
        // Block scrolling everywhere except inside modal content
        let content = self.view.widget(ids!(content));
        if content.area() != Area::Empty {
            cx.block_scrolling_except_within(content.area());
        }
    }

    pub fn close(&mut self, cx: &mut Cx) {
        log!("Modal::close called");
        self.is_open = false;
        cx.unblock_scrolling();
        // Note: visible must be set via apply_over on WidgetRef from outside
    }

    pub fn toggle(&mut self, cx: &mut Cx) {
        if self.is_open {
            self.close(cx);
        } else {
            self.open(cx);
        }
    }
}

impl Widget for Modal {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        if !self.is_open {
            return;
        }

        let uid = self.widget_uid();

        // Forward events to content (child widgets)
        self.view.handle_event(cx, event, scope);

        // Check for Actions from children (Close/Dismiss)
        if let Event::Actions(actions) = event {
            for action in actions {
                if let ModalAction::Dismissed = action.as_widget_action().cast() {
                    self.close(cx);
                    cx.widget_action(uid, &scope.path, ModalAction::Dismissed);
                    return;
                }
                if let ModalAction::Accepted = action.as_widget_action().cast() {
                    self.close(cx);
                    cx.widget_action(uid, &scope.path, ModalAction::Accepted);
                    return;
                }
            }
        }

        // Handle Key Events (Escape)
        if let Event::KeyDown(ke) = event {
            if ke.key_code == KeyCode::Escape {
                self.close(cx);
                cx.widget_action(uid, &scope.path, ModalAction::Dismissed);
                return;
            }
        }

        // Handle click outside content (dismiss) - use FingerUp to prevent event leaking
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
        // Re-apply scroll blocking after redraw (area may have changed)
        if self.is_open {
            let content = self.view.widget(ids!(content));
            if content.area() != Area::Empty {
                cx.block_scrolling_except_within(content.area());
            }
        }
        self.view.draw_walk(cx, scope, walk)
    }
}
