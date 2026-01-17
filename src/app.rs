use makepad_widgets::*;
use crate::widgets::input::{Input, InputAction};
use crate::widgets::button::Button as Btn;
use crate::widgets::modal::Modal;

live_design!{
    use makepad_widgets::base::*;
    use makepad_widgets::theme_desktop_dark::*; 
    use makepad_widgets::window::Window;
    use makepad_widgets::scroll_bars::ScrollBars;
    use makepad_widgets::view_ui::View;
    use makepad_widgets::button::Button;
    use crate::widgets::input::Input;
    use crate::widgets::button::Btn;
    use crate::widgets::button::Btn;
    use crate::widgets::modal::Modal;
    use crate::widgets::text::Text;
    use crate::theme::*;
    
    App = {{App}} {
        ui: <Window> {
            window: {inner_size: vec2(900, 700), min_size: vec2(600, 400)}
            body = <View> {
                width: Fill, height: Fill
                flow: Overlay // Overlay allows stacking for modal
                
                main_content = <View> {
                    width: Fill, height: Fill,
                    flow: Down,
                    spacing: 20.0,
                    padding: 20.0,
                    
                    show_bg: true,
                    draw_bg: {
                        color: #1f1f1f
                    }
                    
                    scroll_bars: <ScrollBars> {}
                    
                    // Typography Section
                    <Text> {
                        width: Fit, margin: {bottom: 10.0}
                        text: "Typography System"
                        draw_text: {
                            color: #9cb4d8
                            text_style: { font_size: 14.0 }
                        }
                    }
    
                    <Text> {
                        text: "Regular Text (Inter)"
                        draw_text: {
                            color: #ffffff
                            text_style: { font_size: 20.0 }
                        }
                    }
                    
                    <Text> {
                        text: "Monospace Text (Ubuntu Mono)"
                        draw_text: {
                            color: #a3d9ff
                            text_style: <THEME_FONT_MONO> { font_size: 20.0 }
                        }
                    }
                    
                    <Text> {
                        text: "code_snippet = fn main() {}"
                        draw_text: {
                            color: #ffaa00
                            text_style: <THEME_FONT_MONO> { font_size: 16.0 }
                        }
                    }
                    
                    input1 = <Input> {
                        width: Fill, height: Fit
                        empty_text: "* Morning Routine"
                        is_numeric_only: true
                        is_required: true
                    }
                    submit_btn = <Btn> {
                        text: "Submit"
                    }
                    
                    <View> {
                        width: Fill, height: Fit
                        flow: Right
                        spacing: 20.0
                        align: {y: 0.5}
                        
                        <Btn> {
                            text: "Demo Enabled"
                        }
                        <Btn> {
                            text: "Demo Disabled"
                            enabled: false
                        }
                    }
                    
                    open_modal_btn = <Btn> {
                        margin: {top: 20.0}
                        text: "Open Modal"
                    }
                }
                
                demo_modal = <Modal> {
                    visible: false
                }
            }
        }
    }
}

#[derive(Live, LiveHook)]
pub struct App {
    #[live] ui: WidgetRef,
}

impl LiveRegister for App {
    fn live_register(_cx: &mut Cx) {
    }
}

impl AppMain for App {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event) {
        let scope = &mut Scope::empty();
        self.ui.handle_event(cx, event, scope);
        
        if let Event::Actions(actions) = event {
            let submit_clicked = self.ui.widget(ids!(submit_btn))
                .borrow::<Btn>()
                .map(|btn| btn.clicked(&actions))
                .unwrap_or(false);

            if submit_clicked {
                if let Some(mut input) = self.ui.widget(ids!(input1)).borrow_mut::<Input>() {
                     let valid = input.validate(cx);
                     if let Some(mut btn) = self.ui.widget(ids!(submit_btn)).borrow_mut::<Btn>() {
                        btn.set_disabled(cx, !valid);
                     }
                     
                     if valid {
                         log!("Submitted successfully!");
                     } else {
                         log!("Validation failed");
                     }
                }
            }
        
            if let Some(mut input) = self.ui.widget(ids!(input1)).borrow_mut::<Input>() {
                if let Some(action) = actions.find_widget_action(input.widget_uid()) {
                    if let InputAction::Changed(_) = action.cast() {
                        let valid = input.validate(cx);
                        if let Some(mut btn) = self.ui.widget(ids!(submit_btn)).borrow_mut::<Btn>() {
                            btn.set_disabled(cx, !valid);
                        }
                    }
                }
            }

             // Open Modal Logic
             let mut open_modal = false;
             if let Some(btn) = self.ui.widget(ids!(open_modal_btn)).borrow::<Btn>() {
                 if btn.clicked(&actions) {
                     open_modal = true;
                 }
             }
             
             if open_modal {
                 let mut opened = false;
                 if let Some(mut modal) = self.ui.widget(ids!(demo_modal)).borrow_mut::<Modal>() {
                     modal.set_visible(cx, true);
                     opened = true;
                 }
                 if opened {
                     self.ui.redraw(cx);
                 }
             }
             
             // Modal Actions
             let modal = self.ui.widget(ids!(demo_modal));
             let inner = modal.widget(ids!(modal_inner));
             
             let mut close = false;
             if let Some(btn) = inner.widget(ids!(cancel_button)).borrow::<Btn>(){
                if btn.clicked(&actions){
                    close = true;
                }
             }
             if let Some(btn) = inner.widget(ids!(ok_button)).borrow::<Btn>(){
                if btn.clicked(&actions){
                    close = true;
                }
             }
             
             if close {
                 let mut closed = false;
                 if let Some(mut modal) = modal.borrow_mut::<Modal>() {
                     modal.set_visible(cx, false);
                     closed = true;
                 }
                 if closed {
                     self.ui.redraw(cx);
                 }
             }
        }

    }
}
