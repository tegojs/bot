use image::{ImageBuffer, Rgba};
use std::sync::Arc;
use winit::window::Window;
use egui::Context as EguiContext;
use egui_wgpu::Renderer as EguiWgpuRenderer;
use egui_wgpu::ScreenDescriptor;
use wgpu;

use crate::ui::Toolbar;
use super::texture;
use super::ui;

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
        let (surface, device, queue, config) = Self::init_wgpu(&window, width, height)?;
        
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

    /// 初始化 WGPU 资源
    fn init_wgpu(
        window: &Arc<Window>,
        width: u32,
        height: u32,
    ) -> anyhow::Result<(
        wgpu::Surface<'static>,
        wgpu::Device,
        wgpu::Queue,
        wgpu::SurfaceConfiguration,
    )> {
        // 创建 WGPU 实例
        let instance = wgpu::Instance::new(wgpu::InstanceDescriptor {
            backends: wgpu::Backends::PRIMARY,
            ..Default::default()
        });
        
        // 创建表面
        let surface = instance
            .create_surface(window.clone())
            .map_err(|e| anyhow::anyhow!("Failed to create surface: {:?}", e))?;
        
        // 请求适配器
        let adapter = pollster::block_on(instance.request_adapter(&wgpu::RequestAdapterOptions {
            power_preference: wgpu::PowerPreference::HighPerformance,
            compatible_surface: Some(&surface),
            force_fallback_adapter: false,
        }))
        .ok_or_else(|| anyhow::anyhow!("Failed to find suitable adapter"))?;
        
        // 请求设备和队列
        let (device, queue) = pollster::block_on(adapter.request_device(
            &wgpu::DeviceDescriptor {
                label: None,
                required_features: wgpu::Features::empty(),
                required_limits: wgpu::Limits::default(),
            },
            None,
        ))
        .map_err(|e| anyhow::anyhow!("Failed to request device: {:?}", e))?;
        
        // 配置表面
        let config = Self::create_surface_config(&surface, &adapter, width, height);
        surface.configure(&device, &config);
        
        Ok((surface, device, queue, config))
    }

    /// 创建表面配置
    fn create_surface_config(
        surface: &wgpu::Surface,
        adapter: &wgpu::Adapter,
        width: u32,
        height: u32,
    ) -> wgpu::SurfaceConfiguration {
        let surface_caps = surface.get_capabilities(adapter);
        
        // 优先选择 sRGB 格式
        let surface_format = surface_caps
            .formats
            .iter()
            .copied()
            .find(|f| f.is_srgb())
            .unwrap_or(surface_caps.formats[0]);
        
        wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: surface_format,
            width,
            height,
            present_mode: surface_caps.present_modes[0],
            alpha_mode: surface_caps.alpha_modes[0],
            view_formats: vec![],
            desired_maximum_frame_latency: 2,
        }
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
    ) -> anyhow::Result<Option<String>> {
        // 1. 处理输入事件
        let raw_input = self.build_egui_input(mouse_pos, mouse_pressed);
        
        // 2. 开始 Egui 帧
        self.egui_context.set_pixels_per_point(self.scale_factor as f32);
        self.egui_context.begin_frame(raw_input);
        
        // 3. 渲染 UI
        let clicked_button_id = self.render_ui(selection, toolbar)?;
        
        // 4. 结束帧并获取输出
        let egui_output = self.egui_context.end_frame();
        
        // 5. 渲染到 WGPU
        self.render_to_wgpu(&egui_output)?;

        Ok(clicked_button_id)
    }

    /// 构建 Egui 输入事件
    fn build_egui_input(
        &mut self,
        mouse_pos: Option<(f32, f32)>,
        mouse_pressed: bool,
    ) -> egui::RawInput {
        let mut events = Vec::new();
        
        // 鼠标移动事件
        if let Some((x, y)) = mouse_pos {
            events.push(egui::Event::PointerMoved(egui::Pos2::new(x, y)));
        }
        
        // 鼠标按钮事件（检测点击）
        if let Some((x, y)) = mouse_pos {
            let pos = egui::Pos2::new(x, y);
            
            if mouse_pressed && !self.last_mouse_pressed {
                // 按钮按下
                events.push(egui::Event::PointerButton {
                    pos,
                    button: egui::PointerButton::Primary,
                    pressed: true,
                    modifiers: egui::Modifiers::default(),
                });
            } else if !mouse_pressed && self.last_mouse_pressed {
                // 按钮释放（点击完成）
                events.push(egui::Event::PointerButton {
                    pos,
                    button: egui::PointerButton::Primary,
                    pressed: false,
                    modifiers: egui::Modifiers::default(),
                });
            }
        }
        
        self.last_mouse_pressed = mouse_pressed;
        
        egui::RawInput {
            screen_rect: Some(egui::Rect::from_min_size(
                egui::Pos2::ZERO,
                egui::Vec2::new(self.width as f32, self.height as f32),
            )),
            events,
            ..Default::default()
        }
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

    /// 将 Egui 输出渲染到 WGPU
    fn render_to_wgpu(&mut self, egui_output: &egui::FullOutput) -> anyhow::Result<()> {
        let screen_descriptor = ScreenDescriptor {
            size_in_pixels: [self.width, self.height],
            pixels_per_point: self.scale_factor as f32,
        };

        // 将形状转换为绘制任务
        let paint_jobs = self.egui_context.tessellate(
            egui_output.shapes.clone(),
            self.scale_factor as f32,
        );
        
        // 更新纹理
        self.update_textures(&egui_output.textures_delta)?;
        
        // 获取表面纹理
        let output = self
            .surface
            .get_current_texture()
            .map_err(|e| anyhow::anyhow!("Failed to get current texture: {:?}", e))?;
        let view = output
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());
        
        // 创建命令编码器并渲染
        let mut encoder = self.device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
            label: Some("egui_render_encoder"),
        });
        
        // 更新缓冲区
        self.egui_renderer.update_buffers(
            &self.device,
            &self.queue,
            &mut encoder,
            &paint_jobs,
            &screen_descriptor,
        );
        
        // 执行渲染
        self.execute_render_pass(&mut encoder, &view, &paint_jobs, &screen_descriptor)?;
        
        // 提交并呈现
        self.queue.submit(std::iter::once(encoder.finish()));
        output.present();

        Ok(())
    }

    /// 更新 Egui 纹理
    fn update_textures(
        &mut self,
        textures: &egui::TexturesDelta,
    ) -> anyhow::Result<()> {
        for (id, image_delta) in &textures.set {
            self.egui_renderer.update_texture(
                &self.device,
                &self.queue,
                *id,
                image_delta,
            );
        }
        Ok(())
    }

    /// 执行渲染通道
    fn execute_render_pass(
        &mut self,
        encoder: &mut wgpu::CommandEncoder,
        view: &wgpu::TextureView,
        paint_jobs: &[egui::ClippedPrimitive],
        screen_descriptor: &ScreenDescriptor,
    ) -> anyhow::Result<()> {
        let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: Some("egui_render_pass"),
            color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                view,
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
        
        self.egui_renderer.render(
            &mut render_pass,
            paint_jobs,
            screen_descriptor,
        );
        
        Ok(())
    }
}
