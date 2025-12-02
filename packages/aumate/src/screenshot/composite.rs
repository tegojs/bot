//! Composite module - renders annotations onto an ImageBuffer
//!
//! This module provides functionality to composite annotations (strokes, arrows,
//! shapes, etc.) onto a base screenshot image, producing a final image that
//! includes all user-drawn content.
//!
//! # Coordinate Systems
//!
//! - **Logical pixels**: Used by egui/UI, independent of screen density
//! - **Physical pixels**: Actual screen pixels, used by ImageBuffer
//! - **scale_factor**: physical / logical (e.g., 2.0 for Retina displays)
//!
//! All annotation coordinates are stored in logical pixels and must be converted
//! to physical pixels before rendering to the ImageBuffer.

use egui::{Color32, Pos2};
use image::{ImageBuffer, Rgba};

use super::stroke::{
    Annotations, Arrow, FillMode, Highlighter, Polyline, SequenceMarker, Shape, ShapeType, Stroke,
    StrokeStyle,
};

/// Composite annotations onto the selected region of the screenshot
///
/// # Arguments
/// * `base` - The full screenshot ImageBuffer
/// * `annotations` - All annotations to render
/// * `selection` - Selection bounds in physical pixels: ((min_x, min_y), (max_x, max_y))
/// * `selection_logical` - Selection bounds in logical pixels for coordinate conversion
/// * `scale_factor` - DPI scale factor (physical / logical)
///
/// # Returns
/// A new ImageBuffer containing only the selected region with annotations composited
pub fn composite_annotations(
    base: &ImageBuffer<Rgba<u8>, Vec<u8>>,
    annotations: &Annotations,
    selection: ((u32, u32), (u32, u32)),
    selection_logical: ((f32, f32), (f32, f32)),
    scale_factor: f32,
) -> ImageBuffer<Rgba<u8>, Vec<u8>> {
    let ((x1, y1), (x2, y2)) = selection;
    let width = x2.saturating_sub(x1);
    let height = y2.saturating_sub(y1);

    if width == 0 || height == 0 {
        return ImageBuffer::new(0, 0);
    }

    // First, extract the selected region from the base image
    let mut result = ImageBuffer::new(width, height);
    for y in 0..height {
        for x in 0..width {
            let src_x = x1 + x;
            let src_y = y1 + y;
            if src_x < base.width() && src_y < base.height() {
                let pixel = base.get_pixel(src_x, src_y);
                result.put_pixel(x, y, *pixel);
            }
        }
    }

    // Create compositor context for drawing
    let ctx = CompositorContext { selection_logical, scale_factor };

    // Draw highlighters first (they go under everything else)
    for highlighter in &annotations.highlighters {
        draw_highlighter(&mut result, highlighter, &ctx);
    }

    // Draw shapes
    for shape in &annotations.shapes {
        draw_shape(&mut result, shape, &ctx);
    }

    // Draw polylines
    for polyline in &annotations.polylines {
        draw_polyline(&mut result, polyline, &ctx);
    }

    // Draw freehand strokes
    for stroke in &annotations.strokes {
        draw_stroke(&mut result, stroke, &ctx);
    }

    // Draw arrows
    for arrow in &annotations.arrows {
        draw_arrow(&mut result, arrow, &ctx);
    }

    // Draw sequence markers on top
    for marker in &annotations.markers {
        draw_sequence_marker(&mut result, marker, &ctx);
    }

    result
}

/// Context for coordinate conversion during compositing
struct CompositorContext {
    /// Selection bounds in logical pixels
    selection_logical: ((f32, f32), (f32, f32)),
    /// DPI scale factor
    scale_factor: f32,
}

impl CompositorContext {
    /// Convert a logical position to physical pixels relative to the selection
    fn to_physical(&self, pos: Pos2) -> (i32, i32) {
        let (min_x, min_y) = self.selection_logical.0;
        let relative_x = (pos.x - min_x) * self.scale_factor;
        let relative_y = (pos.y - min_y) * self.scale_factor;
        (relative_x.round() as i32, relative_y.round() as i32)
    }

    /// Scale a value from logical to physical pixels
    fn scale(&self, value: f32) -> f32 {
        value * self.scale_factor
    }
}

/// Convert Color32 to Rgba<u8>
fn color_to_rgba(color: Color32) -> Rgba<u8> {
    Rgba([color.r(), color.g(), color.b(), color.a()])
}

/// Blend a pixel with alpha compositing
fn blend_pixel(image: &mut ImageBuffer<Rgba<u8>, Vec<u8>>, x: u32, y: u32, color: Rgba<u8>) {
    if x >= image.width() || y >= image.height() {
        return;
    }

    let base = image.get_pixel(x, y);
    let alpha = color.0[3] as f32 / 255.0;

    if alpha >= 1.0 {
        image.put_pixel(x, y, color);
    } else if alpha > 0.0 {
        let inv_alpha = 1.0 - alpha;
        let blended = Rgba([
            (color.0[0] as f32 * alpha + base.0[0] as f32 * inv_alpha) as u8,
            (color.0[1] as f32 * alpha + base.0[1] as f32 * inv_alpha) as u8,
            (color.0[2] as f32 * alpha + base.0[2] as f32 * inv_alpha) as u8,
            255,
        ]);
        image.put_pixel(x, y, blended);
    }
}

/// Draw a thick line using Bresenham's algorithm with thickness
#[allow(clippy::too_many_arguments)]
fn draw_thick_line(
    image: &mut ImageBuffer<Rgba<u8>, Vec<u8>>,
    x1: i32,
    y1: i32,
    x2: i32,
    y2: i32,
    thickness: f32,
    color: Rgba<u8>,
    _style: StrokeStyle,
) {
    let radius = (thickness / 2.0).max(0.5);
    let radius_i = radius.ceil() as i32;

    // Use Bresenham's line algorithm
    let dx = (x2 - x1).abs();
    let dy = (y2 - y1).abs();
    let sx = if x1 < x2 { 1 } else { -1 };
    let sy = if y1 < y2 { 1 } else { -1 };
    let mut err = dx - dy;

    let mut x = x1;
    let mut y = y1;

    let mut step = 0;

    loop {
        // Draw a filled circle at each point for thickness
        for dy_off in -radius_i..=radius_i {
            for dx_off in -radius_i..=radius_i {
                let dist_sq = (dx_off * dx_off + dy_off * dy_off) as f32;
                if dist_sq <= radius * radius {
                    let px = x + dx_off;
                    let py = y + dy_off;
                    if px >= 0 && py >= 0 {
                        // Apply style (dashed/dotted)
                        let should_draw = match _style {
                            StrokeStyle::Solid => true,
                            StrokeStyle::Dashed => (step / 8) % 2 == 0,
                            StrokeStyle::Dotted => (step / 4) % 2 == 0,
                        };
                        if should_draw {
                            blend_pixel(image, px as u32, py as u32, color);
                        }
                    }
                }
            }
        }

        if x == x2 && y == y2 {
            break;
        }

        let e2 = 2 * err;
        if e2 > -dy {
            err -= dy;
            x += sx;
        }
        if e2 < dx {
            err += dx;
            y += sy;
        }
        step += 1;
    }
}

/// Draw a freehand stroke onto the image
fn draw_stroke(
    image: &mut ImageBuffer<Rgba<u8>, Vec<u8>>,
    stroke: &Stroke,
    ctx: &CompositorContext,
) {
    if stroke.points.len() < 2 {
        return;
    }

    let color = color_to_rgba(stroke.settings.color);
    let thickness = ctx.scale(stroke.settings.width);

    for window in stroke.points.windows(2) {
        let (x1, y1) = ctx.to_physical(window[0]);
        let (x2, y2) = ctx.to_physical(window[1]);
        draw_thick_line(image, x1, y1, x2, y2, thickness, color, stroke.settings.style);
    }
}

/// Draw an arrow onto the image
fn draw_arrow(image: &mut ImageBuffer<Rgba<u8>, Vec<u8>>, arrow: &Arrow, ctx: &CompositorContext) {
    let color = color_to_rgba(arrow.settings.color);
    let thickness = ctx.scale(arrow.settings.width);

    let (x1, y1) = ctx.to_physical(arrow.start);
    let (x2, y2) = ctx.to_physical(arrow.end);

    // Draw the main line
    draw_thick_line(image, x1, y1, x2, y2, thickness, color, arrow.settings.style);

    // Draw arrowhead
    let arrow_length = (arrow.length() * ctx.scale_factor).max(10.0);
    let head_length = (arrow_length * 0.25).min(20.0 * ctx.scale_factor);
    let head_width = head_length * 0.6;

    let dir = arrow.direction();
    let perp = egui::vec2(-dir.y, dir.x);

    // Arrowhead points
    let tip = egui::pos2(x2 as f32, y2 as f32);
    let back = tip - dir * head_length;
    let left = back + perp * head_width;
    let right = back - perp * head_width;

    // Draw filled arrowhead triangle
    draw_filled_triangle(
        image,
        (tip.x as i32, tip.y as i32),
        (left.x as i32, left.y as i32),
        (right.x as i32, right.y as i32),
        color,
    );
}

/// Draw a filled triangle
fn draw_filled_triangle(
    image: &mut ImageBuffer<Rgba<u8>, Vec<u8>>,
    p1: (i32, i32),
    p2: (i32, i32),
    p3: (i32, i32),
    color: Rgba<u8>,
) {
    // Simple scanline fill
    let min_y = p1.1.min(p2.1).min(p3.1).max(0);
    let max_y = p1.1.max(p2.1).max(p3.1).min(image.height() as i32 - 1);

    for y in min_y..=max_y {
        let mut intersections = Vec::new();

        // Check intersection with each edge
        for (a, b) in [(p1, p2), (p2, p3), (p3, p1)] {
            if (a.1 <= y && b.1 > y) || (b.1 <= y && a.1 > y) {
                let t = (y - a.1) as f32 / (b.1 - a.1) as f32;
                let x = a.0 as f32 + t * (b.0 - a.0) as f32;
                intersections.push(x as i32);
            }
        }

        intersections.sort();

        if intersections.len() >= 2 {
            let x1 = intersections[0].max(0) as u32;
            let x2 = intersections[1].min(image.width() as i32 - 1) as u32;
            for x in x1..=x2 {
                blend_pixel(image, x, y as u32, color);
            }
        }
    }
}

/// Draw a shape (rectangle or ellipse) onto the image
fn draw_shape(image: &mut ImageBuffer<Rgba<u8>, Vec<u8>>, shape: &Shape, ctx: &CompositorContext) {
    let color = color_to_rgba(shape.settings.color);
    let thickness = ctx.scale(shape.settings.width);

    let (min_x, min_y) = ctx.to_physical(shape.rect.min);
    let (max_x, max_y) = ctx.to_physical(shape.rect.max);

    match shape.shape_type {
        ShapeType::Rectangle => {
            if shape.fill_mode == FillMode::Filled {
                draw_filled_rect(image, min_x, min_y, max_x, max_y, color);
            } else {
                draw_rect_outline(image, min_x, min_y, max_x, max_y, thickness, color);
            }
        }
        ShapeType::Ellipse => {
            let cx = (min_x + max_x) / 2;
            let cy = (min_y + max_y) / 2;
            let rx = (max_x - min_x).abs() / 2;
            let ry = (max_y - min_y).abs() / 2;

            if shape.fill_mode == FillMode::Filled {
                draw_filled_ellipse(image, cx, cy, rx, ry, color);
            } else {
                draw_ellipse_outline(image, cx, cy, rx, ry, thickness, color);
            }
        }
    }
}

/// Draw a filled rectangle
fn draw_filled_rect(
    image: &mut ImageBuffer<Rgba<u8>, Vec<u8>>,
    x1: i32,
    y1: i32,
    x2: i32,
    y2: i32,
    color: Rgba<u8>,
) {
    let min_x = x1.min(x2).max(0) as u32;
    let max_x = x1.max(x2).min(image.width() as i32 - 1) as u32;
    let min_y = y1.min(y2).max(0) as u32;
    let max_y = y1.max(y2).min(image.height() as i32 - 1) as u32;

    for y in min_y..=max_y {
        for x in min_x..=max_x {
            blend_pixel(image, x, y, color);
        }
    }
}

/// Draw a rectangle outline
fn draw_rect_outline(
    image: &mut ImageBuffer<Rgba<u8>, Vec<u8>>,
    x1: i32,
    y1: i32,
    x2: i32,
    y2: i32,
    thickness: f32,
    color: Rgba<u8>,
) {
    // Draw four edges
    draw_thick_line(image, x1, y1, x2, y1, thickness, color, StrokeStyle::Solid); // Top
    draw_thick_line(image, x2, y1, x2, y2, thickness, color, StrokeStyle::Solid); // Right
    draw_thick_line(image, x2, y2, x1, y2, thickness, color, StrokeStyle::Solid); // Bottom
    draw_thick_line(image, x1, y2, x1, y1, thickness, color, StrokeStyle::Solid); // Left
}

/// Draw a filled ellipse using midpoint algorithm
fn draw_filled_ellipse(
    image: &mut ImageBuffer<Rgba<u8>, Vec<u8>>,
    cx: i32,
    cy: i32,
    rx: i32,
    ry: i32,
    color: Rgba<u8>,
) {
    if rx <= 0 || ry <= 0 {
        return;
    }

    let rx2 = (rx * rx) as i64;
    let ry2 = (ry * ry) as i64;

    for y in -ry..=ry {
        // Calculate x range for this y using ellipse equation
        let y2 = (y * y) as i64;
        let x_range_sq = rx2 - (rx2 * y2) / ry2;
        if x_range_sq < 0 {
            continue;
        }
        let x_range = (x_range_sq as f64).sqrt() as i32;

        let py = cy + y;
        if py < 0 || py >= image.height() as i32 {
            continue;
        }

        for x in -x_range..=x_range {
            let px = cx + x;
            if px >= 0 && px < image.width() as i32 {
                blend_pixel(image, px as u32, py as u32, color);
            }
        }
    }
}

/// Draw an ellipse outline
fn draw_ellipse_outline(
    image: &mut ImageBuffer<Rgba<u8>, Vec<u8>>,
    cx: i32,
    cy: i32,
    rx: i32,
    ry: i32,
    thickness: f32,
    color: Rgba<u8>,
) {
    if rx <= 0 || ry <= 0 {
        return;
    }

    // Draw ellipse using parametric equations
    let steps = ((rx + ry) * 2).max(32);
    let mut prev: Option<(i32, i32)> = None;

    for i in 0..=steps {
        let t = (i as f32 / steps as f32) * std::f32::consts::TAU;
        let x = cx + (rx as f32 * t.cos()) as i32;
        let y = cy + (ry as f32 * t.sin()) as i32;

        if let Some((px, py)) = prev {
            draw_thick_line(image, px, py, x, y, thickness, color, StrokeStyle::Solid);
        }
        prev = Some((x, y));
    }
}

/// Draw a polyline onto the image
fn draw_polyline(
    image: &mut ImageBuffer<Rgba<u8>, Vec<u8>>,
    polyline: &Polyline,
    ctx: &CompositorContext,
) {
    if polyline.points.len() < 2 {
        return;
    }

    let color = color_to_rgba(polyline.settings.color);
    let thickness = ctx.scale(polyline.settings.width);

    for window in polyline.points.windows(2) {
        let (x1, y1) = ctx.to_physical(window[0]);
        let (x2, y2) = ctx.to_physical(window[1]);
        draw_thick_line(image, x1, y1, x2, y2, thickness, color, polyline.settings.style);
    }

    // If closed, connect last to first
    if polyline.closed && polyline.points.len() >= 3 {
        let (x1, y1) = ctx.to_physical(*polyline.points.last().unwrap());
        let (x2, y2) = ctx.to_physical(polyline.points[0]);
        draw_thick_line(image, x1, y1, x2, y2, thickness, color, polyline.settings.style);
    }
}

/// Draw a highlighter (semi-transparent rectangle) onto the image
fn draw_highlighter(
    image: &mut ImageBuffer<Rgba<u8>, Vec<u8>>,
    highlighter: &Highlighter,
    ctx: &CompositorContext,
) {
    let color = color_to_rgba(highlighter.color);

    let (min_x, min_y) = ctx.to_physical(highlighter.rect.min);
    let (max_x, max_y) = ctx.to_physical(highlighter.rect.max);

    draw_filled_rect(image, min_x, min_y, max_x, max_y, color);
}

/// Draw a sequence marker onto the image
fn draw_sequence_marker(
    image: &mut ImageBuffer<Rgba<u8>, Vec<u8>>,
    marker: &SequenceMarker,
    ctx: &CompositorContext,
) {
    let (cx, cy) = ctx.to_physical(marker.pos);
    let radius = ctx.scale(marker.radius) as i32;

    // Draw filled circle background
    let bg_color = color_to_rgba(marker.color);
    draw_filled_ellipse(image, cx, cy, radius, radius, bg_color);

    // Draw white text (simplified - just draw a small centered dot for now)
    // Full text rendering would require a font rasterizer
    // For a proper implementation, consider using rusttype or ab_glyph
    let text_color = color_to_rgba(Color32::WHITE);
    let text_radius = (radius / 3).max(1);
    draw_filled_ellipse(image, cx, cy, text_radius, text_radius, text_color);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_composite_empty_annotations() {
        let base = ImageBuffer::from_pixel(100, 100, Rgba([255, 0, 0, 255]));
        let annotations = Annotations::new();
        let result = composite_annotations(
            &base,
            &annotations,
            ((10, 10), (50, 50)),
            ((10.0, 10.0), (50.0, 50.0)),
            1.0,
        );

        assert_eq!(result.width(), 40);
        assert_eq!(result.height(), 40);
        // Should be red (base color)
        assert_eq!(result.get_pixel(0, 0), &Rgba([255, 0, 0, 255]));
    }

    #[test]
    fn test_blend_pixel_alpha() {
        let mut image = ImageBuffer::from_pixel(10, 10, Rgba([255, 0, 0, 255]));
        // Blend 50% transparent blue
        blend_pixel(&mut image, 5, 5, Rgba([0, 0, 255, 128]));

        let pixel = image.get_pixel(5, 5);
        // Should be roughly purple (127, 0, 127, 255)
        assert!(pixel.0[0] > 100 && pixel.0[0] < 150); // Some red
        assert!(pixel.0[2] > 100 && pixel.0[2] < 150); // Some blue
    }
}
