use egui::ColorImage;
use image::ImageBuffer;
use image::Rgba;

/// 加载图标图像为 egui 纹理
pub fn load_icon_image(icon_data: &[u8]) -> anyhow::Result<ColorImage> {
    let img = image::load_from_memory(icon_data)?;
    let rgba = img.to_rgba8();
    let size = [rgba.width() as usize, rgba.height() as usize];
    let pixels = rgba.as_raw();
    
    // Convert RGBA to Color32
    let color_image = ColorImage::from_rgba_unmultiplied(size, pixels);
    Ok(color_image)
}

/// 将截图图像转换为 egui 纹理
pub fn screenshot_to_color_image(screenshot: &ImageBuffer<Rgba<u8>, Vec<u8>>) -> ColorImage {
    let screenshot_rgba = screenshot.as_raw();
    let screenshot_size = [screenshot.width() as usize, screenshot.height() as usize];
    ColorImage::from_rgba_unmultiplied(screenshot_size, screenshot_rgba)
}

