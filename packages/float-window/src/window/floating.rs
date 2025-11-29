//! FloatingWindow implementation with GPU rendering

use super::builder::FloatingWindowBuilder;
use super::config::{Position, WindowConfig, WindowLevel};
use crate::animation::AnimationController;
use crate::effect::ParticleSystem;
use crate::event::{EventHandler, FloatingWindowEvent};
use crate::render::WindowPainter;
use crate::shape::{ShapeMask, WindowShape};

use egui::{CentralPanel, Color32, Context, Pos2, Rect, Vec2};
use std::sync::Arc;
use winit::application::ApplicationHandler;
use winit::dpi::{LogicalPosition, LogicalSize, PhysicalPosition};
use winit::event::{ElementState, MouseButton, WindowEvent};
use winit::event_loop::{ActiveEventLoop, ControlFlow, EventLoop};
use winit::window::{Window, WindowAttributes, WindowId, WindowLevel as WinitWindowLevel};

/// A floating window
pub struct FloatingWindow {
    config: WindowConfig,
    event_handler: EventHandler,
    particle_system: Option<ParticleSystem>,
    show_animation: Option<AnimationController>,
    #[allow(dead_code)]
    hide_animation: Option<AnimationController>,
    pub(crate) shape_mask: ShapeMask,
    /// Margin around content for particle effects
    effect_margin: f32,
    visible: bool,
    // Runtime state - drag uses mouse position relative to window
    drag_offset: Option<(f64, f64)>,  // Offset from window origin to mouse click point
    mouse_pos: Option<(f32, f32)>,
    pub(crate) is_dragging: bool,
}

impl FloatingWindow {
    /// Create a new builder
    pub fn builder() -> FloatingWindowBuilder {
        FloatingWindowBuilder::new()
    }

    /// Create from configuration
    pub(crate) fn from_config(
        config: WindowConfig,
        event_callback: Option<Box<dyn Fn(&FloatingWindowEvent) + Send + Sync>>,
    ) -> Result<Self, String> {
        let event_handler = EventHandler::new();

        if let Some(callback) = event_callback {
            event_handler.on("all", move |event| callback(event));
        }

        let effect_margin = config.effect_margin;
        let content_width = config.size.width as f32;
        let content_height = config.size.height as f32;

        // Particle system uses content size (the shape size), not window size
        let particle_system = config.effect.as_ref().map(|(effect, options)| {
            ParticleSystem::new(
                *effect,
                options.clone(),
                content_width,
                content_height,
            )
        });

        // Shape mask is offset by the effect margin and uses content size
        let shape_mask = ShapeMask::new_with_offset(
            config.shape.clone(),
            content_width,
            content_height,
            effect_margin,
        );

        Ok(Self {
            config,
            event_handler,
            particle_system,
            show_animation: None,
            hide_animation: None,
            shape_mask,
            effect_margin,
            visible: false,
            drag_offset: None,
            mouse_pos: None,
            is_dragging: false,
        })
    }

    /// Run the window (blocking)
    pub fn run(self) -> Result<(), String> {
        let event_loop = EventLoop::new().map_err(|e| e.to_string())?;
        event_loop.set_control_flow(ControlFlow::Poll);

        let mut app = FloatingWindowApp::new(self);
        event_loop.run_app(&mut app).map_err(|e| e.to_string())
    }

    /// Get window ID
    pub fn id(&self) -> Option<&str> {
        self.config.id.as_deref()
    }

    /// Check if window is visible
    pub fn is_visible(&self) -> bool {
        self.visible
    }

    /// Set visibility
    pub fn set_visible(&mut self, visible: bool) {
        if visible != self.visible {
            self.visible = visible;
            let event = if visible {
                FloatingWindowEvent::Show
            } else {
                FloatingWindowEvent::Hide
            };
            self.event_handler.dispatch(&event);
        }
    }

    /// Update particle system
    pub fn update(&mut self) {
        if let Some(ref mut system) = self.particle_system {
            system.update();
        }
    }

    /// Render the window content
    pub fn render(&mut self, ctx: &Context, rect: Rect) {
        // Update animations
        if let Some(ref mut anim) = self.show_animation {
            anim.update();
        }

        // Update particles
        self.update();

        // Use a fully transparent frame for the central panel
        let transparent_frame = egui::Frame::NONE.fill(Color32::TRANSPARENT);

        // Calculate content rect (shape area) with effect margin offset
        let margin = self.effect_margin;
        let content_width = self.config.size.width as f32;
        let content_height = self.config.size.height as f32;
        let content_rect = Rect::from_min_size(
            Pos2::new(margin, margin),
            Vec2::new(content_width, content_height),
        );

        CentralPanel::default()
            .frame(transparent_frame)
            .show(ctx, |ui| {
                // Draw background based on shape (only the shape, rest is transparent)
                let bg_color = Color32::from_rgba_unmultiplied(50, 50, 80, 220);
                match &self.config.shape {
                    WindowShape::Rectangle => {
                        ui.painter().rect_filled(content_rect, 0.0, bg_color);
                    }
                    WindowShape::Circle => {
                        let radius = content_width.min(content_height) / 2.0;
                        ui.painter().circle_filled(content_rect.center(), radius, bg_color);
                    }
                    WindowShape::Custom { .. } => {
                        ui.painter().rect_filled(content_rect, 0.0, bg_color);
                    }
                }

                // Render content (clipped to shape for circle)
                if let Some(ref content) = self.config.content {
                    // For circle, we need to position content in center
                    let inner_rect = match &self.config.shape {
                        WindowShape::Circle => {
                            // Create a smaller rect in the center for content
                            let radius = content_width.min(content_height) / 2.0;
                            let inner_size = radius * 1.2; // Content area
                            Rect::from_center_size(content_rect.center(), Vec2::splat(inner_size))
                        }
                        _ => content_rect,
                    };
                    WindowPainter::render_content(ui, content, inner_rect);
                }

                // Render particles - offset by margin so they're positioned relative to content
                if let Some(ref system) = self.particle_system {
                    WindowPainter::render_particles(ui, system, Pos2::new(margin, margin));
                }
            });
    }

    /// Handle mouse press - x, y are in window-local coordinates
    pub fn on_mouse_press(&mut self, x: f64, y: f64) {
        if self.config.draggable && self.shape_mask.contains(x as f32, y as f32) {
            // Store the offset from window origin to mouse position
            self.drag_offset = Some((x, y));
            self.is_dragging = true;
            self.event_handler
                .dispatch(&FloatingWindowEvent::DragStart {
                    x: x as f32,
                    y: y as f32,
                });
        }
        self.event_handler.dispatch(&FloatingWindowEvent::Click {
            x: x as f32,
            y: y as f32,
        });
    }

    /// Handle mouse release
    pub fn on_mouse_release(&mut self, x: f64, y: f64) {
        if self.is_dragging {
            self.is_dragging = false;
            self.drag_offset = None;
            self.event_handler.dispatch(&FloatingWindowEvent::DragEnd {
                x: x as f32,
                y: y as f32,
            });
        }
    }

    /// Handle mouse move during drag
    /// screen_x, screen_y are global screen coordinates of mouse (physical)
    /// scale_factor is used to convert logical offset to physical
    /// Returns new window position in screen coordinates (physical)
    pub fn on_drag_move(&mut self, screen_x: f64, screen_y: f64, scale_factor: f64) -> Option<(f64, f64)> {
        if self.is_dragging {
            if let Some((offset_x, offset_y)) = self.drag_offset {
                // Convert logical offset to physical
                let physical_offset_x = offset_x * scale_factor;
                let physical_offset_y = offset_y * scale_factor;

                // New window position = mouse screen position - offset (both in physical)
                let new_x = screen_x - physical_offset_x;
                let new_y = screen_y - physical_offset_y;

                self.event_handler.dispatch(&FloatingWindowEvent::Drag {
                    x: new_x as f32,
                    y: new_y as f32,
                });

                return Some((new_x, new_y));
            }
        }
        None
    }

    /// Handle mouse move (for tracking position, not dragging)
    pub fn on_mouse_move(&mut self, x: f64, y: f64) {
        self.mouse_pos = Some((x as f32, y as f32));
        self.event_handler.dispatch(&FloatingWindowEvent::MouseMove {
            x: x as f32,
            y: y as f32,
        });
    }

    /// Update stored window position
    pub fn update_position(&mut self, x: f64, y: f64) {
        self.config.position = Position::new(x, y);
    }
}

/// GPU rendering state
struct GpuState {
    surface: wgpu::Surface<'static>,
    device: wgpu::Device,
    queue: wgpu::Queue,
    surface_config: wgpu::SurfaceConfiguration,
    egui_renderer: egui_wgpu::Renderer,
}

impl GpuState {
    async fn new(window: Arc<Window>) -> Self {
        let size = window.inner_size();

        // Create wgpu instance
        let instance = wgpu::Instance::new(&wgpu::InstanceDescriptor {
            backends: wgpu::Backends::all(),
            ..Default::default()
        });

        // Create surface
        let surface = instance.create_surface(window.clone()).expect("Failed to create surface");

        // Request adapter
        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::LowPower,
                compatible_surface: Some(&surface),
                force_fallback_adapter: false,
            })
            .await
            .expect("Failed to find adapter");

        // Request device
        let (device, queue) = adapter
            .request_device(&wgpu::DeviceDescriptor {
                label: Some("float-window device"),
                required_features: wgpu::Features::empty(),
                required_limits: wgpu::Limits::default(),
                memory_hints: wgpu::MemoryHints::default(),
                trace: wgpu::Trace::Off,
                experimental_features: wgpu::ExperimentalFeatures::default(),
            })
            .await
            .expect("Failed to create device");

        // Configure surface
        let surface_caps = surface.get_capabilities(&adapter);
        let surface_format = surface_caps
            .formats
            .iter()
            .find(|f| f.is_srgb())
            .copied()
            .unwrap_or(surface_caps.formats[0]);

        // Choose best alpha mode for transparency
        let alpha_mode = if surface_caps.alpha_modes.contains(&wgpu::CompositeAlphaMode::PreMultiplied) {
            wgpu::CompositeAlphaMode::PreMultiplied
        } else if surface_caps.alpha_modes.contains(&wgpu::CompositeAlphaMode::PostMultiplied) {
            wgpu::CompositeAlphaMode::PostMultiplied
        } else {
            surface_caps.alpha_modes[0]
        };

        log::info!("Using alpha mode: {:?}", alpha_mode);

        let surface_config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: surface_format,
            width: size.width.max(1),
            height: size.height.max(1),
            present_mode: wgpu::PresentMode::AutoVsync,
            alpha_mode,
            view_formats: vec![],
            desired_maximum_frame_latency: 2,
        };
        surface.configure(&device, &surface_config);

        // Create egui renderer
        let egui_renderer = egui_wgpu::Renderer::new(
            &device,
            surface_format,
            egui_wgpu::RendererOptions::default(),
        );

        Self {
            surface,
            device,
            queue,
            surface_config,
            egui_renderer,
        }
    }

    fn resize(&mut self, width: u32, height: u32) {
        if width > 0 && height > 0 {
            self.surface_config.width = width;
            self.surface_config.height = height;
            self.surface.configure(&self.device, &self.surface_config);
        }
    }

    fn render(
        &mut self,
        primitives: Vec<egui::ClippedPrimitive>,
        textures_delta: egui::TexturesDelta,
        screen_descriptor: egui_wgpu::ScreenDescriptor,
    ) {
        let output = match self.surface.get_current_texture() {
            Ok(output) => output,
            Err(wgpu::SurfaceError::Outdated) => {
                self.surface.configure(&self.device, &self.surface_config);
                return;
            }
            Err(e) => {
                log::error!("Surface error: {:?}", e);
                return;
            }
        };

        let view = output.texture.create_view(&wgpu::TextureViewDescriptor::default());

        // Update textures
        for (id, image_delta) in &textures_delta.set {
            self.egui_renderer.update_texture(&self.device, &self.queue, *id, image_delta);
        }

        let mut encoder = self.device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
            label: Some("egui encoder"),
        });

        // Update buffers
        self.egui_renderer.update_buffers(
            &self.device,
            &self.queue,
            &mut encoder,
            &primitives,
            &screen_descriptor,
        );

        // Create render pass using begin_rendering for better lifetime handling
        {
            let color_attachments = [Some(wgpu::RenderPassColorAttachment {
                view: &view,
                resolve_target: None,
                ops: wgpu::Operations {
                    load: wgpu::LoadOp::Clear(wgpu::Color::TRANSPARENT),
                    store: wgpu::StoreOp::Store,
                },
                depth_slice: None,
            })];

            let descriptor = wgpu::RenderPassDescriptor {
                label: Some("egui render pass"),
                color_attachments: &color_attachments,
                depth_stencil_attachment: None,
                timestamp_writes: None,
                occlusion_query_set: None,
            };

            // Use owned render pass
            let render_pass = encoder.begin_render_pass(&descriptor);
            let mut render_pass: wgpu::RenderPass<'static> = render_pass.forget_lifetime();
            self.egui_renderer.render(&mut render_pass, &primitives, &screen_descriptor);
        }

        self.queue.submit(std::iter::once(encoder.finish()));
        output.present();

        // Free textures
        for id in &textures_delta.free {
            self.egui_renderer.free_texture(id);
        }
    }
}

/// Application handler for the floating window
struct FloatingWindowApp {
    floating_window: FloatingWindow,
    window: Option<Arc<Window>>,
    gpu_state: Option<GpuState>,
    egui_ctx: Context,
    egui_state: Option<egui_winit::State>,
}

impl FloatingWindowApp {
    fn new(floating_window: FloatingWindow) -> Self {
        Self {
            floating_window,
            window: None,
            gpu_state: None,
            egui_ctx: Context::default(),
            egui_state: None,
        }
    }
}

impl ApplicationHandler for FloatingWindowApp {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        if self.window.is_some() {
            return;
        }

        let config = &self.floating_window.config;
        let margin = self.floating_window.effect_margin;

        // Window size includes content size + margin on all sides
        let window_width = config.size.width as f32 + margin * 2.0;
        let window_height = config.size.height as f32 + margin * 2.0;

        let mut attrs = WindowAttributes::default()
            .with_title(config.title.as_deref().unwrap_or("Float Window"))
            .with_inner_size(LogicalSize::new(window_width, window_height))
            .with_position(LogicalPosition::new(config.position.x, config.position.y))
            .with_decorations(false)
            .with_transparent(true)
            .with_resizable(config.resizable);

        // Set window level
        attrs = attrs.with_window_level(match config.level {
            WindowLevel::Normal => WinitWindowLevel::Normal,
            WindowLevel::Top => WinitWindowLevel::AlwaysOnTop,
            WindowLevel::AlwaysOnTop => WinitWindowLevel::AlwaysOnTop,
        });

        let window = Arc::new(
            event_loop
                .create_window(attrs)
                .expect("Failed to create window"),
        );

        // Initialize GPU state (blocking on async)
        let gpu_state = pollster::block_on(GpuState::new(window.clone()));

        // Initialize egui state
        let egui_state = egui_winit::State::new(
            self.egui_ctx.clone(),
            self.egui_ctx.viewport_id(),
            &window,
            None,
            None,
            None,
        );

        self.gpu_state = Some(gpu_state);
        self.egui_state = Some(egui_state);
        self.window = Some(window);
        self.floating_window.set_visible(true);

        log::info!("Window created successfully with GPU rendering");
    }

    fn window_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        _window_id: WindowId,
        event: WindowEvent,
    ) {
        let Some(window) = &self.window else { return };
        let Some(egui_state) = &mut self.egui_state else {
            return;
        };
        let Some(gpu_state) = &mut self.gpu_state else {
            return;
        };

        // Let egui handle the event first
        let _response = egui_state.on_window_event(&window, &event);

        match event {
            WindowEvent::CloseRequested => {
                self.floating_window
                    .event_handler
                    .dispatch(&FloatingWindowEvent::Close);
                event_loop.exit();
            }
            WindowEvent::RedrawRequested => {
                let size = window.inner_size();
                if size.width == 0 || size.height == 0 {
                    return;
                }

                let raw_input = egui_state.take_egui_input(&window);
                let pixels_per_point = self.egui_ctx.pixels_per_point();

                // Use logical size for egui rect (not physical)
                let logical_width = size.width as f32 / pixels_per_point;
                let logical_height = size.height as f32 / pixels_per_point;
                let rect = Rect::from_min_size(
                    Pos2::ZERO,
                    Vec2::new(logical_width, logical_height),
                );

                let full_output = self.egui_ctx.run(raw_input, |ctx| {
                    self.floating_window.render(ctx, rect);
                });

                egui_state.handle_platform_output(&window, full_output.platform_output);

                // Tessellate
                let primitives = self.egui_ctx.tessellate(full_output.shapes, full_output.pixels_per_point);

                let screen_descriptor = egui_wgpu::ScreenDescriptor {
                    size_in_pixels: [size.width, size.height],
                    pixels_per_point: full_output.pixels_per_point,
                };

                gpu_state.render(primitives, full_output.textures_delta, screen_descriptor);
            }
            WindowEvent::CursorMoved { position, .. } => {
                // Convert physical to logical coordinates for shape mask
                let scale_factor = window.scale_factor();
                let logical_x = position.x / scale_factor;
                let logical_y = position.y / scale_factor;

                // Check if cursor is inside the shape
                let inside_shape = self.floating_window.shape_mask.contains(logical_x as f32, logical_y as f32);

                // Enable/disable hit testing based on whether cursor is inside shape
                // This makes areas outside the shape "click-through"
                let _ = window.set_cursor_hittest(inside_shape);

                // Update local mouse position (in logical coordinates)
                self.floating_window.on_mouse_move(logical_x, logical_y);

                // If dragging, calculate new window position using screen coordinates
                if self.floating_window.is_dragging {
                    // Get window position on screen
                    if let Ok(window_pos) = window.outer_position() {
                        // Calculate mouse position in screen coordinates (physical)
                        let screen_x = window_pos.x as f64 + position.x;
                        let screen_y = window_pos.y as f64 + position.y;

                        if let Some((new_x, new_y)) = self.floating_window.on_drag_move(screen_x, screen_y, scale_factor) {
                            window.set_outer_position(PhysicalPosition::new(new_x as i32, new_y as i32));
                            self.floating_window.update_position(new_x, new_y);
                        }
                    }
                }
            }
            WindowEvent::MouseInput { state, button, .. } => {
                if button == MouseButton::Left {
                    match state {
                        ElementState::Pressed => {
                            if let Some((x, y)) = self.floating_window.mouse_pos {
                                self.floating_window.on_mouse_press(x as f64, y as f64);
                            }
                        }
                        ElementState::Released => {
                            if let Some((x, y)) = self.floating_window.mouse_pos {
                                self.floating_window.on_mouse_release(x as f64, y as f64);
                            }
                        }
                    }
                }
            }
            WindowEvent::Resized(size) => {
                gpu_state.resize(size.width, size.height);
                self.floating_window
                    .event_handler
                    .dispatch(&FloatingWindowEvent::Resize {
                        width: size.width,
                        height: size.height,
                    });
                // Particle system uses content size (excluding margin), not window size
                // Content size stays constant as configured, no need to update on resize
            }
            _ => {}
        }
    }

    fn about_to_wait(&mut self, _event_loop: &ActiveEventLoop) {
        if let Some(window) = &self.window {
            window.request_redraw();
        }
    }
}
