use makepad_widgets::*;
use crate::widgets::modal::{Modal, ModalAction};

live_design! {
    use makepad_widgets::base::*;
    use makepad_widgets::theme_desktop_dark::*;
    use makepad_widgets::view_ui::View;
    use link::styling::*;
    use crate::widgets::text::Text;
    use crate::widgets::button::Btn;
    use crate::widgets::modal::Modal;
    use crate::widgets::modal::DialogStyle;

    pub DatePicker = {{DatePicker}} {
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
                    text: "Select Date"
                }

                // TODO: Calendar grid will go here
                placeholder = <View> {
                    width: 280.0, height: 200.0
                    show_bg: true
                    draw_bg: { color: #333 }
                    align: {x: 0.5, y: 0.5}

                    <Text> {
                        width: Fit, height: Fit
                        text: "Calendar Placeholder"
                        draw_text: {
                            color: #666
                            text_style: { font_size: 14.0 }
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
pub enum DatePickerAction {
    None,
    Dismissed,
    Accepted { year: i32, month: u32, day: u32 },
}

#[derive(Live, LiveHook, Widget)]
pub struct DatePicker {
    #[deref] view: View,
}

impl DatePicker {
    fn modal_ref(&self) -> WidgetRef {
        self.view.widget(ids!(modal))
    }

    pub fn is_open(&self) -> bool {
        self.modal_ref().borrow::<Modal>().map(|m| m.is_open()).unwrap_or(false)
    }

    pub fn open(&mut self, cx: &mut Cx) {
        if let Some(mut modal) = self.modal_ref().borrow_mut::<Modal>() {
            modal.open(cx);
        }
    }

    pub fn close(&mut self, cx: &mut Cx) {
        if let Some(mut modal) = self.modal_ref().borrow_mut::<Modal>() {
            modal.close(cx);
        }
    }

    // TODO: Implement when calendar is ready
    fn get_selected_date(&self) -> (i32, u32, u32) {
        (2025, 1, 1) // Placeholder
    }

    // TODO: Implement when calendar is ready
    pub fn set_date(&self, _cx: &mut Cx, _year: i32, _month: u32, _day: u32) {
        // Will set calendar to specific date
    }
}

impl Widget for DatePicker {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        let uid = self.widget_uid();
        let modal_uid = self.modal_ref().borrow::<Modal>().map(|m| m.widget_uid());

        self.view.handle_event(cx, event, scope);

        if let Event::Actions(actions) = event {
            if let Some(modal_uid) = modal_uid {
                if let Some(action) = actions.find_widget_action(modal_uid) {
                    match action.cast() {
                        ModalAction::Accepted => {
                            let (year, month, day) = self.get_selected_date();
                            cx.widget_action(uid, &scope.path, DatePickerAction::Accepted { year, month, day });
                        }
                        ModalAction::Dismissed => {
                            cx.widget_action(uid, &scope.path, DatePickerAction::Dismissed);
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
