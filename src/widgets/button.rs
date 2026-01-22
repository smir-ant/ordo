use makepad_widgets::*;
use makepad_widgets::widget::{WidgetDesignAction, WidgetActionData};
use makepad_widgets::touch_gesture::*;

live_design! {
    use makepad_widgets::base::*;
    use makepad_widgets::theme_desktop_dark::*;
    use makepad_draw::shader::std::*;
    use link::styling::*;
    
    pub BtnBase = {{Button}} {}
    
    pub BtnFlat = <BtnBase> {
        text: "Button"
        width: Fit, height: Fit,
        spacing: (THEME_SPACE_2),
        align: {x: 0.5, y: 0.5},
        padding: <THEME_MSPACE_1> { left: (THEME_SPACE_2), right: (THEME_SPACE_2) }
        margin: <THEME_MSPACE_V_1> {}
        label_walk: { width: Fit, height: Fit },

        draw_text: {
            instance hover: 0.0,
            instance down: 0.0,
            instance focus: 0.0,
            instance disabled: 0.0

            color: (THEME_COLOR_LABEL_INNER)
            uniform color_hover: (THEME_COLOR_LABEL_INNER_HOVER)
            uniform color_down: (THEME_COLOR_LABEL_INNER_DOWN)
            uniform color_focus: (THEME_COLOR_LABEL_INNER_FOCUS)
            color: (THEME_COLOR_LABEL_INNER)
            uniform color_hover: (THEME_COLOR_LABEL_INNER_HOVER)
            uniform color_down: (THEME_COLOR_LABEL_INNER_DOWN)
            uniform color_focus: (THEME_COLOR_LABEL_INNER_FOCUS)
            uniform color_disabled: #555  // Even duller text

            text_style: <THEME_FONT_REGULAR> {
                font_size: (THEME_FONT_SIZE_P)
            }
            fn get_color(self) -> vec4 {
                return mix(
                    mix(
                        mix(
                            mix(self.color, self.color_focus, self.focus),
                            self.color_hover,
                            self.hover
                        ),
                        self.color_down,
                        self.down
                    ),
                    self.color_disabled,
                    self.disabled
                )
            }
        }
        
        icon_walk: {
            width: (THEME_DATA_ICON_WIDTH), height: Fit,
        }
        
        draw_icon: {
            instance hover: 0.0
            instance down: 0.0
            instance focus: 0.0
            instance disabled: 0.0

            uniform color_dither: 1.0

            uniform gradient_fill_horizontal: 0.
            uniform color: (THEME_COLOR_LABEL_OUTER)
            uniform color_hover: (THEME_COLOR_LABEL_OUTER_HOVER)
            uniform color_down: (THEME_COLOR_LABEL_OUTER_DOWN)
            uniform color_focus: (THEME_COLOR_LABEL_OUTER_FOCUS)
            uniform color_disabled: (THEME_COLOR_LABEL_OUTER_DISABLED)

            uniform color_2: vec4(-1.0, -1.0, -1.0, -1.0)
            uniform color_2_hover: (THEME_COLOR_LABEL_OUTER_HOVER)
            uniform color_2_down: (THEME_COLOR_LABEL_OUTER_DOWN)
            uniform color_2_focus: (THEME_COLOR_LABEL_OUTER_FOCUS)
            uniform color_2_disabled: (THEME_COLOR_LABEL_OUTER_DISABLED)

            fn get_color(self) -> vec4 {
                let dither = Math::random_2d(self.pos.xy) * 0.04 * self.color_dither;

                let color_2 = self.color;
                let color_2_hover = self.color_hover;
                let color_2_down = self.color_down;
                let color_2_focus = self.color_focus;
                let color_2_disabled = self.color_disabled;

                if (self.color_2.x > -0.5) {
                    color_2 = self.color_2
                    color_2_hover = self.color_2_hover;
                    color_2_down = self.color_2_down;
                    color_2_focus = self.color_2_focus;
                    color_2_disabled = self.color_2_disabled;
                }

                let gradient_fill_dir = self.pos.y + dither;
                if (self.gradient_fill_horizontal > 0.5) {
                    gradient_fill_dir = self.pos.x + dither;
                }

                return mix(
                    mix(
                        mix(
                            mix(
                                mix(self.color, color_2, gradient_fill_dir),
                                mix(self.color_focus, color_2_focus, gradient_fill_dir),
                                self.focus
                            ),
                            mix(self.color_hover, color_2_hover, gradient_fill_dir),
                            self.hover
                        ),
                        mix(self.color_down, color_2_down, gradient_fill_dir),
                        self.down
                    ),
                    mix(self.color_disabled, color_2_disabled, gradient_fill_dir),
                    self.disabled
                )
            }
        }

        draw_bg: {
            instance hover: 0.0
            instance focus: 0.0
            instance down: 0.0
            instance disabled: 0.0
            instance accent: 0.0

            uniform border_size: (THEME_BEVELING)
            uniform border_radius: (THEME_CORNER_RADIUS)

            // Normal colors (DOW-style)
            uniform color: #444
            uniform color_hover: #505050
            uniform color_down: #383838
            uniform color_focus: #4a4a4a
            uniform color_disabled: #292929
            
            // Accent colors from styling
            uniform accent_color: (THEME_COLOR_ACCENT)
            uniform accent_color_light: (THEME_COLOR_ACCENT_LIGHT)
            uniform accent_color_dark: (THEME_COLOR_ACCENT_DARK)

            fn pixel(self) -> vec4 {
                let sdf = Sdf2d::viewport(self.pos * self.rect_size)
                
                // Normal base color (DOW-style)
                let normal_base = mix(
                    mix(
                        mix(
                            mix(self.color, self.color_focus, self.focus),
                            self.color_hover,
                            self.hover
                        ),
                        self.color_down,
                        self.down
                    ),
                    self.color_disabled,
                    self.disabled
                );
                
                // Accent base color with hover effect (like DOW active_hover)
                let accent_fill = mix(self.accent_color_light, self.accent_color, self.pos.y);
                let accent_hover_fill = mix(accent_fill, #fff, 0.2);  // 20% lighter on hover
                let accent_down_fill = self.accent_color * 0.8;  // Darken on down
                
                let accent_base = mix(
                    mix(accent_fill, accent_hover_fill, self.hover),
                    accent_down_fill,
                    self.down
                );
                
                // Mix between normal and accent based on accent property
                let base_color = mix(normal_base, accent_base, self.accent);
                let fill_color = mix(base_color * 1.05, base_color * 0.95, self.pos.y);
                
                // Normal stroke gradient
                let normal_stroke_top = mix(#666, #777, self.hover);
                let normal_stroke_bottom = mix(#222, #333, self.hover);
                let normal_stroke = mix(normal_stroke_top, normal_stroke_bottom, self.pos.y);
                
                // Accent stroke gradient (like DOW active)
                let accent_stroke = mix(self.accent_color_light, self.accent_color_dark, self.pos.y);
                
                // Mix strokes and apply disabled
                let stroke_color = mix(
                    mix(normal_stroke, accent_stroke, self.accent),
                    #292929,
                    self.disabled
                );

                sdf.box(
                    self.border_size,
                    self.border_size,
                    self.rect_size.x - self.border_size * 2.,
                    self.rect_size.y - self.border_size * 2.,
                    self.border_radius
                );

                sdf.fill_keep(fill_color);
                sdf.stroke(stroke_color, self.border_size);
                
                return sdf.result
            }
        }
        
        animator: {
            disabled = {
                default: off,
                off = {
                    from: {all: Forward {duration: 0.}}
                    apply: {
                        draw_bg: {disabled: 0.0}
                        draw_text: {disabled: 0.0}
                        draw_icon: {disabled: 0.0}
                    }
                }
                on = {
                    from: {all: Forward {duration: 0.2}}
                    apply: {
                        draw_bg: {disabled: 1.0}
                        draw_text: {disabled: 1.0}
                        draw_icon: {disabled: 1.0}
                    }
                }
            }
            time = {
                default: off,
                off = {
                    from: {all: Forward {duration: 0.}}
                    apply: {
                        //draw_bg: {anim_time: 0.0}
                    }
                }
                on = {
                    from: {all: Loop {duration: 1.0, end:1000000000.0}}
                    apply: {
                        draw_bg: {anim_time: [{time: 0.0, value: 0.0},{time:1.0, value:1.0}]}
                    }
                }
            }
            hover = {
                default: off,
                off = {
                    from: {all: Forward {duration: 0.1}}
                    apply: {
                        draw_bg: {down: 0.0, hover: 0.0}
                        draw_icon: {down: 0.0, hover: 0.0}
                        draw_text: {down: 0.0, hover: 0.0}
                    }
                }
                
                on = {
                    from: {
                        all: Forward {duration: 0.1}
                        down: Forward {duration: 0.01}
                    }
                    apply: {
                        draw_bg: {down: 0.0, hover: [{time: 0.0, value: 1.0}],}
                        draw_icon: {down: 0.0, hover: [{time: 0.0, value: 1.0}],}
                        draw_text: {down: 0.0, hover: [{time: 0.0, value: 1.0}],}
                    }
                }
                
                down = {
                    from: {all: Forward {duration: 0.2}}
                    apply: {
                        draw_bg: {down: [{time: 0.0, value: 1.0}], hover: 1.0,}
                        draw_icon: {down: [{time: 0.0, value: 1.0}], hover: 1.0,}
                        draw_text: {down: [{time: 0.0, value: 1.0}], hover: 1.0,}
                    }
                }
            }
            focus = {
                default: off
                off = {
                    from: {all: Forward {duration: 0.2}}
                    apply: {
                        draw_bg: {focus: 0.0}
                        draw_icon: {focus: 0.0}
                        draw_text: {focus: 0.0}
                    }
                }
                on = {
                    cursor: Arrow,
                    from: {all: Forward {duration: 0.0}}
                    apply: {
                        draw_bg: {focus: 1.0}
                        draw_icon: {focus: 1.0}
                        draw_text: {focus: 1.0}
                    }
                }
            }
        }
    }

    pub BtnFlatter = <BtnFlat> {
        draw_bg: {
            color: (THEME_COLOR_U_HIDDEN)
            color_hover: (THEME_COLOR_U_HIDDEN)
            color_down: (THEME_COLOR_U_HIDDEN)
            color_disabled: (THEME_COLOR_OUTSET_DISABLED)
        }
    }

    pub Btn = <BtnFlat> {}

 
    // BtnGradientX and BtnGradientY removed - gradients not supported in simplified shader
    // Use Btn instead

  
    pub BtnIcon = <Btn> {
        spacing: 0.
        text: ""
    }
    
    // BtnGradientXIcon and BtnGradientYIcon removed - use BtnIcon instead

    
    pub BtnFlatIcon = <BtnFlat> {
        spacing: 0.
        text: ""
    }
    
    pub BtnFlatterIcon = <BtnFlatter> {
        draw_bg: { color_focus: (THEME_COLOR_U_HIDDEN)}        
        spacing: 0.
        text: ""
    }
    
}

/// Actions emitted by a button widget, including the key modifiers
/// that were active when the action occurred.
///
/// The sequence of actions emitted by a button is as follows:
/// 1. `ButtonAction::Pressed` when the button is pressed.
/// 2. `ButtonAction::LongPressed` when the button has been pressed for a long time.
///    * This only occurs on platforms that support a *native* long press, e.g., mobile.
/// 3. Then, either one of the following, but not both:
///    * `ButtonAction::Clicked` when the mouse/finger is lifted up while over the button area.
///    * `ButtonAction::Released` when the mouse/finger is lifted up while *not* over the button area.
#[derive(Clone, Debug, DefaultNone)]
pub enum ButtonAction {
    None,
    /// The button was pressed (a "down" event).
    Pressed(KeyModifiers),
    /// The button was pressed for a long time (only occurs on mobile platforms).
    LongPressed,
    /// The button was clicked (an "up" event).
    Clicked(KeyModifiers),
    /// The button was released (an "up" event), but should not be considered clicked
    /// because the mouse/finger was not over the button area when released.
    Released(KeyModifiers),
    Scroll(Vec2d),
}

/// A clickable button widget that emits actions when pressed, and when either released or clicked.
#[derive(Live, Widget)]
pub struct Button {
    #[animator]
    animator: Animator,

    #[redraw]
    #[live]
    draw_bg: DrawQuad,
    #[live]
    draw_text: DrawText,
    #[live]
    draw_icon: DrawIcon,
    #[live]
    icon_walk: Walk,
    #[live]
    label_walk: Walk,
    #[walk]
    walk: Walk,

    #[layout]
    layout: Layout,

    #[live(true)]
    grab_key_focus: bool,

    #[live(true)]
    enabled: bool,

    #[live(false)]
    accent: bool,

    #[live(true)]
    #[visible] visible: bool,

    /// Set the long-press handling behavior of this button.
    /// * If `false` (default), the button will ignore long-press events
    ///   and will never emit [`ButtonAction::LongPressed`].
    ///   * Also, the button logic will *not* call [`FingerUpEvent::was_tap()`]
    ///     to check if the button press was a short tap.
    ///     This means that this button will consider itself to be clicked
    ///     (and thus emit a [`ButtonAction::Clicked`] event)
    ///     if the finger-up/release event occurs within the button area,
    ///     *regardless* of how long the button was pressed down before it was released.
    /// * If `true`, the button will respond to a long-press event
    ///   by emitting [`ButtonAction::LongPressed`], which can only occur on
    ///   mobile platforms that support a *native* long press event.
    ///   * Also, the button will only consider itself to be clicked
    ///     (and thus emit [`ButtonAction::Clicked`]) if [`FingerUpEvent::was_tap()`] returns `true`,
    ///     meaning that a long press did *not* occur and that the button was released over the button area
    ///     within a short time frame (~0.5 seconds) after the initial down press.
    #[live]
    pub enable_long_press: bool,

    /// It indicates if the hover state will be reset when the button is clicked.
    /// This could be useful for buttons that disappear when clicked, where the hover state
    /// should not be preserved.
    #[live]
    reset_hover_on_click: bool,

    #[live]
    pub text: ArcStringMut,
    
    #[action_data] #[rust] action_data: WidgetActionData,
    
    #[rust] touch_gesture: TouchGesture,
    #[rust] last_scroll_pos: f64,
}

impl LiveHook for Button{
    fn after_new_from_doc(&mut self, _cx:&mut Cx){
        self.touch_gesture = TouchGesture::new();
    }
}

impl Widget for Button {
    fn set_disabled(&mut self, cx:&mut Cx, disabled:bool){
        self.animator_toggle(cx, disabled, Animate::Yes, ids!(disabled.on), ids!(disabled.off));
    }
                
    fn disabled(&self, cx:&Cx) -> bool {
        self.animator_in_state(cx, ids!(disabled.on))
    }

    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        let uid = self.widget_uid();
        
        if self.touch_gesture.handle_event(cx, event, self.draw_bg.area()).has_changed() {
            let new_pos = self.touch_gesture.scrolled_at;
            let delta = self.last_scroll_pos - new_pos; // Reversed direction
            self.last_scroll_pos = new_pos;
            if delta.abs() > 1e-5 {
                 cx.widget_action_with_data(&self.action_data, uid, &scope.path, ButtonAction::Scroll(DVec2{x:0.0, y: delta}));
            }
        }
    
        if self.animator_handle_event(cx, event).must_redraw() {
            self.draw_bg.redraw(cx);
        }
        
        match event.hit_designer(cx, self.draw_bg.area()){
            HitDesigner::DesignerPick(_e)=>{
                cx.widget_action_with_data(&self.action_data, uid, &scope.path, WidgetDesignAction::PickedBody)
            }
            _=>()
        }
        
        
        // The button only handles hits when it's visible and enabled.
        // If it's not enabled, we still show the button, but we set
        // the NotAllowed mouse cursor upon hover instead of the Hand cursor.
        match event.hits(cx, self.draw_bg.area()) {
            Hit::KeyFocus(_) => {
                self.animator_play(cx, ids!(focus.on));
            }
            Hit::KeyFocusLost(_) => {
                self.animator_play(cx, ids!(focus.off));
                self.draw_bg.redraw(cx);
            }
            Hit::FingerDown(fe) if self.enabled && fe.is_primary_hit() => {
                if self.grab_key_focus {
                    cx.set_key_focus(self.draw_bg.area());
                }
                cx.widget_action_with_data(&self.action_data, uid, &scope.path, ButtonAction::Pressed(fe.modifiers));
                self.animator_play(cx, ids!(hover.down));
                self.set_key_focus(cx);
                
                self.touch_gesture.set_mode(ScrollMode::Swipe);
                self.last_scroll_pos = self.touch_gesture.scrolled_at;
            }
            Hit::FingerHoverIn(_) => {
                if self.enabled {
                    cx.set_cursor(MouseCursor::Hand);
                    self.animator_play(cx, ids!(hover.on));
                } else {
                    cx.set_cursor(MouseCursor::NotAllowed);
                }
            }
            Hit::FingerHoverOut(_) => {
                self.animator_play(cx, ids!(hover.off));
            }
            Hit::FingerMove(fe) => {
                 // Scroll handled by TouchGesture. 
                 // We don't need manual logic here.
            }
            Hit::FingerLongPress(_lp) if self.enabled && self.enable_long_press => {
                cx.widget_action_with_data(&self.action_data, uid, &scope.path, ButtonAction::LongPressed);
            }
            Hit::FingerUp(fe) if self.enabled && fe.is_primary_hit() => {
                 if fe.was_tap() {
                    let was_clicked = fe.is_over;
                    if was_clicked {
                        cx.widget_action_with_data(&self.action_data, uid, &scope.path, ButtonAction::Clicked(fe.modifiers));
                        if self.reset_hover_on_click {
                            self.animator_cut(cx, ids!(hover.off));
                        } else if fe.has_hovers() {
                            self.animator_play(cx, ids!(hover.on));
                        } else {
                            self.animator_play(cx, ids!(hover.off));
                        }
                    } else {
                        cx.widget_action_with_data(&self.action_data, uid, &scope.path, ButtonAction::Released(fe.modifiers));
                        self.animator_play(cx, ids!(hover.off));
                    }
                } else {
                    cx.widget_action_with_data(&self.action_data, uid, &scope.path, ButtonAction::Released(fe.modifiers));
                    self.animator_play(cx, ids!(hover.off));
                }
            }
            _ => (),
        }
    }

    fn draw_walk(&mut self, cx: &mut Cx2d, _scope: &mut Scope, walk: Walk) -> DrawStep {
        if !self.visible {
            return DrawStep::done();
        }

        self.draw_bg.apply_over(cx, live!{accent: (if self.accent { 1.0 } else { 0.0 })});
        self.draw_bg.begin(cx, walk, self.layout);
        if self.enabled {
             self.animator_cut(cx, ids!(disabled.off));
        }
        else {
             self.animator_cut(cx, ids!(disabled.on));
        }
        self.draw_icon.draw_walk(cx, self.icon_walk);
        self.draw_text
            .draw_walk(cx, self.label_walk, Align::default(), self.text.as_ref());
        self.draw_bg.end(cx);
        cx.add_nav_stop(self.draw_bg.area(), NavRole::TextInput, Margin::default());
        DrawStep::done()
    }

    fn text(&self) -> String {
        self.text.as_ref().to_string()
    }

    fn set_text(&mut self, cx:&mut Cx, v: &str) {
        self.text.as_mut_empty().push_str(v);
        self.redraw(cx);
    }
}

impl Button {
        
    pub fn draw_button(&mut self, cx: &mut Cx2d, label:&str) {
        self.draw_bg.begin(cx, self.walk, self.layout);
        self.draw_icon.draw_walk(cx, self.icon_walk);
        self.draw_text
            .draw_walk(cx, self.label_walk, Align::default(), label);
        self.draw_bg.end(cx);
    }
    
    /// Returns `true` if this button was clicked.
    ///
    /// See [`ButtonAction`] for more details.
    pub fn clicked(&self, actions: &Actions) -> bool {
        self.clicked_modifiers(actions).is_some()
    }

    /// Returns `true` if this button was pressed down.
    ///
    /// See [`ButtonAction`] for more details.
    pub fn pressed(&self, actions: &Actions) -> bool {
        self.pressed_modifiers(actions).is_some()
    }

    /// Returns `true` if this button was long-pressed on.
    ///
    /// Note that this does not mean the button has been released yet.
    /// See [`ButtonAction`] for more details.
    pub fn long_pressed(&self, actions: &Actions) -> bool {
        matches!(
            actions.find_widget_action(self.widget_uid()).cast_ref(),
            ButtonAction::LongPressed,
        )
    }

    /// Returns `true` if this button was released, which is *not* considered to be clicked.
    ///
    /// See [`ButtonAction`] for more details.
    pub fn released(&self, actions: &Actions) -> bool {
        self.released_modifiers(actions).is_some()
    }

    /// Returns `Some` (with active keyboard modifiers) if this button was clicked.
    ///
    /// See [`ButtonAction`] for more details.
    pub fn clicked_modifiers(&self, actions: &Actions) -> Option<KeyModifiers> {
        if let ButtonAction::Clicked(m) = actions.find_widget_action(self.widget_uid()).cast_ref() {
            Some(*m)
        } else {
            None
        }
    }

    /// Returns `Some` (with active keyboard modifiers) if this button was pressed down.
    ///
    /// See [`ButtonAction`] for more details.
    pub fn pressed_modifiers(&self, actions: &Actions) -> Option<KeyModifiers> {
        if let ButtonAction::Pressed(m) = actions.find_widget_action(self.widget_uid()).cast_ref() {
            Some(*m)
        } else {
            None
        }
    }

    /// Returns `Some` (with active keyboard modifiers) if this button was released,
    /// which is *not* considered to be clicked.
    ///
    /// See [`ButtonAction`] for more details.
    pub fn released_modifiers(&self, actions: &Actions) -> Option<KeyModifiers> {
        if let ButtonAction::Released(m) = actions.find_widget_action(self.widget_uid()).cast_ref() {
            Some(*m)
        } else {
            None
        }
    }
}

impl ButtonRef {
    /// See [`Button::clicked()`].
    pub fn clicked(&self, actions: &Actions) -> bool {
        self.borrow().is_some_and(|inner| inner.clicked(actions))
    }

    /// See [`Button::pressed()`].
    pub fn pressed(&self, actions: &Actions) -> bool {
        self.borrow().is_some_and(|inner| inner.pressed(actions))
    }

    /// See [`Button::long_pressed()`].
    pub fn long_pressed(&self, actions: &Actions) -> bool {
        self.borrow().is_some_and(|inner| inner.long_pressed(actions))
    }

    /// See [`Button::released()`].
    pub fn released(&self, actions: &Actions) -> bool {
        self.borrow().is_some_and(|inner| inner.released(actions))
    }

    /// See [`Button::clicked_modifiers()`].
    pub fn clicked_modifiers(&self, actions: &Actions) -> Option<KeyModifiers> {
        self.borrow().and_then(|inner| inner.clicked_modifiers(actions))
    }

    /// See [`Button::pressed_modifiers()`].
    pub fn pressed_modifiers(&self, actions: &Actions) ->  Option<KeyModifiers> {
        self.borrow().and_then(|inner| inner.pressed_modifiers(actions))
    }

    /// See [`Button::released_modifiers()`].
    pub fn released_modifiers(&self, actions: &Actions) -> Option<KeyModifiers> {
        self.borrow().and_then(|inner| inner.released_modifiers(actions))
    }

    pub fn set_visible(&self, cx: &mut Cx, visible: bool) {
        if let Some(mut inner) = self.borrow_mut() {
            inner.visible = visible;
            inner.redraw(cx);
        }
    }

    pub fn set_enabled(&self, cx: &mut Cx, enabled: bool) {
        if let Some(mut inner) = self.borrow_mut() {
            inner.enabled = enabled;
            inner.redraw(cx);
        }
    }

    /// Resets the hover state of this button.
    ///
    /// This is useful in certain cases where the hover state should be reset 
    /// (cleared) regardelss of whether the mouse is over it.
    pub fn reset_hover(&self, cx: &mut Cx) {
        if let Some(mut inner) = self.borrow_mut() {
            inner.animator_cut(cx, ids!(hover.off));
        }
    }
}

impl ButtonSet {
    pub fn clicked(&self, actions: &Actions) -> bool {
        self.iter().any(|v| v.clicked(actions))
    }
    pub fn pressed(&self, actions: &Actions) -> bool {
        self.iter().any(|v| v.pressed(actions))
    }
    pub fn released(&self, actions: &Actions) -> bool {
        self.iter().any(|v| v.released(actions))
    }

    pub fn reset_hover(&self, cx: &mut Cx) {
        for item in self.iter() {
            item.reset_hover(cx)
        }
    }
    
    pub fn which_clicked_modifiers(&self, actions: &Actions) -> Option<(usize,KeyModifiers)> {
        for (index,btn) in self.iter().enumerate(){
            if let Some(km) = btn.clicked_modifiers(actions){
                return Some((index, km))
            }
        }
        None
    }

    pub fn set_visible(&self, cx:&mut Cx, visible: bool) {
        for item in self.iter() {
            item.set_visible(cx, visible)
        }
    }
    pub fn set_enabled(&self, cx:&mut Cx, enabled: bool) {
        for item in self.iter() {
            item.set_enabled(cx, enabled)
        }
    }
}
