use makepad_widgets::*;
use crate::widgets::input::{Input, InputAction};
use crate::widgets::button::{Button as Btn, ButtonAction};
use crate::widgets::modal::Modal;
use crate::widgets::wrapper::{Wrapper, WrapperAction};
use makepad_widgets::view::View;
use makepad_widgets::keyboard_view::KeyboardView;

live_design!{
    use makepad_widgets::base::*;
    use makepad_widgets::theme_desktop_dark::*; 
    use makepad_widgets::window::Window;
    use makepad_widgets::scroll_bars::ScrollBars;
    use makepad_widgets::view_ui::View;
    use makepad_widgets::button::Button;
    use crate::widgets::input::Input;
    use crate::widgets::button::Btn;
    use crate::widgets::modal::Modal;
    use crate::widgets::text::Text;
    use crate::widgets::modal::Modal;
    use crate::widgets::text::Text;
    use crate::widgets::wrapper::Wrapper;
    use crate::theme::*;
    use makepad_widgets::keyboard_view::KeyboardView;
    
    App = {{App}} {
        ui: <Window> {
            window: {inner_size: vec2(900, 700), min_size: vec2(600, 400)}
            body = <View> {
                width: Fill, height: Fill
                flow: Overlay // Overlay allows stacking for modal
                
                main_content = <KeyboardView> {
                    width: Fill, height: Fill,
                    flow: Down,
                    spacing: 20.0,
                    padding: 20.0,
                    
                    show_bg: true,
                    draw_bg: {
                        color: #1f1f1f
                    }
                    
                    scroll_bars: <ScrollBars> {
                        show_scroll_x: false
                        show_scroll_y: true
                        scroll_bar_y: {
                            drag_scrolling: true 
                            smoothing: 0.15
                        }
                    }
                    
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

                    <Text> {
                        width: Fill, margin: {top: 20.0}
                        text: "Lorem ipsum dolor sit amet, consectetur adipiscing elit. Sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat. Duis aute irure dolor in reprehenderit in voluptate velit esse cillum dolore eu fugiat nulla pariatur."
                        draw_text: {
                            wrapping: Word
                            color: #888
                            text_style: { font_size: 12.0 }
                        }
                    }
                    
                    <Text> {
                        width: Fill, margin: {top: 10.0}
                        text: "Съешь ещё этих мягких французских булок, да выпей чаю."
                        draw_text: {
                            wrapping: Word
                            color: #888
                            text_style: { font_size: 12.0 }
                        }
                    }
                    
                    <Text> {
                        width: Fill, margin: {top: 10.0}
                        text: "!@#$%^&*()_+-=[]{}|;':\",./<>?`~№"
                        draw_text: {
                            color: #888
                            text_style: { font_size: 14.0 }
                        }
                    }
                    
                    <Text> {
                        width: Fill, margin: {top: 10.0}
                        text: "A a B E e K M H O o P p C c T y X x (English)"
                        draw_text: {
                            color: #888
                            text_style: { font_size: 14.0 }
                        }
                    }
                    
                    <Text> {
                        width: Fill, margin: {top: 5.0}
                        text: "А а В Е е К М Н О о Р р С с Т у Х х (Russian)"
                        draw_text: {
                            color: #888
                            text_style: { font_size: 14.0 }
                        }
                    }
                    
                    <Input> {
                        width: Fill, height: Fit, margin: {top: 20.0}
                        is_numeric_only: true
                        empty_text: "Numeric Only"
                    }

                    <Input> {
                        width: Fill, height: Fit, margin: {top: 10.0}
                        empty_text: "Standard Text Input"
                    }
                    
                    <Wrapper> {
                         width: Fill, height: Fit
                        <Text> {
                            width: Fill, margin: {top: 20.0}
                            text: "--- Scroll Test Area ---"
                            draw_text: { color: #FFF, text_style: { font_size: 14.0 } }
                        }
                    }
                    <Text> { width: Fill, margin: {top: 10.0}, text: "Line 1: Testing scroll behavior..." }
                    <Text> { width: Fill, margin: {top: 10.0}, text: "Line 2: Swipe up to see more." }
                    <Text> { width: Fill, margin: {top: 10.0}, text: "Line 3: Keyboard should push this up." }
                    <Text> { width: Fill, margin: {top: 10.0}, text: "Line 4: Bottom of the scrollable area." }
                    <Text> { width: Fill, margin: {top: 10.0}, text: "Line 5: Use this to test edge bounce." }
                    <Text> { width: Fill, margin: {top: 10.0, bottom: 50.0}, text: "--- End of Content ---" }
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
            // Generic Scroll Handler
            let mut total_scroll_delta = DVec2::default();
            for action in actions {
                let mut handled = false;

                if let InputAction::Scroll(delta) = action.cast() {
                    total_scroll_delta += delta;
                    handled = true;
                }
                
                if !handled {
                    if let ButtonAction::Scroll(delta) = action.cast() {
                         total_scroll_delta += delta;
                         handled = true;
                    }
                }
                
                if !handled {
                     if let WrapperAction::Scroll(delta) = action.cast() {
                         total_scroll_delta += delta;
                         handled = true;
                    }
                }
                
                // Fallback: Parse Debug String if cast fails (handles TypeId mismatches)
                if !handled {
                    let action_str = format!("{:?}", action);
                    if action_str.contains("Scroll") && action_str.contains("Vec2d") {
                        // Expected format: ... Scroll(Vec2d { x: VAL, y: VAL }) ...
                        if let Some(start) = action_str.find("Scroll(Vec2d { x: ") {
                            let rest = &action_str[start..];
                            if let Some(x_start) = rest.find("x: ") {
                                 let x_str = &rest[x_start + 3..];
                                 if let Some(comma) = x_str.find(",") {
                                     let x_val_str = &x_str[..comma];
                                     if let Some(y_start) = x_str.find("y: ") {
                                         let y_str = &x_str[y_start + 3..];
                                         if let Some(brace) = y_str.find("}") {
                                             let y_val_str = &y_str[..brace].trim();
                                             
                                             if let (Ok(x), Ok(y)) = (x_val_str.parse::<f64>(), y_val_str.parse::<f64>()) {
                                                 let delta = DVec2 { x, y };
                                                 total_scroll_delta += delta;
                                             }
                                         }
                                     }
                                 }
                            }
                        }
                    }
                }
            }

            if total_scroll_delta.x != 0.0 || total_scroll_delta.y != 0.0 {
                 if let Some(mut view) = self.ui.widget(ids!(main_content)).borrow_mut::<KeyboardView>() {
                    let current_scroll = view.get_scroll_pos(cx);
                    let new_scroll = DVec2{x: current_scroll.x - total_scroll_delta.x, y: current_scroll.y - total_scroll_delta.y};
                    view.set_scroll_pos(cx, new_scroll);
                    view.redraw(cx);
                }
            }

            // Input Validation Logic (Specific to input1)
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
            
            // Check Input 1 Changed (Validation)
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
                 if let Some(action) = actions.find_widget_action(btn.widget_uid()) {
                      if let ButtonAction::Clicked(_) = action.cast() {
                           open_modal = true;
                       }
                 }
             }
             
             if open_modal {
                  if let Some(mut modal) = self.ui.widget(ids!(demo_modal)).borrow_mut::<Modal>() {
                      modal.set_visible(cx, true);
                  }
                  self.ui.redraw(cx);
             }
             
             // Check for Wrapper Actions (Right Click / Long Press)
             for action in actions { // This will re-iterate over the actions if `actions` is a slice or reference
                 if let WrapperAction::RightClick = action.as_widget_action().cast() {
                     open_modal = true;
                 }
                 if let WrapperAction::LongPress = action.as_widget_action().cast() {
                     open_modal = true;
                 }
                 if let WrapperAction::Scroll(delta) = action.as_widget_action().cast() {
                    // Manually scroll main_content if the wrapper captured the touch
                    if let Some(mut view) = self.ui.widget(ids!(main_content)).borrow_mut::<KeyboardView>() {
                        let current_scroll = view.get_scroll_pos(cx);
                        let new_scroll = DVec2{x: current_scroll.x - delta.x, y: current_scroll.y - delta.y};
                        view.set_scroll_pos(cx, new_scroll);
                        view.redraw(cx);
                    }
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
             
             if let Some(action) = actions.find_widget_action(modal.widget_uid()) {
                if let crate::widgets::modal::ModalAction::Dismissed = action.cast() {
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
