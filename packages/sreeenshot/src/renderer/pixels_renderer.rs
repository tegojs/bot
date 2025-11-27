use anyhow::Context as AnyhowContext;
use image::{ImageBuffer, Rgba};
use pixels::{Pixels, PixelsBuilder, SurfaceTexture};
use std::rc::Rc;
use winit::window::Window;

use super::RendererTrait;

pub struct PixelsRenderer {
    window: Rc<Window>,
    pixels: Pixels<'static>,
    width: u32,
    height: u32,
    screenshot: ImageBuffer<Rgba<u8>, Vec<u8>>,
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
        })
    }
}

impl RendererTrait for PixelsRenderer {
    fn render(
        &mut self,
        selection: Option<(f32, f32, f32, f32)>,
    ) -> anyhow::Result<()> {
        let frame = self.pixels.frame_mut();
        let screenshot_data = self.screenshot.as_raw();
        
        // Pre-calculate darkening factor using integer math (faster than float)
        // 80% opacity = 20% brightness = multiply by 51 and divide by 255
        // Using fixed-point math: value * 51 / 255 ≈ value * 0.2
        const DARKEN_MUL: u16 = 51;
        
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
        
        // Draw selection border if there's a selection
        if let Some((x, y, width, height)) = selection {
            let start_x = x.max(0.0).floor() as u32;
            let start_y = y.max(0.0).floor() as u32;
            let end_x = (x + width).min(self.width as f32).floor() as u32;
            let end_y = (y + height).min(self.height as f32).floor() as u32;

            // Draw selection border (white, fully opaque)
            let border_color = [255u8, 255u8, 255u8, 255u8];
            let border_width = 2u32;

            // Top and bottom borders
            for px in start_x.saturating_sub(border_width)..end_x.saturating_add(border_width).min(self.width) {
                for bw in 0..border_width {
                    let top_y = start_y.saturating_sub(bw);
                    let bottom_y = end_y.saturating_add(bw);
                    if top_y < self.height && (px < start_x || px >= end_x) {
                        let idx = ((top_y * self.width + px) * 4) as usize;
                        if idx + 3 < frame.len() {
                            frame[idx..idx + 4].copy_from_slice(&border_color);
                        }
                    }
                    if bottom_y < self.height && (px < start_x || px >= end_x) {
                        let idx = ((bottom_y * self.width + px) * 4) as usize;
                        if idx + 3 < frame.len() {
                            frame[idx..idx + 4].copy_from_slice(&border_color);
                        }
                    }
                }
            }

            // Left and right borders
            for py in start_y.saturating_sub(border_width)..end_y.saturating_add(border_width).min(self.height) {
                for bw in 0..border_width {
                    let left_x = start_x.saturating_sub(bw);
                    let right_x = end_x.saturating_add(bw);
                    if left_x < self.width {
                        let idx = ((py * self.width + left_x) * 4) as usize;
                        if idx + 3 < frame.len() {
                            frame[idx..idx + 4].copy_from_slice(&border_color);
                        }
                    }
                    if right_x < self.width {
                        let idx = ((py * self.width + right_x) * 4) as usize;
                        if idx + 3 < frame.len() {
                            frame[idx..idx + 4].copy_from_slice(&border_color);
                        }
                    }
                }
            }
        }

        self.pixels.render()
            .map_err(|e| anyhow::anyhow!("Failed to render pixels: {:?}", e))?;
        Ok(())
    }
    
    fn window(&self) -> &Rc<Window> {
        &self.window
    }
}

