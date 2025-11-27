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
        
        // First, draw the original screenshot across the entire screen (fully opaque)
        // This ensures we can see the original content everywhere
        for py in 0..self.height {
            for px in 0..self.width {
                if let Some(pixel) = self.screenshot.get_pixel_checked(px, py) {
                    let idx = ((py * self.width + px) * 4) as usize;
                    if idx + 3 < frame.len() {
                        frame[idx] = pixel[0];     // R
                        frame[idx + 1] = pixel[1]; // G
                        frame[idx + 2] = pixel[2]; // B
                        frame[idx + 3] = 255;      // A (fully opaque)
                    }
                }
            }
        }

        // Then, overlay 80% transparent black on non-selected areas
        // This creates the darkening effect while keeping selection area clear
        let overlay_color = [0u8, 0u8, 0u8, 204u8]; // Black with 80% opacity (204/255)
        
        if let Some((x, y, width, height)) = selection {
            let start_x = x.max(0.0).floor() as u32;
            let start_y = y.max(0.0).floor() as u32;
            let end_x = (x + width).min(self.width as f32).floor() as u32;
            let end_y = (y + height).min(self.height as f32).floor() as u32;

            // Apply overlay to non-selected areas
            for py in 0..self.height {
                for px in 0..self.width {
                    // Skip the selection area
                    if px >= start_x && px < end_x && py >= start_y && py < end_y {
                        continue;
                    }
                    
                    let idx = ((py * self.width + px) * 4) as usize;
                    if idx + 3 < frame.len() {
                        // Blend: overlay 80% transparent black on top of original screenshot
                        // Since we already have the original screenshot, we just need to darken it
                        // by blending with black at 80% opacity
                        let r = frame[idx] as f32;
                        let g = frame[idx + 1] as f32;
                        let b = frame[idx + 2] as f32;
                        let alpha = 204.0 / 255.0; // 80% opacity
                        
                        // Blend: result = original * (1 - alpha) + overlay * alpha
                        // Since overlay is black (0, 0, 0), this simplifies to: original * (1 - alpha)
                        frame[idx] = (r * (1.0 - alpha)) as u8;
                        frame[idx + 1] = (g * (1.0 - alpha)) as u8;
                        frame[idx + 2] = (b * (1.0 - alpha)) as u8;
                        frame[idx + 3] = 255; // Keep fully opaque
                    }
                }
            }

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

