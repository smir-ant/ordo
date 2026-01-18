use makepad_widgets::*;

live_design! {
    use link::shaders::*;
    use makepad_draw::shader::std::*;
    
    // --- Fonts ---
    pub THEME_FONT_REGULAR = {
        font_family: {
            latin = font("Sans", 0.0, 0.0)
        }
    }
    
    pub THEME_FONT_BOLD = {
        font_family: {
            latin = font("Sans", 0.0, 0.0)
        }
    }
    
    pub THEME_FONT_MONO = {
        font_family: {
            latin = font("Sans", 0.0, 0.0)
        }
    }
    
    pub THEME_FONT_SIZE_BASE = 14.0
    
    // --- Global Theme Colors ---
    
    // Theme Mode (1.0 = Dark, 0.0 = Light)
    pub THEME_IS_DARK = 1.0
    
    // Accent Color (Primary brand color)
    pub THEME_COLOR_ACCENT = #007AFF
}
