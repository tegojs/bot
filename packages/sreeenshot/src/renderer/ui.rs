use egui::{Area, Frame, Rounding, Stroke, Color32, Pos2, Vec2};
use egui::epaint::Shadow;
use std::cell::RefCell;
use std::rc::Rc;
use image::ImageBuffer;
use image::Rgba;

use crate::ui::Toolbar;
use super::texture;

/// 渲染截图背景
pub fn render_screenshot(
    ctx: &egui::Context,
    screenshot_texture_id: egui::TextureId,
    screenshot: &ImageBuffer<Rgba<u8>, Vec<u8>>,
) {
    egui::CentralPanel::default()
        .frame(Frame::none())
        .show(ctx, |ui| {
            let pixels_per_point = ctx.pixels_per_point();
            let screenshot_texel_size = Vec2::new(
                screenshot.width() as f32,
                screenshot.height() as f32,
            );
            ui.add(egui::Image::new(egui::load::SizedTexture {
                id: screenshot_texture_id,
                size: screenshot_texel_size,
            })
            .fit_to_original_size(1.0 / pixels_per_point));
        });
}

/// 渲染选择区域的遮罩和边框
pub fn render_selection_mask(
    ctx: &egui::Context,
    selection: (f32, f32, f32, f32),
    screen_rect: egui::Rect,
    show_info: bool,
) {
    let (sel_x, sel_y, sel_width, sel_height) = selection;
    let selection_rect = egui::Rect::from_min_size(
        Pos2::new(sel_x, sel_y),
        Vec2::new(sel_width, sel_height),
    );

    let painter = ctx.layer_painter(egui::LayerId::background());
    let dark_color = Color32::from_rgba_unmultiplied(0, 0, 0, 180);
    
    // 绘制选择区域周围的暗色遮罩（4个矩形）
    if selection_rect.top() > 0.0 {
        painter.rect_filled(
            egui::Rect::from_min_max(
                Pos2::new(0.0, 0.0),
                Pos2::new(screen_rect.right(), selection_rect.top()),
            ),
            0.0,
            dark_color,
        );
    }
    
    if selection_rect.bottom() < screen_rect.bottom() {
        painter.rect_filled(
            egui::Rect::from_min_max(
                Pos2::new(0.0, selection_rect.bottom()),
                Pos2::new(screen_rect.right(), screen_rect.bottom()),
            ),
            0.0,
            dark_color,
        );
    }
    
    if selection_rect.left() > 0.0 {
        painter.rect_filled(
            egui::Rect::from_min_max(
                Pos2::new(0.0, selection_rect.top()),
                Pos2::new(selection_rect.left(), selection_rect.bottom()),
            ),
            0.0,
            dark_color,
        );
    }
    
    if selection_rect.right() < screen_rect.right() {
        painter.rect_filled(
            egui::Rect::from_min_max(
                Pos2::new(selection_rect.right(), selection_rect.top()),
                Pos2::new(screen_rect.right(), selection_rect.bottom()),
            ),
            0.0,
            dark_color,
        );
    }

    // 绘制选择区域边框（蓝色）
    let border_color = Color32::from_rgb(0, 122, 255);
    let border_width = 2.0;
    
    painter.rect_filled(
        egui::Rect::from_min_max(
            Pos2::new(selection_rect.left(), selection_rect.top() - border_width),
            Pos2::new(selection_rect.right(), selection_rect.top()),
        ),
        0.0,
        border_color,
    );
    
    painter.rect_filled(
        egui::Rect::from_min_max(
            Pos2::new(selection_rect.left(), selection_rect.bottom()),
            Pos2::new(selection_rect.right(), selection_rect.bottom() + border_width),
        ),
        0.0,
        border_color,
    );
    
    painter.rect_filled(
        egui::Rect::from_min_max(
            Pos2::new(selection_rect.left() - border_width, selection_rect.top()),
            Pos2::new(selection_rect.left(), selection_rect.bottom()),
        ),
        0.0,
        border_color,
    );
    
    painter.rect_filled(
        egui::Rect::from_min_max(
            Pos2::new(selection_rect.right(), selection_rect.top()),
            Pos2::new(selection_rect.right() + border_width, selection_rect.bottom()),
        ),
        0.0,
        border_color,
    );

    // 绘制选择信息（如果显示）
    if show_info {
        let start_x = sel_x.max(0.0).floor();
        let start_y = sel_y.max(0.0).floor();
        let sel_w = sel_width.max(0.0);
        let sel_h = sel_height.max(0.0);

        let info_text = format!("{}x{}\n{} {}", 
            sel_w as u32, 
            sel_h as u32,
            start_x as u32,
            start_y as u32
        );

        Area::new(egui::Id::new("selection_info"))
            .fixed_pos(Pos2::new(start_x - 4.0, start_y - 24.0))
            .show(ctx, |ui| {
                Frame::popup(ui.style())
                    .fill(Color32::from_rgba_unmultiplied(0, 0, 0, 180))
                    .stroke(Stroke::NONE)
                    .show(ui, |ui| {
                        ui.style_mut().text_styles.insert(
                            egui::TextStyle::Body,
                            egui::FontId::proportional(11.0),
                        );
                        ui.label(egui::RichText::new(info_text).color(Color32::WHITE));
                    });
            });
    }
}

/// 渲染全屏暗色遮罩（无选择区域时）
pub fn render_fullscreen_mask(ctx: &egui::Context, screen_rect: egui::Rect) {
    let painter = ctx.layer_painter(egui::LayerId::background());
    let dark_color = Color32::from_rgba_unmultiplied(0, 0, 0, 180);
    painter.rect_filled(screen_rect, 0.0, dark_color);
}

/// 渲染绘图（红色画笔）
pub fn render_drawing(
    ctx: &egui::Context,
    selection: (f32, f32, f32, f32),
    drawing_points: &[glam::Vec2],
) {
    if drawing_points.len() < 2 {
        return;
    }
    
    let (sel_x, sel_y, _, _) = selection;
    let painter = ctx.layer_painter(egui::LayerId::new(egui::Order::Foreground, egui::Id::new("drawing")));
    
    // 红色画笔
    let stroke = Stroke::new(2.0, Color32::from_rgb(255, 0, 0));
    
    // 绘制连续的线条
    for i in 0..drawing_points.len() - 1 {
        let start = drawing_points[i];
        let end = drawing_points[i + 1];
        
        // 转换为屏幕坐标（相对于选择区域）
        let start_pos = Pos2::new(sel_x + start.x, sel_y + start.y);
        let end_pos = Pos2::new(sel_x + end.x, sel_y + end.y);
        
        painter.line_segment([start_pos, end_pos], stroke);
    }
}

/// 渲染工具栏
pub fn render_toolbar(
    ctx: &egui::Context,
    toolbar: &Toolbar,
) -> anyhow::Result<Option<String>> {
    let clicked_id = Rc::new(RefCell::new(None));
    
    // 预加载所有图标
    let mut button_textures: Vec<Option<(egui::TextureId, egui::Vec2)>> = Vec::new();
    for button in &toolbar.buttons {
        let texture_info = if let Some(icon_data) = &button.icon {
            match texture::load_icon_image(icon_data) {
                Ok(icon_image) => {
                    let size = egui::Vec2::new(icon_image.width() as f32, icon_image.height() as f32);
                    let texture_id = ctx.load_texture(
                        format!("icon_{}", button.id),
                        icon_image,
                        Default::default()
                    ).id();
                    Some((texture_id, size))
                }
                Err(_) => None,
            }
        } else {
            None
        };
        button_textures.push(texture_info);
    }

    let clicked_id_clone = clicked_id.clone();
    Area::new(egui::Id::new("toolbar"))
        .fixed_pos(Pos2::new(toolbar.x, toolbar.y))
        .show(ctx, |ui| {
            Frame::popup(ui.style())
                .fill(Color32::from_rgb(40, 40, 40))
                .stroke(Stroke::new(1.0, Color32::from_rgba_unmultiplied(255, 255, 255, 30)))
                .rounding(Rounding::same(8.0))
                .shadow(Shadow {
                    offset: Vec2::new(0.0, 4.0),
                    blur: 8.0,
                    spread: 0.0,
                    color: Color32::from_black_alpha(76),
                })
                .show(ui, |ui| {
                    ui.set_width(toolbar.width);
                    ui.set_height(toolbar.height);
                    
                    ui.horizontal(|ui| {
                        ui.spacing_mut().item_spacing = Vec2::new(4.0, 0.0);
                        
                        for (idx, button) in toolbar.buttons.iter().enumerate() {
                            let button_response = if let Some((texture_id, size)) = button_textures.get(idx).and_then(|t| *t) {
                                ui.add(egui::ImageButton::new(egui::load::SizedTexture {
                                    id: texture_id,
                                    size,
                                }))
                            } else {
                                let label = button.id.chars().next().unwrap_or('?').to_uppercase().next().unwrap_or('?');
                                ui.button(label.to_string())
                            };

                            if button_response.clicked() {
                                *clicked_id_clone.borrow_mut() = Some(button.id.clone());
                            }
                        }
                    });
                });
        });
    
    Ok(clicked_id.borrow().clone())
}
