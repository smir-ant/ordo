use makepad_widgets::*;

live_design! {
    // link widgets; // Linking widgets causes conflict with View name if we redefine it
    // use link::widgets::*;
    
    use makepad_widgets::base::*;
    use makepad_widgets::theme_desktop_dark::*; 
    use makepad_widgets::view_ui::View as BaseView;
    
    // Custom View wrapper
    // Defaults to Fill/Fill to ensure full usage of available space
    pub View = <BaseView> {
        width: Fill, height: Fill
    }
}

pub use makepad_widgets::View;
