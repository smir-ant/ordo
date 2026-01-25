use makepad_widgets::*;
use crate::widgets::input::{Input, InputAction};
use crate::widgets::button::{Button as Btn, ButtonAction};
use crate::widgets::modal::{Modal, ModalAction, TooltipContent, DialogContent, SidePanelContent, TooltipTrigger, TooltipTriggerAction};
use crate::widgets::text::Text;
use crate::widgets::day_of_week::DayOfWeek;
use crate::widgets::tabs::TabsAction;
use crate::widgets::check::Check;
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
    use crate::widgets::modal::DialogContent;
    use crate::widgets::modal::TooltipContent;
    use crate::widgets::modal::SidePanelContent;
    use crate::widgets::modal::TooltipTrigger;
    
    use crate::widgets::text::Text;
    // use crate::widgets::hint::Hint; // Deleted
    use crate::widgets::group::Group;
    use crate::widgets::day_of_week::DayOfWeek;
    use crate::widgets::tabs::Tabs;
    use crate::widgets::check::Check;
    use crate::widgets::details::Details;
    use crate::theme::*;
    use makepad_widgets::keyboard_view::KeyboardView;
    use makepad_widgets::drop_down::DropDown;
    
    App = {{App}} {
        ui: <Window> {
            window: {inner_size: vec2(900, 700), min_size: vec2(600, 400)}
            body = <View> {
                width: Fill, height: Fill
                flow: Overlay // Overlay allows stacking for modal
                
                content_wrapper = <View> {
                    width: Fill, height: Fill
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
                            text: "Regular Text (Inter)"
                            draw_text: {
                                color: #ffaa00
                                text_style: { font_size: 20.0 }
                            }
                        }
                        
                        <Text> {
                             text: "Lorem ipsum dolor sit amet, consectetur adipiscing elit."
                             draw_text: {
                                 color: #888
                                 text_style: { font_size: 14.0 }
                             }
                        }

                        <Text> {
                             text: "Съешь ещё этих мягких французских булок, да выпей чаю."
                             draw_text: {
                                 color: #888
                                 text_style: { font_size: 14.0 }
                             }
                        }
                        
                        input_group = <Group> {
                            width: Fill, height: Fit
                            input1 = <Input> {
                                width: Fill, height: Fit
                                empty_text: "* Morning Routine"
                                is_numeric_only: true
                                is_required: true
                            }

                            submit_btn = <Btn> {
                                text: "Submit"
                            }
                        }
                        
                        <Group> {
                            width: Fill, height: Fit
                            flow: Down
                            spacing: 10.0
                            
                            <Text> {
                                text: "Regularity"
                                draw_text: {
                                    color: #DDD
                                    text_style: { font_size: 13.0 }
                                }
                            }
                            
                            day_of_week = <DayOfWeek> {
                                width: Fill, height: Fit
                            }
                            
                            receive_dow_btn = <Btn> {
                                width: Fit
                                text: "Receive DOW"
                            }
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
                            <Btn> {
                                text: "Demo Accent"
                                accent: true
                            }
                        }
                        
                        // Tabs Demo Section
                        <Group> {
                            width: Fill, height: Fit
                            flow: Down
                            spacing: 15.0
                            
                            <Text> {
                                text: "Tabs Widget Demo"
                                draw_text: {
                                    color: #DDD
                                    text_style: { font_size: 13.0 }
                                }
                            }
                            
                            // Single tab example
                            tabs_single = <Tabs> {
                                labels: ["Only Option"]
                            }
                            
                            // Two tabs example (like check.html)
                            tabs_two = <Tabs> {
                                labels: ["Require All", "Require Any"]
                            }
                            
                            // Six tabs example
                            tabs_six = <Tabs> {
                                labels: ["Day", "Week", "Month", "Year", "All Time", "Custom"]
                            }
                        }
                        
                        // Check Widget Demo
                        <Group> {
                            width: Fill, height: Fit
                            flow: Down
                            spacing: 10.0
                            
                            <Text> {
                                text: "Check Widget Demo"
                                draw_text: {
                                    color: #DDD
                                    text_style: { font_size: 13.0 }
                                }
                            }
                            
                            check_demo = <Check> { label: "Enable feature" }
                            check_demo_2 = <Check> { checked: true, label: "Pre-checked option" }
                            
                        }

                        <Text> {
                            margin: {top: 16.0}
                            text: "Details Widget Demo"
                            draw_text: {
                                color: #DDD
                                text_style: { font_size: 13.0 }
                            }
                        }
                        
                        <Group> {
                            details_demo = <Details> {
                                summary: "Advanced Settings"
                                
                                content = {
                                    <Check> { label: "Allow skipping" }
                                    <Check> { label: "Carry over missed reps" }
                                    <Text> {
                                        text: "Nested content works!"
                                        draw_text: { color: #888, text_style: { font_size: 11.0 } }
                                    }
                                }
                            }
                        }
                        
                        details_open = <Details> {
                            summary: "Pre-opened Section"
                            open: true
                            
                            content = {
                                <Text> {
                                    text: "This section starts open"
                                    draw_text: { color: #AAA, text_style: { font_size: 12.0 } }
                                }
                            }
                        }

                        open_modal_btn = <Btn> {
                            margin: {top: 20.0}
                            text: "Open Modal"
                        }
                        
                        side_panel_btn = <Btn> {
                            margin: {top: 10.0}
                            text: "Side Panel"
                        }

                        <Group> {
                            width: Fill, height: Fit
                            <Input> {
                                width: Fill, height: Fit
                                is_numeric_only: true
                                empty_text: "Numeric Only"
                            }
    
                            <Input> {
                                width: Fill, height: Fit, margin: {top: 10.0}
                                empty_text: "Standard Text Input"
                            }
                        }
                        
                        scroll_wrapper = <TooltipTrigger> {
                             width: Fit, height: Fit
                            
                              <Text> {
                                text: "RMB or long press for tooltip!"
                                draw_text: {
                                    color: #ffffff
                                    text_style: { font_size: 16.0 }
                                }
                            }
                        }
             
                        help_btn = <Btn> {
                            width: Fit, height: Fit
                            text: "?"
                        }
                    }
                }
                
                // --- STATIC MODAL INSTANCES (Overlay Layer) ---
                
                demo_modal = <Modal> {
                    content = <DialogContent> {
                        title = { text: "Confirm Action" }
                        text = { text: "Are you sure you want to proceed?" }
                    }
                }
                
                // Specific Modal for Trigger
                trigger_tooltip = <Modal> {
                    visible: false
                    content = <TooltipContent> {
                        title = { text: "Trigger Action" }
                        text = { text: "Tooltip opened via Right-Click or Long-Press!" }
                    }
                }

                // Specific Modal for Button
                button_tooltip = <Modal> {
                    visible: false
                    content = <TooltipContent> {
                        title = { text: "Button Action" }
                        text = { text: "Tooltip opened via button!" }
                    }
                }
                
                side_panel_view = <Modal> {
                    align: {x: 0.0, y: 0.0} // Left aligned
                    content = <SidePanelContent> {
                        header = { title = { text: "Side Panel" } }
                        body = {
                            <Text> {
                                text: "This is a custom side panel.\nYou can put anything here."
                                draw_text: {
                                    color: #bbb
                                    text_style: { font_size: 14.0 }
                                }
                            }
                            
                            <Input> {
                                width: Fill, height: Fit
                                empty_text: "Edit me..."
                            }
                        }
                    }
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
            // Generic Scroll Handler - collect scroll deltas from all widget types
            let mut total_scroll_delta = DVec2::default();
            for action in actions {
                // Try each scroll action type
                if let InputAction::Scroll(delta) = action.cast() {
                    total_scroll_delta += delta;
                } else if let ButtonAction::Scroll(delta) = action.cast() {
                    total_scroll_delta += delta;
                }
                /* else if let HintAction::Scroll(delta) = action.cast() {
                    total_scroll_delta += delta;
                }*/
            }

            if total_scroll_delta.x != 0.0 || total_scroll_delta.y != 0.0 {
                 if let Some(mut view) = self.ui.widget(ids!(main_content)).borrow_mut::<KeyboardView>() {
                    let current_scroll = view.get_scroll_pos(cx);
                    let new_scroll = DVec2{x: current_scroll.x - total_scroll_delta.x, y: current_scroll.y - total_scroll_delta.y};
                    view.set_scroll_pos(cx, new_scroll);
                    view.redraw(cx);
                }
            }

            // Input Validation Logic
            let submit_clicked = self.ui.widget(ids!(submit_btn))
                .borrow::<Btn>()
                .map(|btn| btn.clicked(&actions))
                .unwrap_or(false);

            let input_returned = self.ui.widget(ids!(input1))
                .borrow::<Input>()
                .map(|input| (*input).returned(actions).is_some())
                .unwrap_or(false);

            if submit_clicked || input_returned {
                 if let Some(mut input) = self.ui.widget(ids!(input1)).borrow_mut::<Input>() {
                     if input.validate(cx) {
                         let text = input.text();
                         log!("Submitted: {}", text);
                     } else {
                         log!("Input is invalid/empty!");
                     }
                 }
            }
            
            // Handle Receive DOW
            if let Some(btn) = self.ui.widget(ids!(receive_dow_btn)).borrow::<Btn>() {
                if btn.clicked(&actions) {
                    if let Some(dow) = self.ui.widget(ids!(day_of_week)).borrow::<DayOfWeek>() {
                        let selected = dow.get_selected_days();
                        log!("Selected Days Indices: {:?}", selected);
                    }
                }
            }
            
            // Tabs Action Handling - log when tabs change
            for action in actions {
                if let TabsAction::Changed(idx) = action.cast() {
                    log!("Tab changed to index: {}", idx);
                }
            }
            
            // --- Modal Logic ---

// --- Modal Logic ---

            // Open Modal Button
            if self.ui.widget(ids!(open_modal_btn)).borrow::<Btn>().map(|b| b.clicked(&actions)).unwrap_or(false) {
                 // 1. Set data (completely isolated scope)
                 {
                     let modal_ref = self.ui.widget(ids!(demo_modal));
                     if let Some(mut title_widget) = modal_ref.widget(ids!(content)).widget(ids!(title)).borrow_mut::<Text>() {
                         title_widget.set_text(cx, "Dynamic Dialog Title");
                     }
                 }
                 
                 // 2. Open Modal
                 let modal_ref = self.ui.widget(ids!(demo_modal));
                 modal_ref.apply_over(cx, live!{visible: true});
                 self.ui.redraw(cx);
            }
            
            // Tooltip Trigger Action
            let mut open_trigger_tooltip = false;
            for action in actions {
                if let TooltipTriggerAction::ShowTooltip = action.as_widget_action().cast() {
                    open_trigger_tooltip = true;
                }
            }
            
            if open_trigger_tooltip {
                 let modal_ref = self.ui.widget(ids!(trigger_tooltip));
                 modal_ref.apply_over(cx, live!{visible: true});
                 self.ui.redraw(cx);
            }
            
            // Helper Button Tooltip
            if self.ui.widget(ids!(help_btn)).borrow::<Btn>().map(|b| b.clicked(&actions)).unwrap_or(false) {
                 let modal_ref = self.ui.widget(ids!(button_tooltip));
                 modal_ref.apply_over(cx, live!{visible: true});
                 self.ui.redraw(cx);
            }
            
            // Handle Modal Results (Logging)
            // Check specifically for demo_modal actions to log them
            if let Some(action) = actions.find_widget_action(self.ui.widget(ids!(demo_modal)).widget_uid()) {
                if let ModalAction::Accepted = action.cast() {
                     // Log handled by content
                } else if let ModalAction::Dismissed = action.cast() {
                     // Log handled by content
                }
            }
            
            // Open SidePanel Button
            if self.ui.widget(ids!(side_panel_btn)).borrow::<Btn>().map(|b| b.clicked(&actions)).unwrap_or(false) {
                 if let Some(mut modal) = self.ui.widget(ids!(side_panel_view)).borrow_mut::<Modal>() {
                    // Try borrow for side panel if simple apply doesn't animate? 
                    // But to be safe let's use apply here too.
                 }
                 self.ui.widget(ids!(side_panel_view)).apply_over(cx, live!{visible: true});
                 self.ui.redraw(cx); 
            }
            
        }
    }
}
