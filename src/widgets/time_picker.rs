use makepad_widgets::*;
use crate::widgets::button::Button;
use crate::widgets::wheel_picker::WheelPicker;

live_design! {
    use makepad_widgets::base::*;
    use makepad_widgets::theme_desktop_dark::*;
    use makepad_draw::shader::std::*;
    use makepad_widgets::view_ui::View;
    use link::styling::*;
    use crate::widgets::text::Text;
    use crate::widgets::button::Btn;
    use crate::widgets::wheel_picker::WheelPicker;

    pub TimePicker = {{TimePicker}} {
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

        content = <View> {
            width: Fit, height: Fit
            flow: Down
            spacing: 15.0
            padding: 20.0
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

            title = <Text> {
                width: Fit, height: Fit
                draw_text: {
                    text_style: <THEME_FONT_BOLD> { font_size: 14.0 }
                    color: #fff
                }
                text: "Select Time"
            }

            pickers_row = <View> {
                width: Fit, height: Fit
                flow: Right
                align: {y: 0.5}

                hour_picker = <WheelPicker> {
                    width: 80.0, height: 160.0
                    range_min: 0
                    range_max: 23
                }

                separator1 = <Text> {
                    width: 32.0, height: Fit
                    align: {x: 0.5}
                    draw_text: {
                        text_style: <THEME_FONT_BOLD> { font_size: 24.0 }
                        color: #fff
                    }
                    text: ":"
                }

                minute_picker = <WheelPicker> {
                    width: 80.0, height: 160.0
                    range_min: 0
                    range_max: 59
                }

                seconds_wrap = <View> {
                    width: Fit, height: Fit
                    flow: Right
                    align: {y: 0.5}

                    separator2 = <Text> {
                        width: 32.0, height: Fit
                        align: {x: 0.5}
                        draw_text: {
                            text_style: <THEME_FONT_BOLD> { font_size: 24.0 }
                            color: #fff
                        }
                        text: ":"
                    }

                    second_picker = <WheelPicker> {
                        width: 80.0, height: 160.0
                        range_min: 0
                        range_max: 59
                    }
                }
            }

            buttons_wrap = <View> {
                width: Fill, height: Fit
                flow: Right
                spacing: 10.0
                align: {x: 1.0}
                margin: {top: 5.0}

                cancel_button = <Btn> { text: "Cancel" }
                ok_button = <Btn> { text: "OK", accent: true }
            }
        }
    }
}

#[derive(Clone, Debug, DefaultNone)]
pub enum TimePickerAction {
    None,
    Dismissed,
    /// Accepted(hours, minutes, seconds) - seconds is None if with_seconds=false
    Accepted { hours: i32, minutes: i32, seconds: Option<i32> },
}

#[derive(Live, LiveHook, Widget)]
pub struct TimePicker {
    #[deref] view: View,
    #[rust] is_open: bool,
    #[live(true)] with_seconds: bool,
}

impl TimePicker {
    pub fn is_open(&self) -> bool {
        self.is_open
    }

    pub fn open(&mut self, cx: &mut Cx) {
        self.is_open = true;
        self.update_seconds_visibility(cx);
        self.view.redraw(cx);
    }

    pub fn close(&mut self, _cx: &mut Cx) {
        self.is_open = false;
    }

    fn update_seconds_visibility(&self, cx: &mut Cx) {
        let content = self.view.widget(ids!(content));
        let pickers_row = content.widget(ids!(pickers_row));
        let seconds_wrap = pickers_row.widget(ids!(seconds_wrap));

        if self.with_seconds {
            seconds_wrap.apply_over(cx, live!{ width: Fit });
        } else {
            seconds_wrap.apply_over(cx, live!{ width: 0 });
        }
    }

    fn get_current_time(&self) -> (i32, i32, Option<i32>) {
        let content = self.view.widget(ids!(content));
        let pickers_row = content.widget(ids!(pickers_row));

        let hours = pickers_row.widget(ids!(hour_picker))
            .borrow::<WheelPicker>()
            .map(|p| p.get_value())
            .unwrap_or(0);

        let minutes = pickers_row.widget(ids!(minute_picker))
            .borrow::<WheelPicker>()
            .map(|p| p.get_value())
            .unwrap_or(0);

        let seconds = if self.with_seconds {
            Some(pickers_row.widget(ids!(seconds_wrap)).widget(ids!(second_picker))
                .borrow::<WheelPicker>()
                .map(|p| p.get_value())
                .unwrap_or(0))
        } else {
            None
        };

        (hours, minutes, seconds)
    }

    pub fn set_time(&self, cx: &mut Cx, hours: i32, minutes: i32, seconds: Option<i32>) {
        let content = self.view.widget(ids!(content));
        let pickers_row = content.widget(ids!(pickers_row));

        if let Some(mut picker) = pickers_row.widget(ids!(hour_picker)).borrow_mut::<WheelPicker>() {
            picker.set_value(cx, hours);
        }
        if let Some(mut picker) = pickers_row.widget(ids!(minute_picker)).borrow_mut::<WheelPicker>() {
            picker.set_value(cx, minutes);
        }
        if let Some(secs) = seconds {
            if let Some(mut picker) = pickers_row.widget(ids!(seconds_wrap)).widget(ids!(second_picker)).borrow_mut::<WheelPicker>() {
                picker.set_value(cx, secs);
            }
        }
    }
}

impl Widget for TimePicker {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        if !self.is_open {
            return;
        }

        let uid = self.widget_uid();

        self.view.handle_event(cx, event, scope);

        // Handle button clicks
        if let Event::Actions(actions) = event {
            let content = self.view.widget(ids!(content));
            let buttons = content.widget(ids!(buttons_wrap));

            // OK button
            if let Some(btn) = buttons.widget(ids!(ok_button)).borrow::<Button>() {
                if btn.clicked(actions) {
                    let (hours, minutes, seconds) = self.get_current_time();
                    self.close(cx);
                    cx.widget_action(uid, &scope.path, TimePickerAction::Accepted { hours, minutes, seconds });
                    return;
                }
            }

            // Cancel button
            if let Some(btn) = buttons.widget(ids!(cancel_button)).borrow::<Button>() {
                if btn.clicked(actions) {
                    self.close(cx);
                    cx.widget_action(uid, &scope.path, TimePickerAction::Dismissed);
                    return;
                }
            }
        }

        // Escape key
        if let Event::KeyDown(ke) = event {
            if ke.key_code == KeyCode::Escape {
                self.close(cx);
                cx.widget_action(uid, &scope.path, TimePickerAction::Dismissed);
                return;
            }
        }

        // Click outside content
        match event.hits(cx, self.view.area()) {
            Hit::FingerUp(fe) => {
                let content = self.view.widget(ids!(content));
                if content.area() != Area::Empty && !content.area().rect(cx).contains(fe.abs) {
                    self.close(cx);
                    cx.widget_action(uid, &scope.path, TimePickerAction::Dismissed);
                }
            }
            _ => ()
        }
    }

    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        if !self.is_open {
            return DrawStep::done();
        }

        self.view.draw_walk(cx, scope, walk)
    }
}
