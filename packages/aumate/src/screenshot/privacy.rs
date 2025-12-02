//! Privacy tools for screenshot annotation
//!
//! Provides mosaic, blur, and smart erase effects to hide sensitive information.

use image::{ImageBuffer, Rgba};

/// Privacy tool type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum PrivacyTool {
    #[default]
    Mosaic,
    Blur,
    SmartErase,
}

/// Privacy region annotation
#[derive(Debug, Clone)]
pub struct PrivacyRegion {
    /// Bounding rect in logical coordinates
    pub rect: egui::Rect,
    /// Tool used
    pub tool: PrivacyTool,
    /// Block size for mosaic (pixels)
    pub block_size: u32,
    /// Blur radius
    pub blur_radius: u32,
}

impl PrivacyRegion {
    pub fn new(rect: egui::Rect, tool: PrivacyTool) -> Self {
        Self { rect, tool, block_size: 10, blur_radius: 5 }
    }

    /// Create mosaic region with custom block size
    pub fn mosaic(rect: egui::Rect, block_size: u32) -> Self {
        Self { rect, tool: PrivacyTool::Mosaic, block_size, blur_radius: 5 }
    }

    /// Create blur region with custom radius
    pub fn blur(rect: egui::Rect, blur_radius: u32) -> Self {
        Self { rect, tool: PrivacyTool::Blur, block_size: 10, blur_radius }
    }

    /// Create smart erase region
    pub fn smart_erase(rect: egui::Rect) -> Self {
        Self { rect, tool: PrivacyTool::SmartErase, block_size: 10, blur_radius: 5 }
    }
}

/// Apply mosaic effect to a region of the image
///
/// Pixelates the region by replacing pixels with block averages.
pub fn apply_mosaic(
    image: &mut ImageBuffer<Rgba<u8>, Vec<u8>>,
    x: u32,
    y: u32,
    w: u32,
    h: u32,
    block_size: u32,
) {
    let img_width = image.width();
    let img_height = image.height();

    // Clamp region to image bounds
    let x = x.min(img_width);
    let y = y.min(img_height);
    let w = w.min(img_width - x);
    let h = h.min(img_height - y);

    if w == 0 || h == 0 || block_size == 0 {
        return;
    }

    let block_size = block_size.max(1);

    // Process each block
    for by in (y..y + h).step_by(block_size as usize) {
        for bx in (x..x + w).step_by(block_size as usize) {
            // Calculate block bounds
            let block_end_x = (bx + block_size).min(x + w).min(img_width);
            let block_end_y = (by + block_size).min(y + h).min(img_height);

            // Calculate average color in block
            let mut r_sum = 0u32;
            let mut g_sum = 0u32;
            let mut b_sum = 0u32;
            let mut a_sum = 0u32;
            let mut count = 0u32;

            for py in by..block_end_y {
                for px in bx..block_end_x {
                    let pixel = image.get_pixel(px, py);
                    r_sum += pixel[0] as u32;
                    g_sum += pixel[1] as u32;
                    b_sum += pixel[2] as u32;
                    a_sum += pixel[3] as u32;
                    count += 1;
                }
            }

            if count > 0 {
                let avg = Rgba([
                    (r_sum / count) as u8,
                    (g_sum / count) as u8,
                    (b_sum / count) as u8,
                    (a_sum / count) as u8,
                ]);

                // Fill block with average color
                for py in by..block_end_y {
                    for px in bx..block_end_x {
                        image.put_pixel(px, py, avg);
                    }
                }
            }
        }
    }
}

/// Apply blur effect to a region of the image
///
/// Uses box blur approximation (faster than Gaussian).
/// Multiple passes approximate Gaussian blur.
pub fn apply_blur(
    image: &mut ImageBuffer<Rgba<u8>, Vec<u8>>,
    x: u32,
    y: u32,
    w: u32,
    h: u32,
    radius: u32,
) {
    let img_width = image.width();
    let img_height = image.height();

    // Clamp region to image bounds
    let x = x.min(img_width);
    let y = y.min(img_height);
    let w = w.min(img_width - x);
    let h = h.min(img_height - y);

    if w == 0 || h == 0 || radius == 0 {
        return;
    }

    // Apply 3 passes of box blur for Gaussian approximation
    for _ in 0..3 {
        horizontal_blur(image, x, y, w, h, radius);
        vertical_blur(image, x, y, w, h, radius);
    }
}

/// Horizontal box blur pass
fn horizontal_blur(
    image: &mut ImageBuffer<Rgba<u8>, Vec<u8>>,
    x: u32,
    y: u32,
    w: u32,
    h: u32,
    radius: u32,
) {
    let img_width = image.width();
    let kernel_size = (2 * radius + 1) as usize;

    // Create temporary buffer for one row
    let mut row_buffer = Vec::with_capacity(w as usize);

    for py in y..y + h {
        row_buffer.clear();

        // Blur each pixel in the row
        for px in x..x + w {
            let mut r_sum = 0u32;
            let mut g_sum = 0u32;
            let mut b_sum = 0u32;
            let mut a_sum = 0u32;
            let mut count = 0u32;

            // Sample within kernel radius
            let start_x = px.saturating_sub(radius);
            let end_x = (px + radius + 1).min(img_width);

            for sx in start_x..end_x {
                // Clamp to region bounds
                let sample_x = sx.clamp(x, x + w - 1);
                let pixel = image.get_pixel(sample_x, py);
                r_sum += pixel[0] as u32;
                g_sum += pixel[1] as u32;
                b_sum += pixel[2] as u32;
                a_sum += pixel[3] as u32;
                count += 1;
            }

            let avg = Rgba([
                (r_sum / count.max(1)) as u8,
                (g_sum / count.max(1)) as u8,
                (b_sum / count.max(1)) as u8,
                (a_sum / count.max(1)) as u8,
            ]);
            row_buffer.push(avg);
        }

        // Write back to image
        for (i, pixel) in row_buffer.iter().enumerate() {
            image.put_pixel(x + i as u32, py, *pixel);
        }
    }

    let _ = kernel_size; // Silence warning
}

/// Vertical box blur pass
fn vertical_blur(
    image: &mut ImageBuffer<Rgba<u8>, Vec<u8>>,
    x: u32,
    y: u32,
    w: u32,
    h: u32,
    radius: u32,
) {
    let img_height = image.height();

    // Create temporary buffer for one column
    let mut col_buffer = Vec::with_capacity(h as usize);

    for px in x..x + w {
        col_buffer.clear();

        // Blur each pixel in the column
        for py in y..y + h {
            let mut r_sum = 0u32;
            let mut g_sum = 0u32;
            let mut b_sum = 0u32;
            let mut a_sum = 0u32;
            let mut count = 0u32;

            // Sample within kernel radius
            let start_y = py.saturating_sub(radius);
            let end_y = (py + radius + 1).min(img_height);

            for sy in start_y..end_y {
                // Clamp to region bounds
                let sample_y = sy.clamp(y, y + h - 1);
                let pixel = image.get_pixel(px, sample_y);
                r_sum += pixel[0] as u32;
                g_sum += pixel[1] as u32;
                b_sum += pixel[2] as u32;
                a_sum += pixel[3] as u32;
                count += 1;
            }

            let avg = Rgba([
                (r_sum / count.max(1)) as u8,
                (g_sum / count.max(1)) as u8,
                (b_sum / count.max(1)) as u8,
                (a_sum / count.max(1)) as u8,
            ]);
            col_buffer.push(avg);
        }

        // Write back to image
        for (i, pixel) in col_buffer.iter().enumerate() {
            image.put_pixel(px, y + i as u32, *pixel);
        }
    }
}

/// Apply smart erase to a region
///
/// Detects the background color from the edges and fills the region.
pub fn apply_smart_erase(
    image: &mut ImageBuffer<Rgba<u8>, Vec<u8>>,
    x: u32,
    y: u32,
    w: u32,
    h: u32,
) {
    let img_width = image.width();
    let img_height = image.height();

    // Clamp region to image bounds
    let x = x.min(img_width);
    let y = y.min(img_height);
    let w = w.min(img_width - x);
    let h = h.min(img_height - y);

    if w == 0 || h == 0 {
        return;
    }

    // Sample colors from the border of the region
    let mut colors: Vec<Rgba<u8>> = Vec::new();

    // Sample top and bottom edges
    for px in x..x + w {
        colors.push(*image.get_pixel(px, y));
        if h > 1 {
            colors.push(*image.get_pixel(px, y + h - 1));
        }
    }

    // Sample left and right edges (excluding corners to avoid duplicates)
    for py in (y + 1)..(y + h).saturating_sub(1) {
        colors.push(*image.get_pixel(x, py));
        if w > 1 {
            colors.push(*image.get_pixel(x + w - 1, py));
        }
    }

    // Find the most common color (simple mode calculation)
    let bg_color = most_common_color(&colors);

    // Fill region with background color
    for py in y..y + h {
        for px in x..x + w {
            image.put_pixel(px, py, bg_color);
        }
    }
}

/// Find the most common color in a list
fn most_common_color(colors: &[Rgba<u8>]) -> Rgba<u8> {
    use std::collections::HashMap;

    if colors.is_empty() {
        return Rgba([255, 255, 255, 255]);
    }

    // Quantize colors to reduce unique values (group similar colors)
    let quantize = |c: &Rgba<u8>| -> (u8, u8, u8) {
        // Quantize to ~32 levels per channel
        ((c[0] / 8) * 8, (c[1] / 8) * 8, (c[2] / 8) * 8)
    };

    let mut counts: HashMap<(u8, u8, u8), (u32, Rgba<u8>)> = HashMap::new();

    for color in colors {
        let key = quantize(color);
        counts.entry(key).or_insert((0, *color)).0 += 1;
    }

    // Find the most common
    counts
        .values()
        .max_by_key(|(count, _)| *count)
        .map(|(_, color)| *color)
        .unwrap_or(Rgba([255, 255, 255, 255]))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mosaic() {
        let mut img = ImageBuffer::from_fn(20, 20, |x, y| {
            Rgba([(x * 10) as u8, (y * 10) as u8, 128, 255])
        });

        apply_mosaic(&mut img, 0, 0, 20, 20, 5);

        // After mosaic, 5x5 blocks should have same color
        let p00 = *img.get_pixel(0, 0);
        let p11 = *img.get_pixel(1, 1);
        let p44 = *img.get_pixel(4, 4);
        assert_eq!(p00, p11);
        assert_eq!(p00, p44);
    }

    #[test]
    fn test_most_common_color() {
        let colors = vec![
            Rgba([255, 0, 0, 255]),
            Rgba([255, 0, 0, 255]),
            Rgba([255, 0, 0, 255]),
            Rgba([0, 255, 0, 255]),
            Rgba([0, 0, 255, 255]),
        ];
        let result = most_common_color(&colors);
        // Red should be most common
        assert_eq!(result[0], 255);
        assert_eq!(result[1], 0);
        assert_eq!(result[2], 0);
    }
}
