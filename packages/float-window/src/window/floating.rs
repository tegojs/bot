//! FloatingWindow implementation with GPU rendering

use super::builder::FloatingWindowBuilder;
use super::commands::{CommandReceiver, CommandSender, WindowCommand, WindowRegistry};
use super::config::{Position, WindowConfig, WindowLevel};
use super::controller::ControllerState;
use crate::animation::AnimationController;
use crate::content::Content;
use crate::effect::{ParticleSystem, PresetEffect, PresetEffectOptions};
use crate::event::{EventHandler, FloatingWindowEvent};
use crate::menu_bar::{MenuBarEvent, MenuBarManager};
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
    pub(crate) drag_offset: Option<(f64, f64)>,  // Offset from window origin to mouse click point
    mouse_pos: Option<(f32, f32)>,
    pub(crate) is_dragging: bool,
    /// Whether this window has focus (used to prevent drag when other window is focused)
    has_focus: bool,
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
            has_focus: false,
        })
    }

    /// Run the window (blocking)
    pub fn run(self) -> Result<(), String> {
        let event_loop = EventLoop::new().map_err(|e| e.to_string())?;
        event_loop.set_control_flow(ControlFlow::Poll);

        let mut app = FloatingWindowApp::new(self);
        event_loop.run_app(&mut app).map_err(|e| e.to_string())
    }

    /// Run multiple windows together (blocking)
    pub fn run_multiple(windows: Vec<FloatingWindow>) -> Result<(), String> {
        if windows.is_empty() {
            return Err("No windows provided".to_string());
        }

        let event_loop = EventLoop::new().map_err(|e| e.to_string())?;
        event_loop.set_control_flow(ControlFlow::Poll);

        let mut app = FloatingWindowApp::new_multi(windows);
        event_loop.run_app(&mut app).map_err(|e| e.to_string())
    }

    /// Run as a controller window that can create/manage other windows (blocking)
    pub fn run_controller(controller: FloatingWindow) -> Result<(), String> {
        let (tx, rx) = super::commands::create_command_channel();

        let event_loop = EventLoop::new().map_err(|e| e.to_string())?;
        event_loop.set_control_flow(ControlFlow::Poll);

        let mut app = FloatingWindowApp::new_controller(controller, tx, rx);
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

    /// Set content (image/text)
    pub fn set_content(&mut self, content: Option<Content>) {
        self.config.content = content;
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
                // Check if we have image content
                let has_image_content = matches!(&self.config.content, Some(Content::Image { .. }));

                // Draw background based on shape (only when no image content)
                // Use full alpha (255) to avoid compositor blending artifacts
                if !has_image_content {
                    let bg_color = Color32::from_rgba_unmultiplied(50, 50, 80, 255);
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
                }

                // Render content
                if let Some(ref content) = self.config.content {
                    // Render content to the full content_rect - shape masking is applied in the painter
                    WindowPainter::render_content(ui, content, content_rect, &self.config.shape);
                }

                // Render particles - offset by margin so they're positioned relative to content
                if let Some(ref system) = self.particle_system {
                    WindowPainter::render_particles(ui, system, Pos2::new(margin, margin));
                }
            });
    }

    /// Handle mouse press - x, y are in window-local coordinates
    pub fn on_mouse_press(&mut self, x: f64, y: f64) {
        // Only handle drag if window has focus (prevents drag when clicking through other windows)
        if self.has_focus && self.config.draggable && self.shape_mask.contains(x as f32, y as f32) {
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

    /// Set focus state
    pub fn set_focus(&mut self, focused: bool) {
        self.has_focus = focused;
        // Cancel any ongoing drag if focus is lost
        if !focused && self.is_dragging {
            self.is_dragging = false;
            self.drag_offset = None;
        }
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

        // Prefer non-sRGB format for proper alpha blending with transparent windows
        // egui prefers Rgba8Unorm or Bgra8Unorm over sRGB variants
        let surface_format = surface_caps
            .formats
            .iter()
            .find(|f| **f == wgpu::TextureFormat::Bgra8Unorm || **f == wgpu::TextureFormat::Rgba8Unorm)
            .copied()
            .unwrap_or_else(|| {
                // Fallback to first non-sRGB, or just first format
                surface_caps.formats.iter()
                    .find(|f| !f.is_srgb())
                    .copied()
                    .unwrap_or(surface_caps.formats[0])
            });

        // Use PreMultiplied alpha mode for proper compositing with transparent windows
        let alpha_mode = if surface_caps.alpha_modes.contains(&wgpu::CompositeAlphaMode::PreMultiplied) {
            wgpu::CompositeAlphaMode::PreMultiplied
        } else if surface_caps.alpha_modes.contains(&wgpu::CompositeAlphaMode::PostMultiplied) {
            wgpu::CompositeAlphaMode::PostMultiplied
        } else {
            surface_caps.alpha_modes[0]
        };

        log::info!("Using surface format: {:?}, alpha mode: {:?}", surface_format, alpha_mode);

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

/// State for a single window instance
struct WindowState {
    floating_window: FloatingWindow,
    window: Arc<Window>,
    gpu_state: GpuState,
    egui_ctx: Context,
    egui_state: egui_winit::State,
    /// Controller state if this is a controller window
    controller_state: Option<ControllerState>,
    /// Whether this window is a managed flow window (not the controller)
    is_managed: bool,
    /// Whether a widget is currently being interacted with (dragged)
    widget_has_focus: bool,
}

/// Application handler for the floating window
struct FloatingWindowApp {
    /// Pending windows to create
    pending_windows: Vec<FloatingWindow>,
    /// Active windows keyed by WindowId
    windows: std::collections::HashMap<WindowId, WindowState>,
    /// Command receiver for controller commands
    command_receiver: Option<CommandReceiver>,
    /// Command sender to pass to new controller windows
    command_sender: Option<CommandSender>,
    /// Window registry shared with controller
    registry: WindowRegistry,
    /// ID of the controller window (if any)
    controller_window_id: Option<WindowId>,
    /// Pending configs to create (from controller commands)
    pending_configs: Vec<(WindowConfig, Option<(PresetEffect, PresetEffectOptions)>)>,
    /// Menu bar manager for tray icons
    menu_bar_manager: MenuBarManager,
}

impl FloatingWindowApp {
    fn new(floating_window: FloatingWindow) -> Self {
        Self {
            pending_windows: vec![floating_window],
            windows: std::collections::HashMap::new(),
            command_receiver: None,
            command_sender: None,
            registry: WindowRegistry::new(),
            controller_window_id: None,
            pending_configs: Vec::new(),
            menu_bar_manager: MenuBarManager::new(),
        }
    }

    fn new_multi(windows: Vec<FloatingWindow>) -> Self {
        Self {
            pending_windows: windows,
            windows: std::collections::HashMap::new(),
            command_receiver: None,
            command_sender: None,
            registry: WindowRegistry::new(),
            controller_window_id: None,
            pending_configs: Vec::new(),
            menu_bar_manager: MenuBarManager::new(),
        }
    }

    fn new_controller(controller_window: FloatingWindow, command_sender: CommandSender, command_receiver: CommandReceiver) -> Self {
        Self {
            pending_windows: vec![controller_window],
            windows: std::collections::HashMap::new(),
            command_receiver: Some(command_receiver),
            command_sender: Some(command_sender),
            registry: WindowRegistry::new(),
            controller_window_id: None,
            pending_configs: Vec::new(),
            menu_bar_manager: MenuBarManager::new(),
        }
    }

    fn create_egui_context() -> Context {
        let egui_ctx = Context::default();
        // Disable feathering (anti-aliasing) to avoid compositor artifacts
        // on transparent windows. Semi-transparent edge pixels cause
        // white ring artifacts with macOS compositor.
        egui_ctx.tessellation_options_mut(|opts| {
            opts.feathering = false;
        });
        egui_ctx
    }

    /// Process pending commands from the controller
    fn process_commands(&mut self, event_loop: &ActiveEventLoop) {
        if let Some(ref receiver) = self.command_receiver {
            while let Ok(command) = receiver.try_recv() {
                match command {
                    WindowCommand::Create { config, effect } => {
                        log::info!("Creating new window from controller: {:?}", config.title);
                        self.pending_configs.push((config, effect));
                    }
                    WindowCommand::Close { id } => {
                        log::info!("Closing window {:?}", id);
                        if let Some(_state) = self.windows.remove(&id) {
                            self.registry.unregister(id);
                        }
                        if self.windows.is_empty() {
                            event_loop.exit();
                        }
                    }
                    WindowCommand::CloseByName { name } => {
                        log::info!("Closing window by name: {}", name);
                        if let Some(id) = self.registry.find_by_name(&name) {
                            if let Some(_state) = self.windows.remove(&id) {
                                self.registry.unregister(id);
                            }
                        }
                    }
                    WindowCommand::UpdateEffectOptions { id, options } => {
                        log::info!("Updating effect options for {:?}", id);
                        if let Some(state) = self.windows.get_mut(&id) {
                            if let Some(ref mut system) = state.floating_window.particle_system {
                                // Recreate particle system with new options
                                let width = state.floating_window.config.size.width as f32;
                                let height = state.floating_window.config.size.height as f32;
                                if let Some((effect, _)) = &state.floating_window.config.effect {
                                    *system = ParticleSystem::immediate(*effect, options.clone(), width, height);
                                }
                            }
                            self.registry.update_options(id, options);
                        }
                    }
                    WindowCommand::CloseAll => {
                        log::info!("Closing all managed windows");
                        let managed_ids: Vec<WindowId> = self.windows.iter()
                            .filter(|(_, state)| state.is_managed)
                            .map(|(id, _)| *id)
                            .collect();
                        for id in managed_ids {
                            self.windows.remove(&id);
                            self.registry.unregister(id);
                        }
                    }
                    WindowCommand::AddMenuBarItem { item } => {
                        log::info!("Adding menu bar item: {}", item.name);
                        if let Err(e) = self.menu_bar_manager.add_item(item) {
                            log::error!("Failed to add menu bar item: {}", e);
                        }
                    }
                    WindowCommand::RemoveMenuBarItem { id } => {
                        log::info!("Removing menu bar item: {}", id);
                        if let Err(e) = self.menu_bar_manager.remove_item(&id) {
                            log::error!("Failed to remove menu bar item: {}", e);
                        }
                    }
                    WindowCommand::UpdateMenuBarIcon { id, icon } => {
                        log::info!("Updating menu bar icon: {}", id);
                        if let Err(e) = self.menu_bar_manager.update_icon(&id, &icon) {
                            log::error!("Failed to update menu bar icon: {}", e);
                        }
                    }
                    WindowCommand::UpdateMenuBarTooltip { id, tooltip } => {
                        log::info!("Updating menu bar tooltip: {}", id);
                        if let Err(e) = self.menu_bar_manager.update_tooltip(&id, &tooltip) {
                            log::error!("Failed to update menu bar tooltip: {}", e);
                        }
                    }
                    WindowCommand::UpdateContent { id, content } => {
                        log::info!("Updating content for window {:?}", id);
                        if let Some(state) = self.windows.get_mut(&id) {
                            state.floating_window.set_content(content);
                        }
                    }
                }
            }
        }
    }

    /// Process menu bar events
    fn process_menu_bar_events(&mut self, event_loop: &ActiveEventLoop) {
        for event in self.menu_bar_manager.process_events() {
            match event {
                MenuBarEvent::Click { id } => {
                    log::debug!("Menu bar item clicked: {}", id);
                }
                MenuBarEvent::DoubleClick { id } => {
                    log::debug!("Menu bar item double-clicked: {}", id);
                }
                MenuBarEvent::MenuItemClick { tray_id, menu_item_id } => {
                    log::debug!("Menu item clicked: {} -> {}", tray_id, menu_item_id);
                }
                MenuBarEvent::QuitRequested => {
                    log::info!("Quit requested from menu bar");
                    event_loop.exit();
                }
            }
        }
    }

    /// Create pending windows from configs
    fn create_pending_windows(&mut self, event_loop: &ActiveEventLoop) {
        for (config, effect) in self.pending_configs.drain(..) {
            // Build a FloatingWindow from the config
            let window_name = config.title.clone().unwrap_or_else(|| "Window".to_string());
            let effect_info = effect.clone();

            let mut builder = FloatingWindow::builder()
                .position(config.position.x, config.position.y)
                .size(config.size.width, config.size.height)
                .shape(config.shape.clone())
                .draggable(config.draggable)
                .always_on_top(config.level == WindowLevel::AlwaysOnTop);

            if let Some(title) = config.title {
                builder = builder.title(title);
            }

            if let Some((effect, options)) = effect {
                builder = builder.effect(effect, options);
            }

            if let Some(content) = config.content {
                builder = builder.content(content);
            }

            match builder.build() {
                Ok(floating_window) => {
                    // Create the window
                    let margin = floating_window.effect_margin;
                    let window_width = config.size.width as f32 + margin * 2.0;
                    let window_height = config.size.height as f32 + margin * 2.0;

                    let mut attrs = WindowAttributes::default()
                        .with_title(&window_name)
                        .with_inner_size(LogicalSize::new(window_width, window_height))
                        .with_position(LogicalPosition::new(config.position.x, config.position.y))
                        .with_decorations(false)
                        .with_transparent(true)
                        .with_resizable(false)
                        .with_window_level(WinitWindowLevel::AlwaysOnTop);

                    match event_loop.create_window(attrs) {
                        Ok(window) => {
                            let window = Arc::new(window);
                            let window_id = window.id();

                            let gpu_state = pollster::block_on(GpuState::new(window.clone()));
                            let egui_ctx = Self::create_egui_context();
                            let egui_state = egui_winit::State::new(
                                egui_ctx.clone(),
                                egui_ctx.viewport_id(),
                                &window,
                                None,
                                None,
                                None,
                            );

                            let state = WindowState {
                                floating_window,
                                window,
                                gpu_state,
                                egui_ctx,
                                egui_state,
                                controller_state: None,
                                is_managed: true,
                                widget_has_focus: false,
                            };

                            // Register in the registry
                            let effect_type = effect_info.as_ref().map(|(e, _)| *e);
                            let effect_opts = effect_info.map(|(_, o)| o);
                            let window_size = (config.size.width, config.size.height);
                            self.registry.register(window_id, window_name, window_size, effect_type, effect_opts);

                            self.windows.insert(window_id, state);
                            log::info!("Created managed window {:?}", window_id);
                        }
                        Err(e) => {
                            log::error!("Failed to create window: {}", e);
                        }
                    }
                }
                Err(e) => {
                    log::error!("Failed to build FloatingWindow: {}", e);
                }
            }
        }
    }
}

impl ApplicationHandler for FloatingWindowApp {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        // Create all pending windows
        for mut floating_window in self.pending_windows.drain(..) {
            let config = &floating_window.config;
            let margin = floating_window.effect_margin;

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

            let window_id = window.id();

            // Initialize GPU state (blocking on async)
            let gpu_state = pollster::block_on(GpuState::new(window.clone()));

            // Create egui context for this window
            let egui_ctx = Self::create_egui_context();

            // Initialize egui state
            let egui_state = egui_winit::State::new(
                egui_ctx.clone(),
                egui_ctx.viewport_id(),
                &window,
                None,
                None,
                None,
            );

            floating_window.set_visible(true);

            // Check if this is a controller window (first window with command_sender set)
            let controller_state = if self.controller_window_id.is_none() && self.command_sender.is_some() {
                self.controller_window_id = Some(window_id);
                let sender = self.command_sender.clone().unwrap();
                Some(ControllerState::new(sender, self.registry.clone()))
            } else {
                None
            };

            let state = WindowState {
                floating_window,
                window,
                gpu_state,
                egui_ctx,
                egui_state,
                controller_state,
                is_managed: false, // Initial windows are not "managed" flow windows
                widget_has_focus: false,
            };

            self.windows.insert(window_id, state);
            log::info!("Window {:?} created successfully with GPU rendering", window_id);
        }
    }

    fn window_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        window_id: WindowId,
        event: WindowEvent,
    ) {
        let Some(state) = self.windows.get_mut(&window_id) else { return };

        // Let egui handle the event first
        let _response = state.egui_state.on_window_event(&state.window, &event);

        match event {
            WindowEvent::CloseRequested => {
                state.floating_window
                    .event_handler
                    .dispatch(&FloatingWindowEvent::Close);
                self.windows.remove(&window_id);
                if self.windows.is_empty() {
                    event_loop.exit();
                }
            }
            WindowEvent::RedrawRequested => {
                let size = state.window.inner_size();
                if size.width == 0 || size.height == 0 {
                    return;
                }

                let raw_input = state.egui_state.take_egui_input(&state.window);
                let pixels_per_point = state.egui_ctx.pixels_per_point();

                // Use logical size for egui rect (not physical)
                let logical_width = size.width as f32 / pixels_per_point;
                let logical_height = size.height as f32 / pixels_per_point;
                let rect = Rect::from_min_size(
                    Pos2::ZERO,
                    Vec2::new(logical_width, logical_height),
                );

                let full_output = state.egui_ctx.run(raw_input, |ctx| {
                    // Check if this is a controller window
                    if let Some(ref mut controller) = state.controller_state {
                        controller.render(ctx);
                    } else {
                        state.floating_window.render(ctx, rect);
                    }
                });

                state.egui_state.handle_platform_output(&state.window, full_output.platform_output);

                // Tessellate
                let primitives = state.egui_ctx.tessellate(full_output.shapes, full_output.pixels_per_point);

                let screen_descriptor = egui_wgpu::ScreenDescriptor {
                    size_in_pixels: [size.width, size.height],
                    pixels_per_point: full_output.pixels_per_point,
                };

                state.gpu_state.render(primitives, full_output.textures_delta, screen_descriptor);

                // Track if any widget is being actively interacted with
                // is_using_pointer: true when actively dragging a widget (scrollbar, slider)
                // focused: true when a text field has keyboard focus
                state.widget_has_focus = state.egui_ctx.is_using_pointer()
                    || state.egui_ctx.memory(|mem| mem.focused().is_some());
            }
            WindowEvent::CursorMoved { position, .. } => {
                // Convert physical to logical coordinates for shape mask
                let scale_factor = state.window.scale_factor();
                let logical_x = position.x / scale_factor;
                let logical_y = position.y / scale_factor;

                // Check if cursor is inside the shape
                let inside_shape = state.floating_window.shape_mask.contains(logical_x as f32, logical_y as f32);

                // Enable/disable hit testing based on whether cursor is inside shape
                // This makes areas outside the shape "click-through"
                let _ = state.window.set_cursor_hittest(inside_shape);

                // Update local mouse position (in logical coordinates)
                state.floating_window.on_mouse_move(logical_x, logical_y);

                // If dragging, calculate new window position using screen coordinates
                // But stop dragging if egui is using the pointer (e.g., slider, scrollbar)
                if state.floating_window.is_dragging {
                    if state.widget_has_focus {
                        // egui took over - cancel our drag
                        state.floating_window.is_dragging = false;
                        state.floating_window.drag_offset = None;
                    } else if let Ok(window_pos) = state.window.outer_position() {
                        // Calculate mouse position in screen coordinates (physical)
                        let screen_x = window_pos.x as f64 + position.x;
                        let screen_y = window_pos.y as f64 + position.y;

                        if let Some((new_x, new_y)) = state.floating_window.on_drag_move(screen_x, screen_y, scale_factor) {
                            state.window.set_outer_position(PhysicalPosition::new(new_x as i32, new_y as i32));
                            state.floating_window.update_position(new_x, new_y);
                        }
                    }
                }
            }
            WindowEvent::MouseInput { state: button_state, button, .. } => {
                if button == MouseButton::Left {
                    match button_state {
                        ElementState::Pressed => {
                            // Start window drag - if egui takes over (slider, scrollbar),
                            // it will capture the pointer and we'll stop dragging
                            if let Some((x, y)) = state.floating_window.mouse_pos {
                                state.floating_window.on_mouse_press(x as f64, y as f64);
                            }
                        }
                        ElementState::Released => {
                            if let Some((x, y)) = state.floating_window.mouse_pos {
                                state.floating_window.on_mouse_release(x as f64, y as f64);
                            }
                        }
                    }
                }
            }
            WindowEvent::Resized(size) => {
                state.gpu_state.resize(size.width, size.height);
                state.floating_window
                    .event_handler
                    .dispatch(&FloatingWindowEvent::Resize {
                        width: size.width,
                        height: size.height,
                    });
                // Particle system uses content size (excluding margin), not window size
                // Content size stays constant as configured, no need to update on resize
            }
            WindowEvent::Focused(focused) => {
                // Track focus state - only allow dragging when focused
                state.floating_window.set_focus(focused);
            }
            _ => {}
        }
    }

    fn about_to_wait(&mut self, event_loop: &ActiveEventLoop) {
        // Process any pending commands from the controller
        self.process_commands(event_loop);

        // Process menu bar events (tray icon clicks, menu item clicks)
        self.process_menu_bar_events(event_loop);

        // Create any pending windows from commands
        self.create_pending_windows(event_loop);

        // Request redraw for all windows
        for state in self.windows.values() {
            state.window.request_redraw();
        }
    }
}
