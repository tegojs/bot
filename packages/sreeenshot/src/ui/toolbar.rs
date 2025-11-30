use glam::Vec2;
use image;

pub struct ToolbarButton {
    pub id: String,
    pub x: f32,
    pub y: f32,
    #[allow(dead_code)]
    pub width: f32,
    #[allow(dead_code)]
    pub height: f32,
    pub icon: Option<Vec<u8>>, // PNG icon data
}

pub struct Toolbar {
    pub buttons: Vec<ToolbarButton>,
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
}

impl Toolbar {
    pub fn new(selection_x: f32, selection_y: f32, selection_width: f32, selection_height: f32, screen_height: f32, plugin_info: &[crate::plugins::PluginInfo]) -> Self {
        // Toolbar appears below the selection area
        // Calculate icon size from first plugin icon (assuming all icons are same size)
        // Default icon size is typically 24x24 or 32x32, we'll use 32x32 as default
        let default_icon_size = 32.0;
        let icon_size = if let Some(plugin) = plugin_info.first() {
            if let Some(icon_data) = &plugin.icon {
                // Try to decode icon to get actual size
                if let Ok(img) = image::load_from_memory(icon_data) {
                    img.width() as f32
                } else {
                    default_icon_size
                }
            } else {
                default_icon_size
            }
        } else {
            default_icon_size
        };
        
        // Toolbar height should match icon size with minimal padding
        // Use 8px padding (4px top + 4px bottom) for a tight fit
        let toolbar_padding = 8.0;
        let toolbar_height = icon_size + toolbar_padding;
        let button_width = icon_size + 8.0; // Icon + 4px padding on each side
        let button_height = icon_size + 8.0;
        let button_spacing = 8.0; // Space between buttons
        
        let mut buttons = Vec::new();
        let mut current_x = 8.0; // Start with padding
        
        // Create buttons from enabled plugins
        for plugin in plugin_info {
            buttons.push(ToolbarButton {
                id: plugin.id.clone(),
                x: current_x,
                y: 4.0, // 4px padding from top
                width: button_width,
                height: button_height,
                icon: plugin.icon.clone(),
            });
            current_x += button_width + button_spacing;
        }
        
        // Calculate toolbar width based on button count
        let toolbar_width = if buttons.is_empty() {
            0.0
        } else {
            current_x + 8.0 // Add right padding
        };
        
        // Position toolbar below selection, centered horizontally
        // If there's not enough space below, position at the bottom of selection
        let toolbar_x = selection_x + (selection_width - toolbar_width) / 2.0;
        
        // Calculate space below selection
        let space_below = screen_height - (selection_y + selection_height);
        let toolbar_y = if space_below >= toolbar_height + 10.0 {
            selection_y + selection_height + 10.0 // 10px gap below selection
        } else {
            selection_y + selection_height - toolbar_height // At bottom of selection
        };
        
        // Adjust button positions relative to toolbar
        for button in &mut buttons {
            button.x += toolbar_x;
            button.y += toolbar_y;
        }
        
        Self {
            buttons,
            x: toolbar_x,
            y: toolbar_y,
            width: toolbar_width,
            height: toolbar_height,
        }
    }
    
    #[allow(dead_code)]
    pub fn check_click(&self, mouse_pos: Vec2) -> Option<&str> {
        for button in &self.buttons {
            if mouse_pos.x >= button.x
                && mouse_pos.x <= button.x + button.width
                && mouse_pos.y >= button.y
                && mouse_pos.y <= button.y + button.height
            {
                return Some(&button.id);
            }
        }
        None
    }
}
