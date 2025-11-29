//! Window painter - unified rendering interface

use crate::content::{Content, ImageDisplayOptions, ScaleMode, TextAlign, TextDisplayOptions};
use crate::effect::ParticleSystem;
use crate::shape::WindowShape;
use egui::{Color32, Pos2, Rect, Stroke, StrokeKind, Vec2};

/// Window painter for rendering content
pub struct WindowPainter;

impl WindowPainter {
    /// Render content to the UI
    pub fn render_content(ui: &mut egui::Ui, content: &Content, rect: Rect) {
        match content {
            Content::Image {
                data,
                width,
                height,
                options,
            } => {
                Self::render_image(ui, data, *width, *height, options, rect);
            }
            Content::Text { text, options } => {
                Self::render_text(ui, text, options, rect);
            }
            Content::Custom => {
                // Custom rendering is handled elsewhere
            }
        }
    }

    /// Render an image
    fn render_image(
        ui: &mut egui::Ui,
        data: &[u8],
        width: u32,
        height: u32,
        options: &ImageDisplayOptions,
        rect: Rect,
    ) {
        // Draw background if specified
        if let Some(bg) = options.background_color {
            ui.painter().rect_filled(
                rect,
                0.0,
                Color32::from_rgba_unmultiplied(bg[0], bg[1], bg[2], bg[3]),
            );
        }

        // Create texture from image data
        let texture_id = ui.ctx().load_texture(
            "content_image",
            egui::ColorImage::from_rgba_unmultiplied([width as usize, height as usize], data),
            egui::TextureOptions::LINEAR,
        );

        // Calculate destination rect based on scale mode
        let dest_rect = match options.scale_mode {
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
        };

        ui.painter().image(
            texture_id.id(),
            dest_rect,
            Rect::from_min_max(Pos2::ZERO, Pos2::new(1.0, 1.0)),
            Color32::WHITE,
        );
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
            let pos = Pos2::new(
                offset.x + particle.position.0,
                offset.y + particle.position.1,
            );

            let color = Color32::from_rgba_unmultiplied(
                (particle.color[0] * 255.0) as u8,
                (particle.color[1] * 255.0) as u8,
                (particle.color[2] * 255.0) as u8,
                (particle.alpha * 255.0) as u8,
            );

            painter.circle_filled(pos, particle.size, color);
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
