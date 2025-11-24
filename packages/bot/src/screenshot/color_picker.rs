// Color picker functionality

use super::capture::capture_screen_region;
use super::types::*;

/// Get pixel color at specific coordinates
pub async fn get_pixel_color_at(x: u32, y: u32) -> Result<ColorInfo, String> {
    // Capture 1x1 region at the specified position
    let region = ScreenRegion { x: x as i32, y: y as i32, width: 1, height: 1 };

    let result = capture_screen_region(Some(region)).await?;

    // Decode the PNG to get pixel data
    let img = image::load_from_memory(&result.image)
        .map_err(|e| format!("Failed to decode image: {}", e))?;
    let rgba = img.to_rgba8();

    // Get pixel at (0, 0) since we captured a 1x1 region
    if let Some(pixel) = rgba.get_pixel_checked(0, 0) {
        let r = pixel[0];
        let g = pixel[1];
        let b = pixel[2];
        let a = pixel[3] as f32 / 255.0;

        Ok(ColorInfo {
            rgb: RgbColor { r, g, b },
            rgba: RgbaColor { r, g, b, a },
            hex: format!("#{:02X}{:02X}{:02X}", r, g, b),
            hsl: rgb_to_hsl(r, g, b),
            position: Position { x, y },
        })
    } else {
        Err("Failed to read pixel color".to_string())
    }
}

/// Start interactive color picker
pub async fn start_color_picker(_options: ColorPickerOptions) -> Result<ColorInfo, String> {
    // This will be implemented with egui + winit
    // For now, return a placeholder error
    Err("Interactive color picker not yet implemented".to_string())
}

/// Convert RGB to HSL
fn rgb_to_hsl(r: u8, g: u8, b: u8) -> HslColor {
    let r = r as f32 / 255.0;
    let g = g as f32 / 255.0;
    let b = b as f32 / 255.0;

    let max = r.max(g).max(b);
    let min = r.min(g).min(b);
    let delta = max - min;

    let l = (max + min) / 2.0;

    let (h, s) = if delta < 0.00001 {
        (0.0, 0.0)
    } else {
        let s = if l < 0.5 { delta / (max + min) } else { delta / (2.0 - max - min) };

        let h = if (max - r).abs() < 0.00001 {
            ((g - b) / delta + if g < b { 6.0 } else { 0.0 }) / 6.0
        } else if (max - g).abs() < 0.00001 {
            ((b - r) / delta + 2.0) / 6.0
        } else {
            ((r - g) / delta + 4.0) / 6.0
        };

        (h * 360.0, s * 100.0)
    };

    HslColor { h, s, l: l * 100.0 }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rgb_to_hsl_white() {
        let hsl = rgb_to_hsl(255, 255, 255);
        assert!((hsl.l - 100.0).abs() < 1.0);
        assert!((hsl.s - 0.0).abs() < 1.0);
    }

    #[test]
    fn test_rgb_to_hsl_black() {
        let hsl = rgb_to_hsl(0, 0, 0);
        assert!((hsl.l - 0.0).abs() < 1.0);
        assert!((hsl.s - 0.0).abs() < 1.0);
    }

    #[test]
    fn test_rgb_to_hsl_red() {
        let hsl = rgb_to_hsl(255, 0, 0);
        assert!((hsl.h - 0.0).abs() < 1.0);
        assert!((hsl.s - 100.0).abs() < 1.0);
        assert!((hsl.l - 50.0).abs() < 1.0);
    }
}
