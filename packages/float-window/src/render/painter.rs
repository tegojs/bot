//! Window painter - unified rendering interface

use crate::content::{Content, ImageDisplayOptions, ScaleMode, TextAlign, TextDisplayOptions};
use crate::effect::particle::ParticleStyle;
use crate::effect::ParticleSystem;
use crate::shape::WindowShape;
use egui::{Color32, Pos2, Rect, Stroke, StrokeKind, Vec2};

/// Window painter for rendering content
pub struct WindowPainter;

impl WindowPainter {
    /// Render content to the UI with shape masking
    pub fn render_content(ui: &mut egui::Ui, content: &Content, rect: Rect, shape: &WindowShape) {
        match content {
            Content::Image {
                data,
                width,
                height,
                options,
            } => {
                Self::render_image_with_shape(ui, data, *width, *height, options, rect, shape);
            }
            Content::Text { text, options } => {
                Self::render_text(ui, text, options, rect);
            }
            Content::Custom => {
                // Custom rendering is handled elsewhere
            }
        }
    }

    /// Render an image with shape masking
    fn render_image_with_shape(
        ui: &mut egui::Ui,
        data: &[u8],
        width: u32,
        height: u32,
        options: &ImageDisplayOptions,
        rect: Rect,
        shape: &WindowShape,
    ) {
        // For circle shapes, apply a circular mask to the image data
        let (masked_data, masked_width, masked_height) = match shape {
            WindowShape::Circle => {
                // Resize image to destination size and apply circle mask
                let dest_width = rect.width() as u32;
                let dest_height = rect.height() as u32;
                let masked = Self::apply_circle_mask(data, width, height, dest_width, dest_height);
                (masked, dest_width, dest_height)
            }
            _ => {
                // For non-circle shapes, use original data
                (data.to_vec(), width, height)
            }
        };

        // Draw background if specified (only for non-circle, since masked image handles transparency)
        if let Some(bg) = options.background_color {
            if !matches!(shape, WindowShape::Circle) {
                ui.painter().rect_filled(
                    rect,
                    0.0,
                    Color32::from_rgba_unmultiplied(bg[0], bg[1], bg[2], bg[3]),
                );
            }
        }

        // Create texture from (potentially masked) image data
        let texture_id = ui.ctx().load_texture(
            "content_image",
            egui::ColorImage::from_rgba_unmultiplied([masked_width as usize, masked_height as usize], &masked_data),
            egui::TextureOptions::LINEAR,
        );

        // For circle shapes with masking, always use the full rect (image is already resized)
        let dest_rect = if matches!(shape, WindowShape::Circle) {
            rect
        } else {
            // Calculate destination rect based on scale mode
            match options.scale_mode {
                ScaleMode::Fit => {
                    let aspect = width as f32 / height as f32;
                    let rect_aspect = rect.width() / rect.height();
                    if aspect > rect_aspect {
                        let h = rect.width() / aspect;
                        Rect::from_center_size(rect.center(), Vec2::new(rect.width(), h))
                    } else {
                        let w = rect.height() * aspect;
                        Rect::from_center_size(rect.center(), Vec2::new(w, rect.height()))
                    }
                }
                ScaleMode::Fill => {
                    let aspect = width as f32 / height as f32;
                    let rect_aspect = rect.width() / rect.height();
                    if aspect < rect_aspect {
                        let h = rect.width() / aspect;
                        Rect::from_center_size(rect.center(), Vec2::new(rect.width(), h))
                    } else {
                        let w = rect.height() * aspect;
                        Rect::from_center_size(rect.center(), Vec2::new(w, rect.height()))
                    }
                }
                ScaleMode::Stretch => rect,
                ScaleMode::Center => {
                    let size = Vec2::new(width as f32, height as f32);
                    Rect::from_center_size(rect.center(), size)
                }
            }
        };

        ui.painter().image(
            texture_id.id(),
            dest_rect,
            Rect::from_min_max(Pos2::ZERO, Pos2::new(1.0, 1.0)),
            Color32::WHITE,
        );
    }

    /// Apply a circular mask to image data, resizing to destination size
    fn apply_circle_mask(
        data: &[u8],
        src_width: u32,
        src_height: u32,
        dest_width: u32,
        dest_height: u32,
    ) -> Vec<u8> {
        let mut result = vec![0u8; (dest_width * dest_height * 4) as usize];

        let center_x = dest_width as f32 / 2.0;
        let center_y = dest_height as f32 / 2.0;
        let radius = center_x.min(center_y);

        for y in 0..dest_height {
            for x in 0..dest_width {
                let dest_idx = ((y * dest_width + x) * 4) as usize;

                // Calculate distance from center
                let dx = x as f32 - center_x;
                let dy = y as f32 - center_y;
                let distance = (dx * dx + dy * dy).sqrt();

                // Check if pixel is inside the circle
                if distance <= radius {
                    // Map destination coordinates to source coordinates
                    let src_x = (x as f32 * src_width as f32 / dest_width as f32) as u32;
                    let src_y = (y as f32 * src_height as f32 / dest_height as f32) as u32;
                    let src_x = src_x.min(src_width - 1);
                    let src_y = src_y.min(src_height - 1);
                    let src_idx = ((src_y * src_width + src_x) * 4) as usize;

                    // Copy RGBA values
                    if src_idx + 3 < data.len() {
                        result[dest_idx] = data[src_idx];
                        result[dest_idx + 1] = data[src_idx + 1];
                        result[dest_idx + 2] = data[src_idx + 2];
                        result[dest_idx + 3] = data[src_idx + 3];
                    }
                }
                // Pixels outside circle remain transparent (already 0)
            }
        }

        result
    }

    /// Render text
    fn render_text(ui: &mut egui::Ui, text: &str, options: &TextDisplayOptions, rect: Rect) {
        // Draw background if specified
        if let Some(bg) = options.background_color {
            ui.painter().rect_filled(
                rect,
                0.0,
                Color32::from_rgba_unmultiplied(bg[0], bg[1], bg[2], bg[3]),
            );
        }

        let color = Color32::from_rgba_unmultiplied(
            options.color[0],
            options.color[1],
            options.color[2],
            options.color[3],
        );

        let align = match options.align {
            TextAlign::Left => egui::Align::LEFT,
            TextAlign::Center => egui::Align::Center,
            TextAlign::Right => egui::Align::RIGHT,
        };

        let font_id = egui::FontId::proportional(options.font_size);

        if options.wrap {
            ui.painter().text(
                rect.left_top(),
                egui::Align2::LEFT_TOP,
                text,
                font_id,
                color,
            );
        } else {
            let pos = match align {
                egui::Align::LEFT => rect.left_center(),
                egui::Align::Center => rect.center(),
                egui::Align::RIGHT => rect.right_center(),
            };
            let anchor = match align {
                egui::Align::LEFT => egui::Align2::LEFT_CENTER,
                egui::Align::Center => egui::Align2::CENTER_CENTER,
                egui::Align::RIGHT => egui::Align2::RIGHT_CENTER,
            };
            ui.painter().text(pos, anchor, text, font_id, color);
        }
    }

    /// Render particle system
    pub fn render_particles(ui: &mut egui::Ui, system: &ParticleSystem, offset: Pos2) {
        let painter = ui.painter();

        for particle in system.particles() {
            // Skip dead particles
            if particle.alpha < 0.1 {
                continue;
            }

            // Fade by adjusting brightness, not alpha
            // This avoids compositor blending artifacts on transparent windows
            let fade = particle.alpha;
            let r = (particle.color[0] * fade * 255.0) as u8;
            let g = (particle.color[1] * fade * 255.0) as u8;
            let b = (particle.color[2] * fade * 255.0) as u8;

            // IMPORTANT: Use full alpha (255) to avoid compositor artifacts
            let color = Color32::from_rgba_unmultiplied(r, g, b, 255);

            match particle.style {
                ParticleStyle::Dot => {
                    // Skip tiny dots
                    if particle.size < 2.0 {
                        continue;
                    }

                    let pos = Pos2::new(
                        offset.x + particle.position.0,
                        offset.y + particle.position.1,
                    );

                    let size = particle.size.max(2.0);

                    // Use rect_filled with high rounding for dots
                    let rect = Rect::from_center_size(pos, Vec2::splat(size * 2.0));
                    painter.rect_filled(rect, size, color);
                }

                ParticleStyle::Line => {
                    // Draw line from previous position to current position
                    let from = Pos2::new(
                        offset.x + particle.prev_position.0,
                        offset.y + particle.prev_position.1,
                    );
                    let to = Pos2::new(
                        offset.x + particle.position.0,
                        offset.y + particle.position.1,
                    );

                    let stroke = Stroke::new(particle.size.max(1.0), color);
                    painter.line_segment([from, to], stroke);
                }

                ParticleStyle::Trail => {
                    // Draw trail as connected line segments with fading
                    if particle.trail.len() >= 2 {
                        let trail_len = particle.trail.len();
                        for i in 1..trail_len {
                            // Calculate fade for this segment (older = more faded)
                            let segment_fade = (i as f32 / trail_len as f32) * fade;
                            let sr = (particle.color[0] * segment_fade * 255.0) as u8;
                            let sg = (particle.color[1] * segment_fade * 255.0) as u8;
                            let sb = (particle.color[2] * segment_fade * 255.0) as u8;
                            let segment_color = Color32::from_rgba_unmultiplied(sr, sg, sb, 255);

                            // Line width also fades (thinner at tail)
                            let segment_width = particle.size * (i as f32 / trail_len as f32);

                            let from = Pos2::new(
                                offset.x + particle.trail[i - 1].0,
                                offset.y + particle.trail[i - 1].1,
                            );
                            let to = Pos2::new(
                                offset.x + particle.trail[i].0,
                                offset.y + particle.trail[i].1,
                            );

                            let stroke = Stroke::new(segment_width.max(0.5), segment_color);
                            painter.line_segment([from, to], stroke);
                        }

                        // Draw final segment to current position
                        if let Some(&last) = particle.trail.last() {
                            let from = Pos2::new(offset.x + last.0, offset.y + last.1);
                            let to = Pos2::new(
                                offset.x + particle.position.0,
                                offset.y + particle.position.1,
                            );
                            let stroke = Stroke::new(particle.size, color);
                            painter.line_segment([from, to], stroke);
                        }
                    }
                }
            }
        }
    }

    /// Apply shape clipping
    pub fn clip_to_shape(ui: &mut egui::Ui, shape: &WindowShape, rect: Rect) {
        match shape {
            WindowShape::Rectangle => {
                // No special clipping needed
            }
            WindowShape::Circle => {
                // egui doesn't have native circle clipping, so we use a rect clip
                // The actual circle shape is handled by rendering
                ui.set_clip_rect(rect);
            }
            WindowShape::Custom { .. } => {
                // Custom shapes are handled via alpha masking during render
                ui.set_clip_rect(rect);
            }
        }
    }

    /// Draw window border based on shape
    pub fn draw_shape_border(ui: &mut egui::Ui, shape: &WindowShape, rect: Rect, color: Color32, width: f32) {
        let painter = ui.painter();
        let stroke = Stroke::new(width, color);

        match shape {
            WindowShape::Rectangle => {
                painter.rect_stroke(rect, 0.0, stroke, StrokeKind::Outside);
            }
            WindowShape::Circle => {
                let radius = rect.width().min(rect.height()) / 2.0;
                painter.circle_stroke(rect.center(), radius, stroke);
            }
            WindowShape::Custom { .. } => {
                // For custom shapes, just draw rect border
                painter.rect_stroke(rect, 0.0, stroke, StrokeKind::Outside);
            }
        }
    }
}
