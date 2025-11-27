use super::{Plugin, PluginContext, PluginResult};
use image::GenericImageView;

pub struct SavePlugin;

impl SavePlugin {
    pub fn new() -> Self {
        Self
    }
}

impl Plugin for SavePlugin {
    fn id(&self) -> &str {
        "save"
    }
    
    fn name(&self) -> &str {
        "Save"
    }
    
    fn icon(&self) -> Option<&[u8]> {
        Some(include_bytes!("../../icons/save.png"))
    }
    
    fn on_click(&mut self, context: &PluginContext) -> PluginResult {
        let coords = match context.selection_coords {
            Some(c) => c,
            None => return PluginResult::Failure("No selection".to_string()),
        };
        
        let monitor = match &context.monitor {
            Some(m) => m,
            None => return PluginResult::Failure("No monitor available".to_string()),
        };
        
        // Capture screenshot from monitor
        let screenshot = match monitor.capture_image() {
            Ok(img) => img,
            Err(e) => return PluginResult::Failure(format!("Failed to capture screen: {}", e)),
        };
        
        // Crop screenshot to selection area
        let ((min_x, min_y), (max_x, max_y)) = coords;
        let width = max_x.saturating_sub(min_x);
        let height = max_y.saturating_sub(min_y);
        
        if width == 0 || height == 0 {
            return PluginResult::Failure("Invalid selection size".to_string());
        }
        
        let cropped = screenshot.view(min_x, min_y, width, height).to_image();
        
        // Show file save dialog
        let file_path = match rfd::FileDialog::new()
            .set_file_name("screenshot.png")
            .add_filter("PNG", &["png"])
            .add_filter("JPEG", &["jpg", "jpeg"])
            .save_file()
        {
            Some(path) => path,
            None => return PluginResult::Continue, // User cancelled
        };
        
        // Save image
        match cropped.save(&file_path) {
            Ok(_) => PluginResult::Exit, // 保存成功后退出
            Err(e) => PluginResult::Failure(format!("Failed to save image: {}", e)),
        }
    }
}

