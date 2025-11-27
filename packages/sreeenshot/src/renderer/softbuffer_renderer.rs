use anyhow::Context as AnyhowContext;
use image::{ImageBuffer, Rgba};
use softbuffer::{Context, Surface};
use std::num::NonZeroU32;
use std::rc::Rc;
use winit::window::Window;

use super::RendererTrait;

pub struct SoftbufferRenderer {
    window: Rc<Window>,
    context: Context<Rc<Window>>,
    surface: Surface<Rc<Window>, Rc<Window>>,
    width: u32,
    height: u32,
    screenshot: ImageBuffer<Rgba<u8>, Vec<u8>>,
}

impl SoftbufferRenderer {
    pub fn new(window: Rc<Window>, screenshot: ImageBuffer<Rgba<u8>, Vec<u8>>) -> anyhow::Result<Self> {
        let size = window.inner_size();
        let width = size.width;
        let height = size.height;

        let context = unsafe { Context::new(window.clone()) }
            .map_err(|e| anyhow::anyhow!("Failed to create graphics context: {:?}", e))?;
        let surface = unsafe { Surface::new(&context, window.clone()) }
            .map_err(|e| anyhow::anyhow!("Failed to create surface: {:?}", e))?;

        Ok(Self {
            window,
            context,
            surface,
            width,
            height,
            screenshot,
        })
    }
}

impl RendererTrait for SoftbufferRenderer {
    fn render(
        &mut self,
        selection: Option<(f32, f32, f32, f32)>,
    ) -> anyhow::Result<()> {
        let width = NonZeroU32::new(self.width)
            .ok_or_else(|| anyhow::anyhow!("Width must be non-zero"))?;
        let height = NonZeroU32::new(self.height)
            .ok_or_else(|| anyhow::anyhow!("Height must be non-zero"))?;
        
        self.surface.resize(width, height)
            .map_err(|e| anyhow::anyhow!("Failed to resize surface: {:?}", e))?;
        
        let mut buffer = self.surface.buffer_mut()
            .map_err(|e| anyhow::anyhow!("Failed to get buffer: {:?}", e))?;
        
        // Initialize pixels to 0 (transparent/clear)
        // The window background (80% black) will show through where we don't draw
        let mut pixels = vec![0u32; (self.width * self.height) as usize];

        // Only draw the screenshot in the selection area (if there is one)
        // Non-selected areas will be transparent, showing the window's semi-transparent black background
        if let Some((x, y, width, height)) = selection {
            let start_x = x.max(0.0).floor() as u32;
            let start_y = y.max(0.0).floor() as u32;
            let end_x = (x + width).min(self.width as f32).floor() as u32;
            let end_y = (y + height).min(self.height as f32).floor() as u32;

            // Draw the original screenshot only in the selection area
            // This area will be fully opaque, showing the original content
            for py in start_y..end_y.min(self.height) {
                for px in start_x..end_x.min(self.width) {
                    if let Some(pixel) = self.screenshot.get_pixel_checked(px, py) {
                        let r = pixel[0] as u32;
                        let g = pixel[1] as u32;
                        let b = pixel[2] as u32;
                        // softbuffer format: 0x00RRGGBB (no alpha channel)
                        let rgb = (r << 16) | (g << 8) | b;
                        pixels[(py * self.width + px) as usize] = rgb;
                    }
                }
            }

            // Draw selection border
            let border_color = 0xFFFFFFFFu32; // White border
            let border_width = 2u32;

            // Top and bottom borders
            for px in start_x.saturating_sub(border_width)..end_x.saturating_add(border_width).min(self.width) {
                for bw in 0..border_width {
                    let top_y = start_y.saturating_sub(bw);
                    let bottom_y = end_y.saturating_add(bw);
                    if top_y < self.height && (px < start_x || px >= end_x) {
                        pixels[(top_y * self.width + px) as usize] = border_color;
                    }
                    if bottom_y < self.height && (px < start_x || px >= end_x) {
                        pixels[(bottom_y * self.width + px) as usize] = border_color;
                    }
                }
            }

            // Left and right borders
            for py in start_y.saturating_sub(border_width)..end_y.saturating_add(border_width).min(self.height) {
                for bw in 0..border_width {
                    let left_x = start_x.saturating_sub(bw);
                    let right_x = end_x.saturating_add(bw);
                    if left_x < self.width {
                        pixels[(py * self.width + left_x) as usize] = border_color;
                    }
                    if right_x < self.width {
                        pixels[(py * self.width + right_x) as usize] = border_color;
                    }
                }
            }
        }

        // Copy pixels to buffer
        for (i, pixel) in pixels.iter().enumerate() {
            buffer[i] = *pixel;
        }
        
        buffer.present()
            .map_err(|e| anyhow::anyhow!("Failed to present buffer: {:?}", e))?;
        Ok(())
    }
    
    fn window(&self) -> &Rc<Window> {
        &self.window
    }
}

