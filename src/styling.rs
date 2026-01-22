use makepad_widgets::*;

// This is the application styling file.
// We cannot use the name `theme.rs` or `link theme` because that namespace is reserved by Makepad internally (link::theme),
// causing conflicts when trying to override it directly in a way that merges cleanly.
// Therefore, we use `styling` as the distinct namespace for project-specific theme settings.
live_design! {
    link styling;
    
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
    
    // Accent Color (Primary brand color - Orange like DOW)
    pub THEME_COLOR_ACCENT = #FF5C39        // Main accent (DOW active fill bottom)
    pub THEME_COLOR_ACCENT_LIGHT = #FF7E60  // Lighter variant (DOW active fill top)
    pub THEME_COLOR_ACCENT_DARK = #A03010   // Darker variant (DOW stroke bottom)
    pub THEME_COLOR_ACCENT_STROKE_TOP = #FF8C70  // Stroke top gradient
}
