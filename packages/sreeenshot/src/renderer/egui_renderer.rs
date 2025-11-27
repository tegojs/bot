use image::{ImageBuffer, Rgba};
use pixels::{Pixels, PixelsBuilder, SurfaceTexture};
use std::rc::Rc;
use winit::window::Window;
use egui::Context as EguiContext;
use egui_wgpu::Renderer as EguiWgpuRenderer;
use egui_wgpu::ScreenDescriptor;
use pixels::wgpu;

use crate::ui::Toolbar;
use super::texture;
use super::ui;

/// 基于 egui 的渲染器，使用 pixels 和 wgpu 作为后端
pub struct EguiRenderer {
    window: Rc<Window>,
    pixels: Pixels<'static>,
    width: u32,
    height: u32,
    screenshot: ImageBuffer<Rgba<u8>, Vec<u8>>,
    screenshot_texture_id: egui::TextureId,
    scale_factor: f64,
    egui_context: EguiContext,
    egui_renderer: EguiWgpuRenderer,
    last_mouse_pressed: bool,
}

impl EguiRenderer {
    /// 创建新的 egui 渲染器
    pub fn new(window: Rc<Window>, screenshot: ImageBuffer<Rgba<u8>, Vec<u8>>) -> anyhow::Result<Self> {
        let size = window.inner_size();
        let width = size.width;
        let height = size.height;
        
        let scale_factor = window.scale_factor();
        
        // 获取窗口引用用于 SurfaceTexture
        let window_ref: &Window = &*window;
        let window_static_ref: &'static Window = unsafe {
            std::mem::transmute(window_ref)
        };
        
        let surface_texture = SurfaceTexture::new(width, height, window_static_ref);
        let pixels = PixelsBuilder::new(width, height, surface_texture)
            .blend_state(pixels::wgpu::BlendState::ALPHA_BLENDING)
            .build()
            .map_err(|e| anyhow::anyhow!("Failed to create pixels: {:?}", e))?;

        // 初始化 egui 上下文
        let egui_context = EguiContext::default();
        egui_context.set_pixels_per_point(scale_factor as f32);

        // 从 pixels 获取 wgpu 资源
        let device = pixels.device();
        let surface_format = pixels.surface_texture_format();

        // 初始化 egui wgpu 渲染器
        let egui_renderer = EguiWgpuRenderer::new(
            device,
            surface_format,
            None,
            1,
        );

        // 加载截图纹理
        let screenshot_color_image = texture::screenshot_to_color_image(&screenshot);
        let screenshot_texture_id = egui_context.load_texture(
            "screenshot",
            screenshot_color_image,
            Default::default(),
        ).id();

        Ok(Self {
            window,
            pixels,
            width,
            height,
            screenshot,
            screenshot_texture_id,
            scale_factor,
            egui_context,
            egui_renderer,
            last_mouse_pressed: false,
        })
    }

    /// 获取 pixels 实例的引用（用于调整大小等操作）
    pub fn pixels(&mut self) -> &mut Pixels<'static> {
        &mut self.pixels
    }

    /// 获取窗口引用
    pub fn window(&self) -> &Rc<Window> {
        &self.window
    }

    /// 获取 egui 上下文
    #[allow(dead_code)]
    pub fn egui_context(&mut self) -> &mut egui::Context {
        &mut self.egui_context
    }

    /// 渲染一帧
    pub fn render(
        &mut self,
        selection: Option<(f32, f32, f32, f32)>,
        toolbar: Option<&Toolbar>,
        mouse_pos: Option<(f32, f32)>,
        mouse_pressed: bool,
    ) -> anyhow::Result<Option<String>> {
        // 构建 egui 输入事件
        let mut events = Vec::new();
        
        if let Some((x, y)) = mouse_pos {
            events.push(egui::Event::PointerMoved(egui::Pos2::new(x, y)));
        }
        
        // 处理鼠标按钮事件（用于点击检测）
        if let Some((x, y)) = mouse_pos {
            if mouse_pressed && !self.last_mouse_pressed {
                events.push(egui::Event::PointerButton {
                    pos: egui::Pos2::new(x, y),
                    button: egui::PointerButton::Primary,
                    pressed: true,
                    modifiers: egui::Modifiers::default(),
                });
            } else if !mouse_pressed && self.last_mouse_pressed {
                events.push(egui::Event::PointerButton {
                    pos: egui::Pos2::new(x, y),
                    button: egui::PointerButton::Primary,
                    pressed: false,
                    modifiers: egui::Modifiers::default(),
                });
            }
        }
        
        self.last_mouse_pressed = mouse_pressed;
        
        // 开始 egui 帧
        let raw_input = egui::RawInput {
            screen_rect: Some(egui::Rect::from_min_size(
                egui::Pos2::ZERO,
                egui::Vec2::new(self.width as f32, self.height as f32),
            )),
            events,
            ..Default::default()
        };
        self.egui_context.set_pixels_per_point(self.scale_factor as f32);
        self.egui_context.begin_frame(raw_input);
        
        // 渲染 UI
        let clicked_button_id = self.render_ui(selection, toolbar)?;
        
        // 结束帧并获取输出
        let egui_output = self.egui_context.end_frame();
        
        // 获取 wgpu 资源
        let device = self.pixels.device();
        let queue = self.pixels.queue();
        
        let screen_descriptor = ScreenDescriptor {
            size_in_pixels: [self.width, self.height],
            pixels_per_point: self.scale_factor as f32,
        };

        // 将 egui 形状转换为绘制任务
        let paint_jobs = self.egui_context.tessellate(egui_output.shapes, self.scale_factor as f32);
        
        // 更新 egui 渲染器的纹理
        for (id, image_delta) in &egui_output.textures_delta.set {
            self.egui_renderer.update_texture(
                device,
                queue,
                *id,
                image_delta,
            );
        }
        
        // 渲染到 wgpu
        self.pixels.render_with(|encoder, render_target, _context| {
            // 更新 egui 缓冲区
            self.egui_renderer.update_buffers(
                device,
                queue,
                encoder,
                &paint_jobs,
                &screen_descriptor,
            );
            
            // 创建渲染通道
            let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("egui_render_pass"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: render_target,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color::TRANSPARENT),
                        store: wgpu::StoreOp::Store,
                    },
                })],
                depth_stencil_attachment: None,
                occlusion_query_set: None,
                timestamp_writes: None,
            });
            
            // 渲染 egui
            self.egui_renderer.render(
                &mut render_pass,
                &paint_jobs,
                &screen_descriptor,
            );
            
            Ok(())
        })
        .map_err(|e| anyhow::anyhow!("Failed to render egui: {:?}", e))?;

        Ok(clicked_button_id)
    }

    /// 渲染 UI 元素
    fn render_ui(
        &mut self,
        selection: Option<(f32, f32, f32, f32)>,
        toolbar: Option<&Toolbar>,
    ) -> anyhow::Result<Option<String>> {
        let screen_rect = egui::Rect::from_min_size(
            egui::Pos2::ZERO,
            egui::Vec2::new(self.width as f32, self.height as f32),
        );

        // 渲染截图背景
        ui::render_screenshot(
            &self.egui_context,
            self.screenshot_texture_id,
            &self.screenshot,
        );

        // 渲染选择区域或全屏遮罩
        if let Some(selection_rect) = selection {
            ui::render_selection_mask(
                &self.egui_context,
                selection_rect,
                screen_rect,
                toolbar.is_some(),
            );
        } else {
            ui::render_fullscreen_mask(&self.egui_context, screen_rect);
        }

        // 渲染工具栏
        if let Some(toolbar) = toolbar {
            return ui::render_toolbar(&self.egui_context, toolbar);
        }
        
        Ok(None)
    }
}

