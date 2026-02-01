use makepad_widgets::*;
use crate::widgets::modal::{Modal, ModalAction};
use crate::widgets::wheel::Wheel;

live_design! {
    use makepad_widgets::base::*;
    use makepad_widgets::theme_desktop_dark::*;
    use makepad_widgets::view_ui::View;
    use link::styling::*;
    use crate::widgets::text::Text;
    use crate::widgets::button::Btn;
    use crate::widgets::modal::Modal;
    use crate::widgets::modal::DialogStyle;
    use crate::widgets::wheel::WheelV;

    pub TimePicker = {{TimePicker}} {
        width: Fill, height: Fill
        flow: Overlay

        modal = <Modal> {
            content = <DialogStyle> {
                width: Fit
                spacing: 15.0

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

                    hour_picker = <WheelV> {
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

                    minute_picker = <WheelV> {
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

                        second_picker = <WheelV> {
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
}

#[derive(Clone, Debug, DefaultNone)]
pub enum TimePickerAction {
    None,
    Dismissed,
    Accepted { hours: i32, minutes: i32, seconds: Option<i32> },
}

#[derive(Live, LiveHook, Widget)]
pub struct TimePicker {
    #[deref] view: View,
    #[live(true)] with_seconds: bool,
}

impl TimePicker {
    fn modal_ref(&self) -> WidgetRef {
        self.view.widget(ids!(modal))
    }

    pub fn is_open(&self) -> bool {
        self.modal_ref().borrow::<Modal>().map(|m| m.is_open()).unwrap_or(false)
    }

    pub fn open(&mut self, cx: &mut Cx) {
        self.update_seconds_visibility(cx);
        if let Some(mut modal) = self.modal_ref().borrow_mut::<Modal>() {
            modal.open(cx);
        }
    }

    pub fn close(&mut self, cx: &mut Cx) {
        if let Some(mut modal) = self.modal_ref().borrow_mut::<Modal>() {
            modal.close(cx);
        }
    }

    fn update_seconds_visibility(&self, cx: &mut Cx) {
        let content = self.modal_ref().widget(ids!(content));
        let pickers_row = content.widget(ids!(pickers_row));
        let seconds_wrap = pickers_row.widget(ids!(seconds_wrap));

        if self.with_seconds {
            seconds_wrap.apply_over(cx, live!{ width: Fit });
        } else {
            seconds_wrap.apply_over(cx, live!{ width: 0 });
        }
    }

    fn get_current_time(&self) -> (i32, i32, Option<i32>) {
        let content = self.modal_ref().widget(ids!(content));
        let pickers_row = content.widget(ids!(pickers_row));

        let hours = pickers_row.widget(ids!(hour_picker))
            .borrow::<Wheel>()
            .map(|p| p.get_value())
            .unwrap_or(0);

        let minutes = pickers_row.widget(ids!(minute_picker))
            .borrow::<Wheel>()
            .map(|p| p.get_value())
            .unwrap_or(0);

        let seconds = if self.with_seconds {
            Some(pickers_row.widget(ids!(seconds_wrap)).widget(ids!(second_picker))
                .borrow::<Wheel>()
                .map(|p| p.get_value())
                .unwrap_or(0))
        } else {
            None
        };

        (hours, minutes, seconds)
    }

    pub fn set_time(&self, cx: &mut Cx, hours: i32, minutes: i32, seconds: Option<i32>) {
        let content = self.modal_ref().widget(ids!(content));
        let pickers_row = content.widget(ids!(pickers_row));

        if let Some(mut picker) = pickers_row.widget(ids!(hour_picker)).borrow_mut::<Wheel>() {
            picker.set_value(cx, hours);
        }
        if let Some(mut picker) = pickers_row.widget(ids!(minute_picker)).borrow_mut::<Wheel>() {
            picker.set_value(cx, minutes);
        }
        if let Some(secs) = seconds {
            if let Some(mut picker) = pickers_row.widget(ids!(seconds_wrap)).widget(ids!(second_picker)).borrow_mut::<Wheel>() {
                picker.set_value(cx, secs);
            }
        }
    }
}

impl Widget for TimePicker {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        let uid = self.widget_uid();
        let modal_uid = self.modal_ref().borrow::<Modal>().map(|m| m.widget_uid());

        self.view.handle_event(cx, event, scope);

        if let Event::Actions(actions) = event {
            if let Some(modal_uid) = modal_uid {
                if let Some(action) = actions.find_widget_action(modal_uid) {
                    match action.cast() {
                        ModalAction::Accepted => {
                            let (hours, minutes, seconds) = self.get_current_time();
                            cx.widget_action(uid, &scope.path, TimePickerAction::Accepted { hours, minutes, seconds });
                        }
                        ModalAction::Dismissed => {
                            cx.widget_action(uid, &scope.path, TimePickerAction::Dismissed);
                        }
                        _ => {}
                    }
                }
            }
        }
    }

    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        self.view.draw_walk(cx, scope, walk)
    }
}
