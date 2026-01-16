use makepad_widgets::*;

live_design!{
    use makepad_widgets::base::*;
    use makepad_widgets::theme_desktop_dark::*; 
    use makepad_widgets::window::Window;
    use makepad_widgets::scroll_bars::ScrollBars;
    use makepad_widgets::view_ui::View;
    use crate::widgets::text::Text;
    
    App = {{App}} {
        ui: <Window> {
            window: {inner_size: vec2(400, 800), min_size: vec2(300, 400)}
            body = <View> {
                width: Fill, height: Fill,
                flow: Down,
                spacing: 20.0,
                
                show_bg: true,
                draw_bg: {
                    color: #1f1f1f
                }
                
                scroll_bars: <ScrollBars> {}
                
                // Text 1
                <Text> {
                    text: "Hello, Scrollable World!"
                    draw_text: {
                        color: #ffffff
                        text_style: { font_size: 24.0 }
                    }
                }
                
                // Text 2 (Long block)
                <Text> {
                    text: "This is a gallery view designed to test scrolling.\nWe need enough content to exceed the window height.\nMakepad handles scrolling automatically if we use ScrollBars."
                    draw_text: {
                        color: #cccccc
                        text_style: { font_size: 16.0 }
                    }
                }

                // Text 3
                <Text> { text: "Item 3" draw_text: { color: #fff } }
                <Text> { text: "Item 4" draw_text: { color: #fff } }
                <Text> { text: "Item 5" draw_text: { color: #fff } }
                <Text> { text: "Item 6" draw_text: { color: #fff } }
                <Text> { text: "Item 7" draw_text: { color: #fff } }
                <Text> { text: "Item 8" draw_text: { color: #fff } }
                <Text> { text: "Item 9" draw_text: { color: #fff } }
                <Text> { text: "Item 10" draw_text: { color: #fff } }
                
                // Text 4 (Bottom marker)
                <Text> {
                    text: "Bottom of Value\nYou should see this only after scrolling."
                    draw_text: {
                        color: #ffaa00
                        text_style: { font_size: 20.0 }
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
    fn live_register(cx: &mut Cx) {
        makepad_widgets::live_design(cx);
        crate::widgets::live_design(cx);
    }
}

impl AppMain for App {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event) {
        self.ui.handle_event(cx, event, &mut Scope::empty());
    }
}
