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

    // Accent Color - Single value, variants computed via mix() in shaders
    pub THEME_COLOR_ACCENT = #FF5C39

    // --- Semantic Colors (Dark Theme) ---
    // Background hierarchy (dark to light)
    pub THEME_COLOR_BG_DARK = #1f1f1f       // App background
    pub THEME_COLOR_BG_PANEL = #181818      // Header, nav bar
    pub THEME_COLOR_BG_PRIMARY = #2a2a2a    // Cards, modals
    pub THEME_COLOR_BG_SECONDARY = #333     // Inputs, buttons
    pub THEME_COLOR_BG_TERTIARY = #444      // Hover states, borders
    pub THEME_COLOR_BG_ELEVATED = #4a4a4a   // Focus states

    // Text hierarchy
    pub THEME_COLOR_TEXT_PRIMARY = #fff
    pub THEME_COLOR_TEXT_SECONDARY = #888
    pub THEME_COLOR_TEXT_TERTIARY = #666
    pub THEME_COLOR_TEXT_DISABLED = #555

    // Stroke/border colors
    pub THEME_COLOR_STROKE_DARK = #222
    pub THEME_COLOR_STROKE_LIGHT = #444
    pub THEME_COLOR_STROKE_HOVER = #555

    // State colors
    pub THEME_COLOR_ERROR = #ff4444
    pub THEME_COLOR_SUCCESS = #44ff44
}
