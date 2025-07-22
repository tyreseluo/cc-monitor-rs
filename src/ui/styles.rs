use makepad_widgets::*;

live_design! {
    use link::theme::*;
    use link::shaders::*;
    use link::widgets::*;
    
    // Text styles
    pub TEXT_TITLE = <THEME_FONT_BOLD>{
        font_size: (20.0),
        brightness: 1.1
    }
    
    pub TEXT_HEADING = <THEME_FONT_REGULAR>{
        font_size: (16.0),
        brightness: 1.0
    }
    
    pub TEXT_NORMAL = <THEME_FONT_REGULAR>{
        font_size: (14.0),
        brightness: 0.9
    }
    
    pub TEXT_SMALL = <THEME_FONT_REGULAR>{
        font_size: (12.0),
        brightness: 0.8
    }
}