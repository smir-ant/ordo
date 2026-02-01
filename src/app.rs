use makepad_widgets::*;
use crate::widgets::input::{Input, InputAction};
use crate::widgets::button::{Button as Btn, ButtonAction};
use crate::widgets::modal::{Modal, TooltipTriggerAction};
use crate::widgets::text::Text;
use crate::widgets::day_of_week::DayOfWeek;
use crate::widgets::tabs::TabsAction;
use crate::widgets::wheel_v::{WheelV, WheelVAction};
use crate::widgets::wheel_h::{WheelH, WheelHAction};
use crate::widgets::time_picker::{TimePicker, TimePickerAction};
use crate::widgets::date_picker::{DatePicker, DatePickerAction};
use crate::utils::calendar;
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
    use crate::widgets::modal::DialogStyle;
    use crate::widgets::modal::TooltipStyle;
    use crate::widgets::modal::SidePanelStyle;
    use crate::widgets::modal::TooltipTrigger;
    
    use crate::widgets::text::Text;
    // use crate::widgets::hint::Hint; // Deleted
    use crate::widgets::group::Group;
    use crate::widgets::day_of_week::DayOfWeek;
    use crate::widgets::tabs::Tabs;
    use crate::widgets::check::Check;
    use crate::widgets::details::Details;
    use crate::widgets::wheel_v::WheelV;
    use crate::widgets::wheel_h::WheelH;
    use crate::widgets::time_picker::TimePicker;
    use crate::widgets::date_picker::DatePicker;
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


                        // ========== Typography ==========
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

                        <Text> {
                            text: "Regular Text (Inter)"
                            draw_text: {
                                color: #ffaa00
                                text_style: { font_size: 20.0 }
                            }
                        }


                        // ========== BUTTONS ==========
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


                        // ========== Inputs ==========
                        <Group> {
                            width: Fill, height: Fit
                            <Input> {
                                width: Fill, height: Fit
                                is_numeric_only: true
                                empty_text: "Numeric Only"
                            }
    
                            <Input> {
                                width: Fill, height: Fit
                                empty_text: "Standard Text Input"
                            }
                        }
                        
                        input_group = <Group> {
                            width: Fill, height: Fit
                            input1 = <Input> {
                                width: Fill, height: Fit
                                empty_text: "* Validate only num && !empty"
                                is_numeric_only: true
                                is_required: true
                            }

                            submit_btn = <Btn> {
                                text: "Submit"
                            }
                        }

                    
                        // ========== Day Of Week ==========
                        <Group> {
                            width: Fill, height: Fit
                            flow: Down
                            spacing: 10.0

                            <Text> {
                                text: "Day Of Week"
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
                        
                        
                        // ========== TABS ==========
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
                            
                            tabs_single = <Tabs> {
                                labels: ["Only Option"]
                            }
                            
                            tabs_two = <Tabs> {
                                labels: ["Require All", "Require Any"]
                            }
                            
                            tabs_six = <Tabs> {
                                labels: ["Day", "Week", "Month", "Year", "All Time", "Custom"]
                            }
                        }
                        

                        // ========== CHECKBOX ==========
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

                        // ========== Details ==========
                        <Group> {
                            details_demo = <Details> {
                                summary: "Details section"
                                
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
                            summary: "Pre-opened Section without frame"
                            open: true
                            
                            content = {
                                <Text> {
                                    text: "This section starts open"
                                    draw_text: { color: #AAA, text_style: { font_size: 12.0 } }
                                }
                            }
                        }

                        <Group> {
                            width: Fill, height: Fit
                            flow: Down
                            spacing: 10.0

                            <Text> {
                                text: "WheelV (vertical)"
                                draw_text: {
                                    color: #DDD
                                    text_style: { font_size: 13.0 }
                                }
                            }

                            hour_picker = <WheelV> {
                                width: 100.0, height: 160.0
                                range_min: 0
                                range_max: 23
                                initial_value: 12
                            }

                            log_value_btn = <Btn> {
                                width: Fit
                                text: "Get Value"
                            }
                        }

                        <Group> {
                            width: Fill, height: Fit
                            flow: Down
                            spacing: 10.0

                            <Text> {
                                text: "WheelH (horizontal) - Year"
                                draw_text: {
                                    color: #DDD
                                    text_style: { font_size: 13.0 }
                                }
                            }

                            year_picker = <WheelH> {
                                width: Fill, height: 40.0
                                step_width: 80.0
                                range_min: 2001
                                range_max: 2051
                                initial_value: 2026
                                is_infinite: false
                            }

                            <Text> {
                                text: "WheelH - Month (with labels)"
                                draw_text: {
                                    color: #DDD
                                    text_style: { font_size: 13.0 }
                                }
                            }

                            month_picker = <WheelH> {
                                width: Fill, height: 40.0
                                is_infinite: true
                            }
                        }

                        <View> {
                            width: Fill, height: Fit
                            flow: Right
                            spacing: 10.0
                            align: {y: 0.5}

                            <Text> {
                                width: Fit
                                text: "Modal:"
                                draw_text: {
                                    color: #DDD
                                    text_style: { font_size: 13.0 }
                                }
                            }

                            open_modal_btn = <Btn> {
                                text: "Open Modal"
                            }
                            
                            side_panel_btn = <Btn> {
                                text: "Side Panel"
                            }

                            
                            scroll_wrapper = <TooltipTrigger> {
                                width: Fit, height: Fit
                                
                                <Text> {
                                    text: "RMB or long press for tooltip!"
                                    draw_text: {
                                        color: #ffffff
                                        text_style: { font_size: 12.0 }
                                    }
                                }
                            }
                
                            help_btn = <Btn> {
                                width: Fit, height: Fit
                                text: "?"
                            }
                        }

                        // TimePicker demos
                        <View> {
                            width: Fill, height: Fit
                            flow: Right
                            spacing: 10.0
                            align: {y: 0.5}

                            <Text> {
                                width: Fit
                                text: "TimePicker:"
                                draw_text: {
                                    color: #DDD
                                    text_style: { font_size: 13.0 }
                                }
                            }

                            open_time_picker_btn = <Btn> {
                                width: Fit
                                text: "HH:MM:SS"
                            }

                            open_time_picker_hm_btn = <Btn> {
                                width: Fit
                                text: "HH:MM"
                            }
                        }

                        // DatePicker demo
                        <View> {
                            width: Fill, height: Fit
                            flow: Right
                            spacing: 10.0
                            align: {y: 0.5}

                            <Text> {
                                width: Fit
                                text: "DatePicker:"
                                draw_text: {
                                    color: #DDD
                                    text_style: { font_size: 13.0 }
                                }
                            }

                            open_date_picker_btn = <Btn> {
                                width: Fit
                                text: "Select Date"
                            }
                        }
                    }
                }

                // --- MODAL INSTANCES (Overlay Layer) ---

                demo_modal = <Modal> {
                    content = <DialogStyle> {
                        <Text> {
                            text: "Confirm Action"
                            draw_text: { color: #fff, text_style: { font_size: 16.0 } }
                        }
                        <Text> {
                            width: Fill, height: Fit
                            text: "Are you sure you want to proceed? bla bla bla bla"
                            draw_text: { color: #bbb, wrap: Word, text_style: { font_size: 14.0 } }
                        }
                        buttons_wrap = <View> {
                            width: Fill, height: Fit
                            flow: Right
                            align: {x: 1.0}
                            spacing: 15.0

                            cancel_button = <Btn> { width: 100.0, text: "Cancel", draw_bg: { color: #444 } }
                            ok_button = <Btn> { width: 100.0, text: "OK", accent: true }
                        }
                    }
                }

                trigger_tooltip = <Modal> {
                    content = <TooltipStyle> {
                        <Text> {
                            text: "Trigger Action"
                            draw_text: { color: #fff, text_style: { font_size: 14.0 } }
                        }
                        <Text> {
                            width: Fill, height: Fit
                            text: "Tooltip opened via Right-Click or Long-Press!"
                            draw_text: { color: #ccc, wrap: Word, text_style: { font_size: 12.0 } }
                        }
                        ok_button = <Btn> { width: Fill, text: "Got it" }
                    }
                }

                button_tooltip = <Modal> {
                    content = <TooltipStyle> {
                        height: 300.0  // Max height for scroll test
                        <Text> {
                            text: "Button Action"
                            draw_text: { color: #fff, text_style: { font_size: 14.0 } }
                        }
                        <Text> {
                            width: Fill, height: Fit
                            text: "Line 1: Tooltip opened via button!\nLine 2: This is additional content.\nLine 3: More text here.\nLine 4: And even more.\nLine 5: Testing scrolling.\nLine 6: Is it working?\nLine 7: Let's see.\nLine 8: Another line.\nLine 9: Keep going.\nLine 10: Almost there.\nLine 11: One more.\nLine 12: And another.\nLine 13: Final stretch.\nLine 14: Last few lines.\nLine 15: The end!"
                            draw_text: { color: #ccc, wrap: Word, text_style: { font_size: 12.0 } }
                        }
                        ok_button = <Btn> {
                            width: Fill
                            margin: {bottom: 16.0}
                            text: "Got it"
                        }
                    }
                }

                side_panel_view = <Modal> {
                    align: {x: 0.0, y: 0.0}
                    content = <SidePanelStyle> {
                        <View> {
                            width: Fill, height: Fit
                            padding: 20.0
                            <Text> {
                                text: "Side Panel"
                                draw_text: { color: #fff, text_style: { font_size: 18.0 } }
                            }
                        }
                        <View> {
                            width: Fill, height: Fill
                            flow: Down
                            padding: 20.0
                            spacing: 15.0

                            <Text> {
                                text: "This is a custom side panel.\nYou can put anything here."
                                draw_text: { color: #bbb, text_style: { font_size: 14.0 } }
                            }
                            <Input> {
                                width: Fill, height: Fit
                                empty_text: "Edit me..."
                            }
                        }
                    }
                }

                // TimePicker with seconds
                time_picker = <TimePicker> {
                    with_seconds: true
                }

                // TimePicker without seconds (HH:MM only)
                time_picker_hm = <TimePicker> {
                    with_seconds: false
                }

                // DatePicker
                date_picker = <DatePicker> {}
            }
        }
    }
}

#[derive(Live, LiveHook)]
pub struct App {
    #[live] ui: WidgetRef,
    #[rust] initialized: bool,
}

impl LiveRegister for App {
    fn live_register(_cx: &mut Cx) {
    }
}

impl AppMain for App {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event) {
        // Initialize on first event
        if !self.initialized {
            self.initialized = true;

            // Month picker labels
            let months = vec![
                "Jan".to_string(), "Feb".to_string(), "Mar".to_string(),
                "Apr".to_string(), "May".to_string(), "Jun".to_string(),
                "Jul".to_string(), "Aug".to_string(), "Sep".to_string(),
                "Oct".to_string(), "Nov".to_string(), "Dec".to_string(),
            ];
            if let Some(mut picker) = self.ui.widget(ids!(month_picker)).borrow_mut::<WheelH>() {
                picker.set_labels(months);
                picker.set_value(cx, 0); // January
            }

            // DatePicker button: show today's date
            let today = calendar::today();
            let date_text = format!("{:02}.{:02}.{:04}", today.day, today.month, today.year);
            if let Some(mut btn) = self.ui.widget(ids!(open_date_picker_btn)).borrow_mut::<Btn>() {
                btn.set_text(cx, &date_text);
            }
        }

        let scope = &mut Scope::empty();

        // Check if any modal is open
        let modal_is_open = self.any_modal_open();

        // Always pass draw events to entire UI for rendering
        // Only block input events (finger, mouse, scroll) when modal is open
        let is_input_event = matches!(event,
            Event::MouseDown(_) | Event::MouseUp(_) | Event::MouseMove(_) |
            Event::Scroll(_) | Event::TouchUpdate(_) | Event::LongPress(_)
        );

        if modal_is_open && is_input_event {
            // Route input events only to modals
            for id in Self::modal_widget_ids() {
                self.ui.widget(id).handle_event(cx, event, scope);
            }
        } else {
            // Normal event handling (draw, actions, key events, etc.)
            self.ui.handle_event(cx, event, scope);
        }

        if let Event::Actions(actions) = event {
            // Only handle main content actions when no modal is open
            if !modal_is_open {
                self.handle_main_content_actions(cx, actions);
            }

            // Always handle modal-related actions (open triggers, etc.)
            self.handle_modal_actions(cx, actions);
        }
    }
}

impl App {
    /// All modal-like widgets (Modal and TimePicker)
    /// Add new modals here - single place to update
    fn modal_widget_ids() -> &'static [&'static [LiveId]] {
        &[
            ids!(demo_modal),
            ids!(trigger_tooltip),
            ids!(button_tooltip),
            ids!(side_panel_view),
            ids!(time_picker),
            ids!(time_picker_hm),
            ids!(date_picker),
        ]
    }

    fn any_modal_open(&self) -> bool {
        for id in Self::modal_widget_ids() {
            let widget = self.ui.widget(id);

            if let Some(modal) = widget.borrow::<Modal>() {
                if modal.is_open() { return true; }
            }
            if let Some(tp) = widget.borrow::<TimePicker>() {
                if tp.is_open() { return true; }
            }
            if let Some(dp) = widget.borrow::<DatePicker>() {
                if dp.is_open() { return true; }
            }
        }
        false
    }

    fn handle_main_content_actions(&mut self, cx: &mut Cx, actions: &Actions) {
        // Generic Scroll Handler
        let mut total_scroll_delta = DVec2::default();
        for action in actions {
            if let InputAction::Scroll(delta) = action.cast() {
                total_scroll_delta += delta;
            } else if let ButtonAction::Scroll(delta) = action.cast() {
                total_scroll_delta += delta;
            }
        }

        if total_scroll_delta.x != 0.0 || total_scroll_delta.y != 0.0 {
            if let Some(mut view) = self.ui.widget(ids!(main_content)).borrow_mut::<KeyboardView>() {
                let current_scroll = view.get_scroll_pos(cx);
                let new_scroll = DVec2 {
                    x: current_scroll.x - total_scroll_delta.x,
                    y: current_scroll.y - total_scroll_delta.y,
                };
                view.set_scroll_pos(cx, new_scroll);
                view.redraw(cx);
            }
        }

        // Input Validation
        let submit_clicked = self.ui.widget(ids!(submit_btn))
            .borrow::<Btn>()
            .map(|btn| btn.clicked(actions))
            .unwrap_or(false);

        let input_returned = self.ui.widget(ids!(input1))
            .borrow::<Input>()
            .map(|input| input.returned(actions).is_some())
            .unwrap_or(false);

        if submit_clicked || input_returned {
            if let Some(mut input) = self.ui.widget(ids!(input1)).borrow_mut::<Input>() {
                if input.validate(cx) {
                    log!("Submitted: {}", input.text());
                } else {
                    log!("Input is invalid/empty!");
                }
            }
        }

        // Handle Receive DOW
        if let Some(btn) = self.ui.widget(ids!(receive_dow_btn)).borrow::<Btn>() {
            if btn.clicked(actions) {
                if let Some(dow) = self.ui.widget(ids!(day_of_week)).borrow::<DayOfWeek>() {
                    log!("Selected Days Indices: {:?}", dow.get_selected_days());
                }
            }
        }

        // Tabs and WheelPicker events
        for action in actions {
            if let TabsAction::Changed(idx) = action.cast() {
                log!("Tab changed to index: {}", idx);
            }
            if let WheelVAction::Changed(val) = action.cast() {
                log!("Hour picked: {}", val);
            }
        }

        // Get Value button
        if self.ui.widget(ids!(log_value_btn)).borrow::<Btn>().map(|b| b.clicked(actions)).unwrap_or(false) {
            if let Some(picker) = self.ui.widget(ids!(hour_picker)).borrow::<WheelV>() {
                log!("Current WheelPicker Value: {}", picker.get_value());
            }
        }
    }

    fn handle_modal_actions(&mut self, cx: &mut Cx, actions: &Actions) {
        // Open Modal Button
        if self.ui.widget(ids!(open_modal_btn)).borrow::<Btn>().map(|b| b.clicked(actions)).unwrap_or(false) {
            // Set dynamic title
            let modal_ref = self.ui.widget(ids!(demo_modal));
            if let Some(mut title) = modal_ref.widget(ids!(content)).widget(ids!(title)).borrow_mut::<Text>() {
                title.set_text(cx, "Dynamic Dialog Title");
            }
                if let Some(mut modal) = self.ui.widget(ids!(demo_modal)).borrow_mut::<Modal>() {
                modal.open(cx);
            }
        }

        // Tooltip Trigger Action (RMB / Long Press)
        for action in actions {
            if let TooltipTriggerAction::ShowTooltip = action.as_widget_action().cast() {
                if let Some(mut modal) = self.ui.widget(ids!(trigger_tooltip)).borrow_mut::<Modal>() {
                    modal.open(cx);
                }
            }
        }

        // Helper Button Tooltip
        if self.ui.widget(ids!(help_btn)).borrow::<Btn>().map(|b| b.clicked(actions)).unwrap_or(false) {
            if let Some(mut modal) = self.ui.widget(ids!(button_tooltip)).borrow_mut::<Modal>() {
                modal.open(cx);
            }
        }

        // Open SidePanel Button
        if self.ui.widget(ids!(side_panel_btn)).borrow::<Btn>().map(|b| b.clicked(actions)).unwrap_or(false) {
            if let Some(mut modal) = self.ui.widget(ids!(side_panel_view)).borrow_mut::<Modal>() {
                modal.open(cx);
            }
        }

        // Open TimePicker (HH:MM:SS)
        if self.ui.widget(ids!(open_time_picker_btn)).borrow::<Btn>().map(|b| b.clicked(actions)).unwrap_or(false) {
            if let Some(mut tp) = self.ui.widget(ids!(time_picker)).borrow_mut::<TimePicker>() {
                tp.open(cx);
            }
        }

        // Open TimePicker (HH:MM only)
        if self.ui.widget(ids!(open_time_picker_hm_btn)).borrow::<Btn>().map(|b| b.clicked(actions)).unwrap_or(false) {
            if let Some(mut tp) = self.ui.widget(ids!(time_picker_hm)).borrow_mut::<TimePicker>() {
                tp.open(cx);
            }
        }

        // Handle TimePicker actions
        for action in actions {
            match action.as_widget_action().cast() {
                TimePickerAction::Accepted { hours, minutes, seconds } => {
                    if let Some(secs) = seconds {
                        log!("Time selected: {:02}:{:02}:{:02}", hours, minutes, secs);
                    } else {
                        log!("Time selected: {:02}:{:02}", hours, minutes);
                    }
                }
                TimePickerAction::Dismissed => {
                    log!("Time picker dismissed");
                }
                _ => {}
            }
        }

        // Open DatePicker
        if self.ui.widget(ids!(open_date_picker_btn)).borrow::<Btn>().map(|b| b.clicked(actions)).unwrap_or(false) {
            if let Some(mut dp) = self.ui.widget(ids!(date_picker)).borrow_mut::<DatePicker>() {
                dp.open(cx);
            }
        }

        // Handle DatePicker actions
        for action in actions {
            match action.as_widget_action().cast() {
                DatePickerAction::Accepted { year, month, day } => {
                    log!("Date selected: {:02}.{:02}.{:04}", day, month, year);
                    let date_text = format!("{:02}.{:02}.{:04}", day, month, year);
                    if let Some(mut btn) = self.ui.widget(ids!(open_date_picker_btn)).borrow_mut::<Btn>() {
                        btn.set_text(cx, &date_text);
                    }
                }
                DatePickerAction::Dismissed => {
                    log!("Date picker dismissed");
                }
                _ => {}
            }
        }
    }
}
