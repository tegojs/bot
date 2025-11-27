use anyhow::Context as AnyhowContext;
use image::{ImageBuffer, Rgba};
use pixels::{Pixels, PixelsBuilder, SurfaceTexture};
use std::rc::Rc;
use winit::window::Window;

use super::RendererTrait;

// Simple 7-segment style digit rendering (5x7 pixels per digit)
// Each digit is represented as a 5x7 bitmap
const DIGIT_BITMAPS: [u8; 10 * 5] = [
    // 0
    0b11110, 0b10010, 0b10010, 0b10010, 0b11110,
    // 1
    0b00100, 0b01100, 0b00100, 0b00100, 0b01110,
    // 2
    0b11110, 0b00010, 0b11110, 0b10000, 0b11110,
    // 3
    0b11110, 0b00010, 0b11110, 0b00010, 0b11110,
    // 4
    0b10010, 0b10010, 0b11110, 0b00010, 0b00010,
    // 5
    0b11110, 0b10000, 0b11110, 0b00010, 0b11110,
    // 6
    0b11110, 0b10000, 0b11110, 0b10010, 0b11110,
    // 7
    0b11110, 0b00010, 0b00100, 0b01000, 0b01000,
    // 8
    0b11110, 0b10010, 0b11110, 0b10010, 0b11110,
    // 9
    0b11110, 0b10010, 0b11110, 0b00010, 0b11110,
];

pub struct PixelsRenderer {
    window: Rc<Window>,
    pixels: Pixels<'static>,
    width: u32,
    height: u32,
    screenshot: ImageBuffer<Rgba<u8>, Vec<u8>>,
    scale_factor: f64, // DPI scale factor
}

impl PixelsRenderer {
    pub fn pixels(&mut self) -> &mut Pixels<'static> {
        &mut self.pixels
    }
}

impl PixelsRenderer {
    pub fn new(window: Rc<Window>, screenshot: ImageBuffer<Rgba<u8>, Vec<u8>>) -> anyhow::Result<Self> {
        let size = window.inner_size();
        let width = size.width;
        let height = size.height;
        
        // Get DPI scale factor from window
        let scale_factor = window.scale_factor();
        
        // Get a reference for SurfaceTexture - we'll use unsafe to extend lifetime
        // This is safe because Rc ensures the window lives as long as we need it
        let window_ref: &Window = &*window;
        let window_static_ref: &'static Window = unsafe {
            std::mem::transmute(window_ref)
        };
        
        let surface_texture = SurfaceTexture::new(width, height, window_static_ref);
        // Use PixelsBuilder to ensure alpha blending is enabled for transparency
        let pixels = PixelsBuilder::new(width, height, surface_texture)
            .blend_state(pixels::wgpu::BlendState::ALPHA_BLENDING)
            .build()
            .map_err(|e| anyhow::anyhow!("Failed to create pixels: {:?}", e))?;

        Ok(Self {
            window,
            pixels,
            width,
            height,
            screenshot,
            scale_factor,
        })
    }
}

impl RendererTrait for PixelsRenderer {
    fn render(
        &mut self,
        selection: Option<(f32, f32, f32, f32)>,
        toolbar: Option<&crate::ui::Toolbar>,
    ) -> anyhow::Result<()> {
        let frame = self.pixels.frame_mut();
        let screenshot_data = self.screenshot.as_raw();
        
        // Pre-calculate darkening factor using integer math (faster than float)
        // Higher opacity (darker) = lower brightness
        // Using ~15% brightness for darker overlay (multiply by ~38 and divide by 255)
        const DARKEN_MUL: u16 = 38; // Darker overlay
        
        // Calculate selection bounds if present
        let (start_x, start_y, end_x, end_y) = if let Some((x, y, width, height)) = selection {
            (
                x.max(0.0).floor() as u32,
                y.max(0.0).floor() as u32,
                (x + width).min(self.width as f32).floor() as u32,
                (y + height).min(self.height as f32).floor() as u32,
            )
        } else {
            (0, 0, 0, 0) // No selection, will darken entire screen
        };
        
        let has_selection = selection.is_some();
        let width = self.width as usize;
        let height = self.height as usize;
        
        // Optimized two-pass rendering:
        // 1. First pass: copy entire screenshot (memcpy, very fast)
        // 2. Second pass: only darken RGB channels in non-selection areas (skip alpha)
        unsafe {
            let frame_ptr = frame.as_mut_ptr();
            let screenshot_ptr = screenshot_data.as_ptr();
            let frame_len = frame.len();
            let screenshot_len = screenshot_data.len();
            
            // Pass 1: Copy entire screenshot to frame (all pixels, all channels including alpha)
            // This is a simple memcpy operation, very fast
            if screenshot_len <= frame_len {
                std::ptr::copy_nonoverlapping(screenshot_ptr, frame_ptr, screenshot_len);
            }
            
            // Pass 2: Only darken RGB channels in non-selection areas
            // Skip alpha channel entirely since it's always 255 and already set in pass 1
            if has_selection {
                for py in 0..height {
                    let y = py as u32;
                    let in_selection_y = y >= start_y && y < end_y;
                    let row_start = py * width * 4;
                    
                    if in_selection_y {
                        // This row has selection: only darken pixels outside selection
                        for px in 0..width {
                            let x = px as u32;
                            if x < start_x || x >= end_x {
                                // Outside selection: darken RGB only (skip alpha)
                                let idx = row_start + (px * 4);
                                if idx + 2 < frame_len {
                                    *frame_ptr.add(idx) = ((*frame_ptr.add(idx) as u16 * DARKEN_MUL) >> 8) as u8;
                                    *frame_ptr.add(idx + 1) = ((*frame_ptr.add(idx + 1) as u16 * DARKEN_MUL) >> 8) as u8;
                                    *frame_ptr.add(idx + 2) = ((*frame_ptr.add(idx + 2) as u16 * DARKEN_MUL) >> 8) as u8;
                                    // Alpha channel already set to 255 in pass 1, skip it
                                }
                            }
                        }
                    } else {
                        // This row has no selection: darken entire row (RGB only)
                        for px in 0..width {
                            let idx = row_start + (px * 4);
                            if idx + 2 < frame_len {
                                *frame_ptr.add(idx) = ((*frame_ptr.add(idx) as u16 * DARKEN_MUL) >> 8) as u8;
                                *frame_ptr.add(idx + 1) = ((*frame_ptr.add(idx + 1) as u16 * DARKEN_MUL) >> 8) as u8;
                                *frame_ptr.add(idx + 2) = ((*frame_ptr.add(idx + 2) as u16 * DARKEN_MUL) >> 8) as u8;
                                // Alpha channel already set to 255 in pass 1, skip it
                            }
                        }
                    }
                }
            } else {
                // No selection: darken entire frame (RGB only, skip alpha)
                for idx in (0..frame_len).step_by(4) {
                    if idx + 2 < frame_len {
                        *frame_ptr.add(idx) = ((*frame_ptr.add(idx) as u16 * DARKEN_MUL) >> 8) as u8;
                        *frame_ptr.add(idx + 1) = ((*frame_ptr.add(idx + 1) as u16 * DARKEN_MUL) >> 8) as u8;
                        *frame_ptr.add(idx + 2) = ((*frame_ptr.add(idx + 2) as u16 * DARKEN_MUL) >> 8) as u8;
                        // Alpha channel already set to 255 in pass 1, skip it
                    }
                }
            }
        }
        
        // Draw selection border and background overlay if there's a selection
        // Only show info box when toolbar is present (selection is completed)
        let show_info = toolbar.is_some();
        
        if let Some((x, y, width, height)) = selection {
            let start_x = x.max(0.0).floor() as u32;
            let start_y = y.max(0.0).floor() as u32;
            let end_x = (x + width).min(self.width as f32).floor() as u32;
            let end_y = (y + height).min(self.height as f32).floor() as u32;

            // No overlay inside selection area - keep original screenshot brightness
            // Selection area should show original content clearly

            // Draw selection border (blue, fully opaque)
            // Blue color similar to reference: RGB(0, 122, 255) or similar
            let border_color = [0u8, 122u8, 255u8, 255u8];
            let border_width = 2u32;

            // Top border - draw along entire width of selection
            for px in start_x..end_x.min(self.width) {
                for bw in 0..border_width {
                    let top_y = start_y.saturating_sub(bw);
                    if top_y < self.height {
                        let idx = ((top_y * self.width + px) * 4) as usize;
                        if idx + 3 < frame.len() {
                            frame[idx..idx + 4].copy_from_slice(&border_color);
                        }
                    }
                }
            }
            
            // Bottom border - draw along entire width of selection
            for px in start_x..end_x.min(self.width) {
                for bw in 0..border_width {
                    let bottom_y = end_y.saturating_add(bw);
                    if bottom_y < self.height {
                        let idx = ((bottom_y * self.width + px) * 4) as usize;
                        if idx + 3 < frame.len() {
                            frame[idx..idx + 4].copy_from_slice(&border_color);
                        }
                    }
                }
            }
            
            // Left border - draw along entire height of selection
            for py in start_y..end_y.min(self.height) {
                for bw in 0..border_width {
                    let left_x = start_x.saturating_sub(bw);
                    if left_x < self.width {
                        let idx = ((py * self.width + left_x) * 4) as usize;
                        if idx + 3 < frame.len() {
                            frame[idx..idx + 4].copy_from_slice(&border_color);
                        }
                    }
                }
            }
            
            // Right border - draw along entire height of selection
            for py in start_y..end_y.min(self.height) {
                for bw in 0..border_width {
                    let right_x = end_x.saturating_add(bw);
                    if right_x < self.width {
                        let idx = ((py * self.width + right_x) * 4) as usize;
                        if idx + 3 < frame.len() {
                            frame[idx..idx + 4].copy_from_slice(&border_color);
                        }
                    }
                }
            }
            
            // Draw selection info box at top-left corner of selection (only when completed)
            if show_info {
                let screen_width = self.width;
                let screen_height = self.height;
                Self::draw_selection_info(screen_width, screen_height, frame, start_x, start_y, end_x, end_y, self.scale_factor)?;
            }
        }
        
        // Draw toolbar if present
        if let Some(toolbar) = toolbar {
            PixelsRenderer::draw_toolbar(self.width, self.height, frame, toolbar, self.scale_factor)?;
        }

        self.pixels.render()
            .map_err(|e| anyhow::anyhow!("Failed to render pixels: {:?}", e))?;
        Ok(())
    }
    
    fn window(&self) -> &Rc<Window> {
        &self.window
    }
}

impl PixelsRenderer {
    fn draw_toolbar(
        screen_width: u32,
        screen_height: u32,
        frame: &mut [u8],
        toolbar: &crate::ui::Toolbar,
        scale_factor: f64,
    ) -> anyhow::Result<()> {
        // Draw toolbar background (semi-transparent dark)
        let toolbar_x = toolbar.x.max(0.0).floor() as u32;
        let toolbar_y = toolbar.y.max(0.0).floor() as u32;
        let toolbar_width = toolbar.width.min(screen_width as f32 - toolbar.x).max(0.0).floor() as u32;
        let toolbar_height = toolbar.height.min(screen_height as f32 - toolbar.y).max(0.0).floor() as u32;
        
        // Draw toolbar background (solid black)
        unsafe {
            let frame_ptr = frame.as_mut_ptr();
            
            for py in toolbar_y..(toolbar_y + toolbar_height).min(screen_height) {
                for px in toolbar_x..(toolbar_x + toolbar_width).min(screen_width) {
                    let idx = ((py * screen_width + px) * 4) as usize;
                    if idx + 3 < frame.len() {
                        // Solid black background
                        *frame_ptr.add(idx) = 0u8;     // R
                        *frame_ptr.add(idx + 1) = 0u8; // G
                        *frame_ptr.add(idx + 2) = 0u8; // B
                        // Alpha stays 255
                    }
                }
            }
        }
        
        // Draw toolbar buttons with icons and text
        // No border for buttons on black toolbar (cleaner look)
        let border_width = 0u32;
        let text_color = [255u8, 255u8, 255u8, 255u8];
        
        for button in &toolbar.buttons {
            let btn_x = button.x.max(0.0).floor() as u32;
            let btn_y = button.y.max(0.0).floor() as u32;
            let btn_width = button.width.min(screen_width as f32 - button.x).max(0.0).floor() as u32;
            let btn_height = button.height.min(screen_height as f32 - button.y).max(0.0).floor() as u32;
            
            // Draw button background (slightly lighter gray on black toolbar)
            unsafe {
                let frame_ptr = frame.as_mut_ptr();
                // Slightly lighter gray for button background
                let btn_bg_color = [30u8, 30u8, 30u8];
                
                for py in btn_y..(btn_y + btn_height).min(screen_height) {
                    for px in btn_x..(btn_x + btn_width).min(screen_width) {
                        let idx = ((py * screen_width + px) * 4) as usize;
                        if idx + 3 < frame.len() {
                            *frame_ptr.add(idx) = btn_bg_color[0];
                            *frame_ptr.add(idx + 1) = btn_bg_color[1];
                            *frame_ptr.add(idx + 2) = btn_bg_color[2];
                        }
                    }
                }
            }
            
            // Draw button icon if available, otherwise draw text label
            if let Some(icon_data) = &button.icon {
                // Render icon from PNG data
                Self::draw_button_icon(screen_width, screen_height, frame, btn_x, btn_y, btn_width, btn_height, icon_data, scale_factor)?;
            } else {
                // Draw button text label (first letter of button id) - scaled by DPI
                let label = button.id.chars().next().unwrap_or('?').to_uppercase().next().unwrap_or('?');
                let base_char_size = 24.0;
                let char_size = (base_char_size * scale_factor).ceil() as u32;
                let label_x = btn_x.saturating_add((btn_width.saturating_sub(char_size)) / 2);
                let label_y = btn_y.saturating_add((btn_height.saturating_sub(char_size)) / 2);
                Self::draw_char(screen_width, screen_height, frame, label as u8, label_x, label_y, char_size, char_size, text_color)?;
            }
            
            // No border drawing for buttons (clean black toolbar design)
        }
        
        Ok(())
    }
}

impl PixelsRenderer {
    fn draw_selection_info(
        screen_width: u32,
        screen_height: u32,
        frame: &mut [u8],
        start_x: u32,
        start_y: u32,
        end_x: u32,
        end_y: u32,
        scale_factor: f64,
    ) -> anyhow::Result<()> {
        let width = (end_x.saturating_sub(start_x)) as u32;
        let height = (end_y.saturating_sub(start_y)) as u32;
        
        // Info box always at top-left corner of selection
        let mut info_x = start_x;
        let mut info_y = start_y;
        
        // Info box dimensions (scaled by DPI)
        let base_box_width = 140.0;
        let base_box_height = 40.0;
        let base_padding = 6.0;
        let box_width = (base_box_width * scale_factor).ceil() as u32;
        let box_height = (base_box_height * scale_factor).ceil() as u32;
        let padding = (base_padding * scale_factor).ceil() as u32;
        
        // Ensure info box is within bounds
        if info_x + box_width > screen_width {
            info_x = screen_width.saturating_sub(box_width);
        }
        if info_y + box_height > screen_height {
            info_y = screen_height.saturating_sub(box_height);
        }
        
        // Draw info box background (solid black, matching toolbar style)
        unsafe {
            let frame_ptr = frame.as_mut_ptr();
            
            for py in info_y..(info_y + box_height).min(screen_height) {
                for px in info_x..(info_x + box_width).min(screen_width) {
                    let idx = ((py * screen_width + px) * 4) as usize;
                    if idx + 3 < frame.len() {
                        // Solid black background
                        *frame_ptr.add(idx) = 0u8;     // R
                        *frame_ptr.add(idx + 1) = 0u8; // G
                        *frame_ptr.add(idx + 2) = 0u8; // B
                        // Alpha stays 255
                    }
                }
            }
        }
        
        // Draw text: "WxH @ X,Y" (scaled by DPI)
        let text_color = [255u8, 255u8, 255u8, 255u8];
        let base_char_width = 8.0;
        let base_char_height = 10.0;
        let base_line_spacing = 12.0;
        let char_width = (base_char_width * scale_factor).ceil() as u32;
        let char_height = (base_char_height * scale_factor).ceil() as u32;
        let line_spacing = (base_line_spacing * scale_factor).ceil() as u32;
        
        // First line: "WxH"
        let mut x_offset = info_x + padding;
        let y_offset = info_y + padding;
        
        // Draw width
        Self::draw_number(screen_width, screen_height, frame, width, x_offset, y_offset, char_width, char_height, text_color)?;
        x_offset += Self::number_width(width) * char_width + 3;
        
        // Draw "x"
        Self::draw_char(screen_width, screen_height, frame, b'x', x_offset, y_offset, char_width, char_height, text_color)?;
        x_offset += char_width + 3;
        
        // Draw height
        Self::draw_number(screen_width, screen_height, frame, height, x_offset, y_offset, char_width, char_height, text_color)?;
        
        // Second line: "@ X,Y"
        x_offset = info_x + padding;
        let y_offset2 = info_y + padding + line_spacing;
        
        // Draw "@"
        Self::draw_char(screen_width, screen_height, frame, b'@', x_offset, y_offset2, char_width, char_height, text_color)?;
        x_offset += char_width + 3;
        
        // Draw X coordinate
        Self::draw_number(screen_width, screen_height, frame, start_x, x_offset, y_offset2, char_width, char_height, text_color)?;
        x_offset += Self::number_width(start_x) * char_width + 3;
        
        // Draw ","
        Self::draw_char(screen_width, screen_height, frame, b',', x_offset, y_offset2, char_width, char_height, text_color)?;
        x_offset += char_width + 3;
        
        // Draw Y coordinate
        Self::draw_number(screen_width, screen_height, frame, start_y, x_offset, y_offset2, char_width, char_height, text_color)?;
        
        Ok(())
    }
    
    fn number_width(num: u32) -> u32 {
        if num == 0 {
            return 1;
        }
        let mut n = num;
        let mut width = 0;
        while n > 0 {
            width += 1;
            n /= 10;
        }
        width
    }
    
    fn draw_number(
        screen_width: u32,
        screen_height: u32,
        frame: &mut [u8],
        num: u32,
        x: u32,
        y: u32,
        char_width: u32,
        char_height: u32,
        color: [u8; 4],
    ) -> anyhow::Result<()> {
        if num == 0 {
            Self::draw_digit(screen_width, screen_height, frame, 0, x, y, char_width, char_height, color)?;
            return Ok(());
        }
        
        let mut n = num;
        let mut digits = Vec::new();
        while n > 0 {
            digits.push((n % 10) as usize);
            n /= 10;
        }
        digits.reverse();
        
        let mut x_offset = x;
        for &digit in &digits {
            Self::draw_digit(screen_width, screen_height, frame, digit, x_offset, y, char_width, char_height, color)?;
            x_offset += char_width + 2;
        }
        
        Ok(())
    }
    
    fn draw_digit(
        screen_width: u32,
        screen_height: u32,
        frame: &mut [u8],
        digit: usize,
        x: u32,
        y: u32,
        char_width: u32,
        char_height: u32,
        color: [u8; 4],
    ) -> anyhow::Result<()> {
        if digit >= 10 {
            return Ok(());
        }
        
        let bitmap_start = digit * 5;
        // Scale bitmap to fit char_width and char_height
        let scale_x = char_width as f32 / 5.0;
        let scale_y = char_height as f32 / 7.0;
        
        unsafe {
            let frame_ptr = frame.as_mut_ptr();
            
            for row in 0..char_height {
                let bitmap_row_idx = (row as f32 / scale_y) as usize;
                let bitmap_row = if bitmap_row_idx < 5 {
                    DIGIT_BITMAPS[bitmap_start + bitmap_row_idx]
                } else {
                    0
                };
                
                for col in 0..char_width {
                    let bitmap_col_idx = (col as f32 / scale_x) as usize;
                    if bitmap_col_idx < 5 && (bitmap_row >> (4 - bitmap_col_idx)) & 1 != 0 {
                        let px = x + col;
                        let py = y + row;
                        if px < screen_width && py < screen_height {
                            let idx = ((py * screen_width + px) * 4) as usize;
                            if idx + 3 < frame.len() {
                                frame[idx..idx + 4].copy_from_slice(&color);
                            }
                        }
                    }
                }
            }
        }
        
        Ok(())
    }
    
    fn draw_char(
        screen_width: u32,
        screen_height: u32,
        frame: &mut [u8],
        ch: u8,
        x: u32,
        y: u32,
        char_width: u32,
        char_height: u32,
        color: [u8; 4],
    ) -> anyhow::Result<()> {
        // Simple character rendering - draw patterns for common chars
        match ch {
            b'x' | b'X' => {
                // Draw 'x' as two diagonal lines (thicker)
                let thickness = 2u32;
                for i in 0..char_width.min(char_height) {
                    for t in 0..thickness {
                        // Top-left to bottom-right
                        if x + i + t < screen_width && y + i < screen_height {
                            let idx = (((y + i) * screen_width + (x + i + t)) * 4) as usize;
                            if idx + 3 < frame.len() {
                                frame[idx..idx + 4].copy_from_slice(&color);
                            }
                        }
                        // Top-right to bottom-left
                        if x + i + t < screen_width && y + (char_height - 1 - i) < screen_height {
                            let idx = (((y + (char_height - 1 - i)) * screen_width + (x + i + t)) * 4) as usize;
                            if idx + 3 < frame.len() {
                                frame[idx..idx + 4].copy_from_slice(&color);
                            }
                        }
                    }
                }
            }
            b'@' => {
                // Draw '@' as a circle outline
                let center_x = x + char_width / 2;
                let center_y = y + char_height / 2;
                let radius = (char_width.min(char_height) / 2 - 1) as i32;
                
                for py in 0..char_height {
                    for px in 0..char_width {
                        let dx = px as i32 - center_x as i32;
                        let dy = py as i32 - center_y as i32;
                        let dist_sq = dx * dx + dy * dy;
                        let r_sq = radius * radius;
                        // Draw circle outline
                        if dist_sq <= r_sq && dist_sq >= (r_sq - radius * 2) {
                            let idx = (((y + py) * screen_width + (x + px)) * 4) as usize;
                            if idx + 3 < frame.len() {
                                frame[idx..idx + 4].copy_from_slice(&color);
                            }
                        }
                    }
                }
            }
            b',' => {
                // Draw ',' as a small comma shape
                let comma_size = 3u32;
                for i in 0..comma_size {
                    if x + i < screen_width && y + char_height - comma_size + i < screen_height {
                        let idx = (((y + char_height - comma_size + i) * screen_width + (x + i)) * 4) as usize;
                        if idx + 3 < frame.len() {
                            frame[idx..idx + 4].copy_from_slice(&color);
                        }
                    }
                }
            }
            _ => {
                // For other characters, draw a simple box
                for py in y..(y + char_height).min(screen_height) {
                    for px in x..(x + char_width).min(screen_width) {
                        let idx = ((py * screen_width + px) * 4) as usize;
                        if idx + 3 < frame.len() {
                            frame[idx..idx + 4].copy_from_slice(&color);
                        }
                    }
                }
            }
        }
        
        Ok(())
    }
    
    fn draw_button_icon(
        screen_width: u32,
        screen_height: u32,
        frame: &mut [u8],
        btn_x: u32,
        btn_y: u32,
        btn_width: u32,
        btn_height: u32,
        icon_data: &[u8],
        scale_factor: f64,
    ) -> anyhow::Result<()> {
        // Decode PNG icon using image crate
        match image::load_from_memory(icon_data) {
            Ok(img) => {
                let rgba = img.to_rgba8();
                let (icon_w, icon_h) = rgba.dimensions();
                
                // Scale icon size based on DPI
                let base_icon_size = ((btn_width.min(btn_height) * 2 / 3).min(32)) as f64;
                let icon_size = (base_icon_size * scale_factor).ceil() as u32;
                let icon_x = btn_x + (btn_width.saturating_sub(icon_size)) / 2;
                let icon_y = btn_y + (btn_height.saturating_sub(icon_size)) / 2;
                
                // Scale and render icon
                let scale_x = icon_size as f32 / icon_w as f32;
                let scale_y = icon_size as f32 / icon_h as f32;
                
                // Icon color (white for visibility on dark background)
                let icon_color = [255u8, 255u8, 255u8];
                
                for py in 0..icon_size {
                    for px in 0..icon_size {
                        let src_x = (px as f32 / scale_x) as u32;
                        let src_y = (py as f32 / scale_y) as u32;
                        
                        if src_x < icon_w && src_y < icon_h {
                            let pixel = rgba.get_pixel(src_x, src_y);
                            // Use alpha channel from icon, but use white color for visibility
                            let a = pixel[3];
                            
                            // If pixel has any opacity, render it as white
                            if a > 0 {
                                let dst_x = icon_x + px;
                                let dst_y = icon_y + py;
                                
                                if dst_x < screen_width && dst_y < screen_height {
                                    let dst_idx = ((dst_y * screen_width + dst_x) * 4) as usize;
                                    if dst_idx + 3 < frame.len() {
                                        // Alpha blend white icon with background
                                        let alpha = a as f32 / 255.0;
                                        let bg_r = frame[dst_idx] as f32;
                                        let bg_g = frame[dst_idx + 1] as f32;
                                        let bg_b = frame[dst_idx + 2] as f32;
                                        
                                        // Blend white icon color with background
                                        frame[dst_idx] = ((icon_color[0] as f32 * alpha) + (bg_r * (1.0 - alpha))) as u8;
                                        frame[dst_idx + 1] = ((icon_color[1] as f32 * alpha) + (bg_g * (1.0 - alpha))) as u8;
                                        frame[dst_idx + 2] = ((icon_color[2] as f32 * alpha) + (bg_b * (1.0 - alpha))) as u8;
                                        // Alpha channel stays 255
                                    }
                                }
                            }
                        }
                    }
                }
            }
            Err(_) => {
                // If icon decoding fails, draw a placeholder
                let base_icon_size = (btn_width.min(btn_height) * 2 / 3).min(32) as f64;
                let icon_size = (base_icon_size * scale_factor).ceil() as u32;
                let icon_x = btn_x + (btn_width.saturating_sub(icon_size)) / 2;
                let icon_y = btn_y + (btn_height.saturating_sub(icon_size)) / 2;
                
                // Draw a simple placeholder icon (filled circle)
                let center_x = icon_x + icon_size / 2;
                let center_y = icon_y + icon_size / 2;
                let radius = icon_size / 2 - 2;
                let icon_color = [200u8, 200u8, 200u8, 255u8];
                
                for py in icon_y..(icon_y + icon_size).min(screen_height) {
                    for px in icon_x..(icon_x + icon_size).min(screen_width) {
                        let dx = px as i32 - center_x as i32;
                        let dy = py as i32 - center_y as i32;
                        let dist_sq = dx * dx + dy * dy;
                        if dist_sq <= (radius * radius) as i32 {
                            let idx = ((py * screen_width + px) * 4) as usize;
                            if idx + 3 < frame.len() {
                                frame[idx..idx + 4].copy_from_slice(&icon_color);
                            }
                        }
                    }
                }
            }
        }
        
        Ok(())
    }
    
    fn window(&self) -> &Rc<Window> {
        &self.window
    }
}

