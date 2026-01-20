use makepad_widgets::*;

live_design! {
    use makepad_widgets::base::*;
    use makepad_widgets::theme_desktop_dark::*;
    use makepad_draw::shader::std::*;
    use makepad_widgets::view_ui::View; 
    
    pub Group = <View> {
        width: Fill, height: Fit
        flow: Down
        padding: 10.0
        spacing: 10.0
        
        show_bg: true
        draw_bg: {
            instance radius: 4.0
            
            fn pixel(self) -> vec4 {
                let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                sdf.box(
                    0.0,
                    0.0,
                    self.rect_size.x,
                    self.rect_size.y,
                    self.radius
                );
                return sdf.fill(self.color);
            }
            
            // Set default color
            color: #4d4d4d
        }
    }
}
