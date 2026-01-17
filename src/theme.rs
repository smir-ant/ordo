use makepad_widgets::*;

live_design! {
    use link::shaders::*;
    use makepad_draw::shader::std::*;
    
    pub THEME_FONT_REGULAR = {
        font_family: {
            base = font("Sans", 0.0, 0.0)  // Inter
        }
    }
    
    pub THEME_FONT_MONO = {
        font_family: {
            base = font("UbuntuMono", 0.0, 0.0)
        }
    }
    
    pub THEME_FONT_SIZE_BASE = 32.0
    
    // --- Button Theme Tokens (Neutral) ---
    pub THEME_BUTTON_BG_COLOR = #383838
    pub THEME_BUTTON_TEXT_COLOR = #ffffff
    pub THEME_BUTTON_BORDER_COLOR = #ffffff1a
    pub THEME_BUTTON_RADIUS = 4.0
    pub THEME_BUTTON_HEIGHT = 38.0
    pub THEME_BUTTON_FONT_SIZE = 14.0
}
