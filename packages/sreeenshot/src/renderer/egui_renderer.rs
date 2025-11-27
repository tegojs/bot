use image::{ImageBuffer, Rgba};
use std::sync::Arc;
use winit::window::Window;
use egui::Context as EguiContext;
use egui_wgpu::Renderer as EguiWgpuRenderer;

use crate::ui::Toolbar;
use super::texture;
use super::ui;
use super::wgpu_init;
use super::input;
use super::render;

/// 基于 egui 的渲染器，使用 wgpu 作为后端
pub struct EguiRenderer {
    // WGPU 资源
    surface: wgpu::Surface<'static>,
    device: wgpu::Device,
    queue: wgpu::Queue,
    config: wgpu::SurfaceConfiguration,
    
    // 窗口和尺寸
    window: Arc<Window>,
    width: u32,
    height: u32,
    scale_factor: f64,
    
    // Egui 资源
    egui_context: EguiContext,
    egui_renderer: EguiWgpuRenderer,
    
    // 截图数据
    screenshot: ImageBuffer<Rgba<u8>, Vec<u8>>,
    screenshot_texture_id: egui::TextureId,
    
    // 输入状态
    last_mouse_pressed: bool,
}

impl EguiRenderer {
    /// 创建新的 egui 渲染器
    pub fn new(window: Arc<Window>, screenshot: ImageBuffer<Rgba<u8>, Vec<u8>>) -> anyhow::Result<Self> {
        let size = window.inner_size();
        let width = size.width;
        let height = size.height;
        let scale_factor = window.scale_factor();
        
        // 初始化 WGPU
        let (surface, device, queue, config) = wgpu_init::init_wgpu(&window, width, height)?;
        
        // 初始化 Egui
        let (egui_context, egui_renderer) = Self::init_egui(&device, config.format, scale_factor)?;
        
        // 加载截图纹理
        let screenshot_texture_id = Self::load_screenshot_texture(&egui_context, &screenshot)?;

        Ok(Self {
            surface,
            device,
            queue,
            config,
            window,
            width,
            height,
            scale_factor,
            egui_context,
            egui_renderer,
            screenshot,
            screenshot_texture_id,
            last_mouse_pressed: false,
        })
    }

    /// 初始化 Egui 上下文和渲染器
    fn init_egui(
        device: &wgpu::Device,
        surface_format: wgpu::TextureFormat,
        scale_factor: f64,
    ) -> anyhow::Result<(EguiContext, EguiWgpuRenderer)> {
        let egui_context = EguiContext::default();
        egui_context.set_pixels_per_point(scale_factor as f32);
        
        let egui_renderer = EguiWgpuRenderer::new(device, surface_format, None, 1);
        
        Ok((egui_context, egui_renderer))
    }

    /// 加载截图纹理到 Egui
    fn load_screenshot_texture(
        egui_context: &EguiContext,
        screenshot: &ImageBuffer<Rgba<u8>, Vec<u8>>,
    ) -> anyhow::Result<egui::TextureId> {
        let screenshot_color_image = texture::screenshot_to_color_image(screenshot);
        let texture_id = egui_context
            .load_texture("screenshot", screenshot_color_image, Default::default())
            .id();
        Ok(texture_id)
    }

    /// 获取窗口引用
    pub fn window(&self) -> &Arc<Window> {
        &self.window
    }

    /// 获取 egui 上下文
    #[allow(dead_code)]
    pub fn egui_context(&mut self) -> &mut egui::Context {
        &mut self.egui_context
    }

    /// 处理窗口大小调整
    pub fn resize(&mut self, new_width: u32, new_height: u32) {
        if new_width > 0 && new_height > 0 {
            self.width = new_width;
            self.height = new_height;
            self.config.width = new_width;
            self.config.height = new_height;
            self.surface.configure(&self.device, &self.config);
        }
    }

    /// 渲染一帧
    pub fn render(
        &mut self,
        selection: Option<(f32, f32, f32, f32)>,
        toolbar: Option<&Toolbar>,
        mouse_pos: Option<(f32, f32)>,
        mouse_pressed: bool,
        drawing_points: &[glam::Vec2],
    ) -> anyhow::Result<Option<String>> {
        // 1. 处理输入事件
        let raw_input = input::build_egui_input(
            self.width,
            self.height,
            mouse_pos,
            mouse_pressed,
            &mut self.last_mouse_pressed,
        );
        
        // 2. 开始 Egui 帧
        self.egui_context.set_pixels_per_point(self.scale_factor as f32);
        self.egui_context.begin_frame(raw_input);
        
        // 3. 渲染 UI
        let clicked_button_id = self.render_ui(selection, toolbar, drawing_points)?;
        
        // 4. 结束帧并获取输出
        let egui_output = self.egui_context.end_frame();
        
        // 5. 渲染到 WGPU
        render::render_to_wgpu(
            &self.surface,
            &self.device,
            &self.queue,
            &mut self.egui_renderer,
            &self.egui_context,
            &egui_output,
            self.width,
            self.height,
            self.scale_factor,
        )?;

        Ok(clicked_button_id)
    }

    /// 渲染 UI 元素
    fn render_ui(
        &mut self,
        selection: Option<(f32, f32, f32, f32)>,
        toolbar: Option<&Toolbar>,
        drawing_points: &[glam::Vec2],
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
            
            // 渲染绘图
            if !drawing_points.is_empty() {
                ui::render_drawing(&self.egui_context, selection_rect, drawing_points);
            }
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
