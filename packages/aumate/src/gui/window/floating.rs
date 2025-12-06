//! FloatingWindow implementation with GPU rendering

use super::builder::FloatingWindowBuilder;
use super::commands::{CommandReceiver, CommandSender, WindowCommand, WindowRegistry};
use super::config::{Position, WindowConfig, WindowLevel};
use crate::gui::animation::AnimationController;
use crate::gui::content::Content;
use crate::gui::controller::ControllerState;
use crate::gui::effect::{ParticleSystem, PresetEffect, PresetEffectOptions};
use crate::gui::event::{EventHandler, FloatingWindowEvent};
use crate::gui::menu_bar::{MenuBarEvent, MenuBarManager};
use crate::gui::render::WindowPainter;
use crate::gui::shape::{ShapeMask, WindowShape};
use crate::screenshot::ScreenshotMode;

#[cfg(all(feature = "click_helper", target_os = "macos"))]
use crate::click_helper::ClickHelperMode;
#[cfg(all(feature = "window_manager", target_os = "macos"))]
use crate::window_manager::{CommandPalette, PaletteResult};

use egui::{CentralPanel, Color32, Context, Pos2, Rect, Vec2};
use std::sync::Arc;
#[cfg(target_os = "macos")]
use std::sync::Mutex;
use winit::application::ApplicationHandler;

// macOS event loop state for deferred event loop execution
#[cfg(target_os = "macos")]
struct MacOsEventLoopState {
    tx: super::commands::CommandSender,
    rx: super::commands::CommandReceiver,
    ready_tx: Option<std::sync::mpsc::Sender<()>>,
}

#[cfg(target_os = "macos")]
static MACOS_EVENT_LOOP_STATE: Mutex<Option<MacOsEventLoopState>> = Mutex::new(None);

/// Initialized event loop for non-blocking mode (macOS only)
/// Uses thread_local since EventLoop isn't Send/Sync but is always used on main thread
#[cfg(target_os = "macos")]
struct InitializedEventLoop {
    event_loop: EventLoop<()>,
    app: FloatingWindowApp,
}

#[cfg(target_os = "macos")]
thread_local! {
    static INITIALIZED_EVENT_LOOP: std::cell::RefCell<Option<InitializedEventLoop>> = const { std::cell::RefCell::new(None) };
}

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
    pub(crate) drag_offset: Option<(f64, f64)>, // Offset from window origin to mouse click point
    mouse_pos: Option<(f32, f32)>,
    pub(crate) is_dragging: bool,
    /// Whether this window has focus (used to prevent drag when other window is focused)
    has_focus: bool,
    /// Widget-based content for programmatic UI
    widget_content: Option<crate::gui::widget::WidgetDef>,
    /// Widget renderer state
    widget_renderer: crate::gui::widget::WidgetRenderer,
}

impl FloatingWindow {
    /// Create a new builder
    pub fn builder() -> FloatingWindowBuilder {
        FloatingWindowBuilder::new()
    }

    /// Create from configuration
    #[allow(clippy::type_complexity)]
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
            ParticleSystem::new(*effect, options.clone(), content_width, content_height)
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
            widget_content: None,
            widget_renderer: crate::gui::widget::WidgetRenderer::new(),
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

    /// Spawn GUI controller in background thread, return command sender and thread handle.
    ///
    /// **Note**: On macOS, this spawns a thread but the EventLoop must be run on the main thread.
    /// Use `spawn_controller_main_thread` for macOS compatibility.
    ///
    /// # Example
    ///
    /// ```ignore
    /// let (sender, handle) = FloatingWindow::spawn_controller()?;
    ///
    /// // Create windows by sending commands
    /// sender.send(WindowCommand::Create { config, effect: None })?;
    ///
    /// // Wait for GUI thread to exit
    /// handle.join().unwrap();
    /// ```
    #[cfg(not(target_os = "macos"))]
    pub fn spawn_controller() -> Result<(CommandSender, std::thread::JoinHandle<()>), String> {
        let (tx, rx) = super::commands::create_command_channel();
        let sender = tx.clone();

        let handle = std::thread::spawn(move || {
            // Create a minimal hidden controller window
            let controller = FloatingWindow::builder()
                .title("aumate-controller")
                .size(1, 1)
                .position(-10000.0, -10000.0) // Off-screen
                .shape(WindowShape::Rectangle)
                .build()
                .expect("Failed to create controller window");

            let event_loop = EventLoop::new().expect("Failed to create event loop");
            event_loop.set_control_flow(ControlFlow::Poll);

            let mut app = FloatingWindowApp::new_controller(controller, tx, rx);
            let _ = event_loop.run_app(&mut app);
        });

        Ok((sender, handle))
    }

    /// macOS version - returns sender and a closure to run the event loop.
    ///
    /// On macOS, the EventLoop MUST be created and run on the main thread.
    /// This function returns the command sender and a closure that must be called
    /// on the main thread to run the event loop.
    #[cfg(target_os = "macos")]
    pub fn spawn_controller() -> Result<(CommandSender, std::thread::JoinHandle<()>), String> {
        use std::sync::mpsc;

        let (tx, rx) = super::commands::create_command_channel();
        let sender = tx.clone();

        // Create a channel to signal when the GUI is ready
        let (ready_tx, ready_rx) = mpsc::channel::<()>();

        // On macOS, we still spawn a thread but it waits for commands
        // The actual event loop runs when run_event_loop is called
        let handle = std::thread::spawn(move || {
            // Wait for signal that we should exit (this thread doesn't run the event loop)
            let _ = ready_rx.recv();
        });

        // Store the receiver in a static so we can run the event loop later
        *MACOS_EVENT_LOOP_STATE.lock().unwrap() =
            Some(MacOsEventLoopState { tx, rx, ready_tx: Some(ready_tx) });

        Ok((sender, handle))
    }

    /// Run the event loop on the current thread (required for macOS).
    ///
    /// This function blocks until all windows are closed or ExitApplication is sent.
    /// On macOS, this MUST be called from the main thread.
    ///
    /// Uses pump_events to allow periodic yielding for callback processing.
    #[cfg(target_os = "macos")]
    pub fn run_event_loop() -> Result<(), String> {
        use winit::platform::pump_events::{EventLoopExtPumpEvents, PumpStatus};

        let state = MACOS_EVENT_LOOP_STATE
            .lock()
            .unwrap()
            .take()
            .ok_or("Event loop state not initialized")?;

        let controller = FloatingWindow::builder()
            .title("aumate-controller")
            .size(1, 1)
            .position(-10000.0, -10000.0)
            .shape(WindowShape::Rectangle)
            .build()?;

        let mut event_loop = EventLoop::new().map_err(|e| e.to_string())?;
        event_loop.set_control_flow(ControlFlow::Poll);

        let mut app = FloatingWindowApp::new_controller(controller, state.tx, state.rx);

        // Signal that we're done if the handle thread is waiting
        if let Some(ready_tx) = state.ready_tx {
            let _ = ready_tx.send(());
        }

        // Use pump_events loop to allow yielding to Node.js event loop
        loop {
            let status = event_loop.pump_app_events(None, &mut app);

            match status {
                PumpStatus::Exit(code) => {
                    return if code == 0 { Ok(()) } else { Err(format!("Exit code: {}", code)) };
                }
                PumpStatus::Continue => {
                    // Small sleep to avoid busy-waiting and allow Node.js to process callbacks
                    std::thread::sleep(std::time::Duration::from_millis(1));
                }
            }
        }
    }

    /// Initialize the event loop for non-blocking mode (macOS only).
    ///
    /// Call this once before using `run_event_loop_once()`.
    /// Returns an error if already initialized.
    #[cfg(target_os = "macos")]
    pub fn init_event_loop() -> Result<(), String> {
        INITIALIZED_EVENT_LOOP.with(|cell| {
            let mut initialized = cell.borrow_mut();
            if initialized.is_some() {
                return Err("Event loop already initialized".to_string());
            }

            // Take the state from MACOS_EVENT_LOOP_STATE
            let state = MACOS_EVENT_LOOP_STATE
                .lock()
                .unwrap()
                .take()
                .ok_or("Event loop state not set (call spawn_controller first)")?;

            let controller = FloatingWindow::builder()
                .title("aumate-controller")
                .size(1, 1)
                .position(-10000.0, -10000.0)
                .shape(WindowShape::Rectangle)
                .build()?;

            let event_loop = EventLoop::new().map_err(|e| e.to_string())?;
            event_loop.set_control_flow(ControlFlow::Poll);

            let app = FloatingWindowApp::new_controller(controller, state.tx, state.rx);

            // Signal that we're done if the handle thread is waiting
            if let Some(ready_tx) = state.ready_tx {
                let _ = ready_tx.send(());
            }

            *initialized = Some(InitializedEventLoop { event_loop, app });

            Ok(())
        })
    }

    #[cfg(not(target_os = "macos"))]
    pub fn init_event_loop() -> Result<(), String> {
        Ok(())
    }

    /// Pump the event loop once (non-blocking).
    ///
    /// Returns `true` if the app should continue running, `false` if it should exit.
    /// You must call `init_event_loop()` first.
    #[cfg(target_os = "macos")]
    pub fn run_event_loop_once() -> Result<bool, String> {
        use winit::platform::pump_events::{EventLoopExtPumpEvents, PumpStatus};

        INITIALIZED_EVENT_LOOP.with(|cell| {
            let mut guard = cell.borrow_mut();
            let state =
                guard.as_mut().ok_or("Event loop not initialized (call init_event_loop first)")?;

            let status = state.event_loop.pump_app_events(None, &mut state.app);

            match status {
                PumpStatus::Exit(_) => Ok(false),
                PumpStatus::Continue => Ok(true),
            }
        })
    }

    #[cfg(not(target_os = "macos"))]
    pub fn run_event_loop_once() -> Result<bool, String> {
        // On non-macOS, the event loop runs in a background thread
        // This is effectively a no-op but allows cross-platform code
        Ok(true)
    }

    /// Run the event loop on the current thread (no-op on non-macOS).
    #[cfg(not(target_os = "macos"))]
    pub fn run_event_loop() -> Result<(), String> {
        // On non-macOS, the event loop runs in the spawned thread
        Ok(())
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
            let event = if visible { FloatingWindowEvent::Show } else { FloatingWindowEvent::Hide };
            self.event_handler.dispatch(&event);
        }
    }

    /// Set content (image/text)
    pub fn set_content(&mut self, content: Option<Content>) {
        self.config.content = content;
    }

    /// Set widget-based content
    pub fn set_widget_content(&mut self, content: Option<crate::gui::widget::WidgetDef>) {
        self.widget_content = content;
        // Clear renderer state when content changes
        self.widget_renderer.clear_state();
    }

    /// Update a widget's state by ID
    pub fn update_widget(&mut self, widget_id: &str, update: &super::commands::WidgetUpdate) {
        use super::commands::WidgetUpdate;
        use crate::gui::widget::WidgetStateUpdate;

        let state_update = match update {
            WidgetUpdate::SetText(text) => WidgetStateUpdate::SetText(text.clone()),
            WidgetUpdate::SetChecked(checked) => WidgetStateUpdate::SetChecked(*checked),
            WidgetUpdate::SetValue(value) => WidgetStateUpdate::SetValue(*value),
            WidgetUpdate::SetVisible(_) | WidgetUpdate::SetEnabled(_) => {
                // These require modifying the widget definition, not just state
                // For now, we just ignore them
                return;
            }
        };

        self.widget_renderer.update_state(&widget_id.to_string(), state_update);
    }

    /// Update particle system
    pub fn update(&mut self) {
        if let Some(ref mut system) = self.particle_system {
            system.update();
        }
    }

    /// Render the window content
    pub fn render(&mut self, ctx: &Context, _rect: Rect) -> Vec<crate::gui::widget::WidgetEvent> {
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

        // Collect widget events
        let mut widget_events = Vec::new();

        // Check if we have widget content - if so, render that instead
        if let Some(ref widget_content) = self.widget_content.clone() {
            CentralPanel::default().frame(transparent_frame).show(ctx, |ui| {
                // Draw background for widget content
                let bg_color = Color32::from_rgba_unmultiplied(40, 40, 50, 255);
                ui.painter().rect_filled(content_rect, 4.0, bg_color);

                // Constrain UI to content rect
                #[allow(deprecated)]
                ui.allocate_ui_at_rect(content_rect, |ui| {
                    ui.style_mut().spacing.item_spacing = egui::vec2(4.0, 4.0);
                    self.widget_renderer.render(ui, widget_content, &mut widget_events);
                });

                // Render particles on top
                if let Some(ref system) = self.particle_system {
                    WindowPainter::render_particles(ui, system, Pos2::new(margin, margin));
                }
            });
        } else {
            CentralPanel::default().frame(transparent_frame).show(ctx, |ui| {
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

        widget_events
    }

    /// Handle mouse press - x, y are in window-local coordinates
    pub fn on_mouse_press(&mut self, x: f64, y: f64) {
        // Only handle drag if window has focus (prevents drag when clicking through other windows)
        if self.has_focus && self.config.draggable && self.shape_mask.contains(x as f32, y as f32) {
            // Store the offset from window origin to mouse position
            self.drag_offset = Some((x, y));
            self.is_dragging = true;
            self.event_handler
                .dispatch(&FloatingWindowEvent::DragStart { x: x as f32, y: y as f32 });
        }
        self.event_handler.dispatch(&FloatingWindowEvent::Click { x: x as f32, y: y as f32 });
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
            self.event_handler.dispatch(&FloatingWindowEvent::DragEnd { x: x as f32, y: y as f32 });
        }
    }

    /// Handle mouse move during drag
    /// screen_x, screen_y are global screen coordinates of mouse (physical)
    /// scale_factor is used to convert logical offset to physical
    /// Returns new window position in screen coordinates (physical)
    pub fn on_drag_move(
        &mut self,
        screen_x: f64,
        screen_y: f64,
        scale_factor: f64,
    ) -> Option<(f64, f64)> {
        if self.is_dragging
            && let Some((offset_x, offset_y)) = self.drag_offset
        {
            // Convert logical offset to physical
            let physical_offset_x = offset_x * scale_factor;
            let physical_offset_y = offset_y * scale_factor;

            // New window position = mouse screen position - offset (both in physical)
            let new_x = screen_x - physical_offset_x;
            let new_y = screen_y - physical_offset_y;

            self.event_handler
                .dispatch(&FloatingWindowEvent::Drag { x: new_x as f32, y: new_y as f32 });

            return Some((new_x, new_y));
        }
        None
    }

    /// Handle mouse move (for tracking position, not dragging)
    pub fn on_mouse_move(&mut self, x: f64, y: f64) {
        self.mouse_pos = Some((x as f32, y as f32));
        self.event_handler.dispatch(&FloatingWindowEvent::MouseMove { x: x as f32, y: y as f32 });
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
            .find(|f| {
                **f == wgpu::TextureFormat::Bgra8Unorm || **f == wgpu::TextureFormat::Rgba8Unorm
            })
            .copied()
            .unwrap_or_else(|| {
                // Fallback to first non-sRGB, or just first format
                surface_caps
                    .formats
                    .iter()
                    .find(|f| !f.is_srgb())
                    .copied()
                    .unwrap_or(surface_caps.formats[0])
            });

        // Use PreMultiplied alpha mode for proper compositing with transparent windows
        let alpha_mode =
            if surface_caps.alpha_modes.contains(&wgpu::CompositeAlphaMode::PreMultiplied) {
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

        Self { surface, device, queue, surface_config, egui_renderer }
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

/// State for screenshot mode window
struct ScreenshotWindowState {
    /// The screenshot mode controller
    mode: ScreenshotMode,
    /// The fullscreen window
    window: Arc<Window>,
    /// GPU state
    gpu_state: GpuState,
    /// egui context
    egui_ctx: Context,
    /// egui state
    egui_state: egui_winit::State,
}

/// State for Click Helper mode window
#[cfg(all(feature = "click_helper", target_os = "macos"))]
struct ClickHelperWindowState {
    /// The Click Helper mode controller
    mode: ClickHelperMode,
    /// The fullscreen window
    window: Arc<Window>,
    /// GPU state
    gpu_state: GpuState,
    /// egui context
    egui_ctx: Context,
    /// egui state
    egui_state: egui_winit::State,
}

/// State for Window Manager palette window
#[cfg(all(feature = "window_manager", target_os = "macos"))]
struct WindowManagerWindowState {
    /// The command palette
    palette: CommandPalette,
    /// The palette window
    window: Arc<Window>,
    /// GPU state
    gpu_state: GpuState,
    /// egui context
    egui_ctx: Context,
    /// egui state
    egui_state: egui_winit::State,
    /// Whether the window has ever been focused (to ignore initial focus loss)
    was_focused: bool,
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
    /// Event senders for widget events by window name
    event_senders: std::collections::HashMap<String, super::commands::WidgetEventSender>,
    /// Menu bar manager for tray icons
    menu_bar_manager: MenuBarManager,
    /// Screenshot mode state (if active)
    screenshot_state: Option<ScreenshotWindowState>,
    /// Pending screenshot mode start request
    pending_screenshot_start: Option<Vec<String>>,
    /// Click Helper mode state (if active)
    #[cfg(all(feature = "click_helper", target_os = "macos"))]
    click_helper_state: Option<ClickHelperWindowState>,
    /// Pending Click Helper mode start request
    #[cfg(all(feature = "click_helper", target_os = "macos"))]
    pending_click_helper_start: bool,
    /// Window Manager palette state (if active)
    #[cfg(all(feature = "window_manager", target_os = "macos"))]
    window_manager_state: Option<WindowManagerWindowState>,
    /// Pending Window Manager palette start request
    #[cfg(all(feature = "window_manager", target_os = "macos"))]
    pending_window_manager_start: bool,
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
            event_senders: std::collections::HashMap::new(),
            menu_bar_manager: MenuBarManager::new(),
            screenshot_state: None,
            pending_screenshot_start: None,
            #[cfg(all(feature = "click_helper", target_os = "macos"))]
            click_helper_state: None,
            #[cfg(all(feature = "click_helper", target_os = "macos"))]
            pending_click_helper_start: false,
            #[cfg(all(feature = "window_manager", target_os = "macos"))]
            window_manager_state: None,
            #[cfg(all(feature = "window_manager", target_os = "macos"))]
            pending_window_manager_start: false,
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
            event_senders: std::collections::HashMap::new(),
            menu_bar_manager: MenuBarManager::new(),
            screenshot_state: None,
            pending_screenshot_start: None,
            #[cfg(all(feature = "click_helper", target_os = "macos"))]
            click_helper_state: None,
            #[cfg(all(feature = "click_helper", target_os = "macos"))]
            pending_click_helper_start: false,
            #[cfg(all(feature = "window_manager", target_os = "macos"))]
            window_manager_state: None,
            #[cfg(all(feature = "window_manager", target_os = "macos"))]
            pending_window_manager_start: false,
        }
    }

    fn new_controller(
        controller_window: FloatingWindow,
        command_sender: CommandSender,
        command_receiver: CommandReceiver,
    ) -> Self {
        Self {
            pending_windows: vec![controller_window],
            windows: std::collections::HashMap::new(),
            command_receiver: Some(command_receiver),
            command_sender: Some(command_sender),
            registry: WindowRegistry::new(),
            controller_window_id: None,
            pending_configs: Vec::new(),
            event_senders: std::collections::HashMap::new(),
            menu_bar_manager: MenuBarManager::new(),
            screenshot_state: None,
            pending_screenshot_start: None,
            #[cfg(all(feature = "click_helper", target_os = "macos"))]
            click_helper_state: None,
            #[cfg(all(feature = "click_helper", target_os = "macos"))]
            pending_click_helper_start: false,
            #[cfg(all(feature = "window_manager", target_os = "macos"))]
            window_manager_state: None,
            #[cfg(all(feature = "window_manager", target_os = "macos"))]
            pending_window_manager_start: false,
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
                        if let Some(id) = self.registry.find_by_name(&name)
                            && self.windows.remove(&id).is_some()
                        {
                            self.registry.unregister(id);
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
                                    *system = ParticleSystem::immediate(
                                        *effect,
                                        options.clone(),
                                        width,
                                        height,
                                    );
                                }
                            }
                            self.registry.update_options(id, options);
                        }
                    }
                    WindowCommand::CloseAll => {
                        log::info!("Closing all managed windows");
                        let managed_ids: Vec<WindowId> = self
                            .windows
                            .iter()
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
                    WindowCommand::StartScreenshotMode { enabled_actions } => {
                        log::info!("Starting screenshot mode with actions: {:?}", enabled_actions);
                        self.pending_screenshot_start = Some(enabled_actions);
                    }
                    WindowCommand::ExitScreenshotMode => {
                        log::info!("Exiting screenshot mode");
                        self.screenshot_state = None;
                    }
                    WindowCommand::ExitApplication => {
                        log::info!("Exiting application");
                        event_loop.exit();
                    }
                    WindowCommand::StartClickHelperMode => {
                        #[cfg(all(feature = "click_helper", target_os = "macos"))]
                        {
                            log::info!("StartClickHelperMode command received");
                            self.pending_click_helper_start = true;
                        }
                        #[cfg(not(all(feature = "click_helper", target_os = "macos")))]
                        {
                            log::warn!(
                                "Click Helper is only available on macOS with click_helper feature"
                            );
                        }
                    }
                    WindowCommand::ExitClickHelperMode => {
                        #[cfg(all(feature = "click_helper", target_os = "macos"))]
                        {
                            log::info!("Exiting Click Helper mode");
                            self.click_helper_state = None;
                        }
                    }
                    WindowCommand::StartWindowManagerPalette => {
                        #[cfg(all(feature = "window_manager", target_os = "macos"))]
                        {
                            log::info!("StartWindowManagerPalette command received");
                            self.pending_window_manager_start = true;
                        }
                        #[cfg(not(all(feature = "window_manager", target_os = "macos")))]
                        {
                            log::warn!(
                                "Window Manager is only available on macOS with window_manager feature"
                            );
                        }
                    }
                    WindowCommand::ExitWindowManagerPalette => {
                        #[cfg(all(feature = "window_manager", target_os = "macos"))]
                        {
                            log::info!("Exiting Window Manager palette");
                            self.window_manager_state = None;
                        }
                    }
                    WindowCommand::SetWidgetContent { id, content } => {
                        log::info!("Setting widget content for window {:?}", id);
                        if let Some(state) = self.windows.get_mut(&id) {
                            state.floating_window.set_widget_content(Some(content));
                        }
                    }
                    WindowCommand::UpdateWidget { widget_id, update } => {
                        log::info!("Updating widget: {}", widget_id);
                        // Find the window containing this widget and update it
                        for state in self.windows.values_mut() {
                            state.floating_window.update_widget(&widget_id, &update);
                        }
                    }
                    WindowCommand::RegisterEventCallback { window_name, event_sender } => {
                        log::info!("Registering event callback for window: {}", window_name);
                        self.event_senders.insert(window_name, event_sender);
                    }
                    WindowCommand::ShowOpenFileDialog { request_id, window_name, options } => {
                        log::info!("Showing open file dialog for window: {}", window_name);
                        let result = Self::show_open_file_dialog_sync(&options);
                        // Emit result as event to the window's callback
                        if let Some(sender) = self.event_senders.get(&window_name) {
                            let event = crate::gui::widget::WidgetEvent::FileDialogCompleted {
                                id: request_id,
                                paths: result.paths,
                                cancelled: result.cancelled,
                            };
                            let _ = sender.send((window_name.clone(), event));
                        }
                    }
                    WindowCommand::ShowSaveFileDialog { request_id, window_name, options } => {
                        log::info!("Showing save file dialog for window: {}", window_name);
                        let result = Self::show_save_file_dialog_sync(&options);
                        // Emit result as event to the window's callback
                        if let Some(sender) = self.event_senders.get(&window_name) {
                            let event = crate::gui::widget::WidgetEvent::FileDialogCompleted {
                                id: request_id,
                                paths: result.paths,
                                cancelled: result.cancelled,
                            };
                            let _ = sender.send((window_name.clone(), event));
                        }
                    }
                    WindowCommand::ShowFolderDialog { request_id, window_name, options } => {
                        log::info!("Showing folder dialog for window: {}", window_name);
                        let result = Self::show_folder_dialog_sync(&options);
                        // Emit result as event to the window's callback
                        if let Some(sender) = self.event_senders.get(&window_name) {
                            let event = crate::gui::widget::WidgetEvent::FileDialogCompleted {
                                id: request_id,
                                paths: result.paths,
                                cancelled: result.cancelled,
                            };
                            let _ = sender.send((window_name.clone(), event));
                        }
                    }
                }
            }
        }
    }

    /// Show open file dialog synchronously (runs on main thread)
    fn show_open_file_dialog_sync(
        options: &super::commands::FileDialogOptions,
    ) -> super::commands::FileDialogResult {
        use rfd::FileDialog;

        let mut dialog = FileDialog::new();

        if let Some(ref title) = options.title {
            dialog = dialog.set_title(title);
        }
        if let Some(ref dir) = options.directory {
            dialog = dialog.set_directory(dir);
        }
        for (name, extensions) in &options.filters {
            let ext_refs: Vec<&str> = extensions.iter().map(|s| s.as_str()).collect();
            dialog = dialog.add_filter(name, &ext_refs);
        }

        let result = if options.multiple {
            dialog.pick_files()
        } else {
            dialog.pick_file().map(|f| vec![f])
        };

        match result {
            Some(files) => super::commands::FileDialogResult {
                paths: files.into_iter().map(|f| f.to_string_lossy().to_string()).collect(),
                cancelled: false,
            },
            None => super::commands::FileDialogResult { paths: vec![], cancelled: true },
        }
    }

    /// Show save file dialog synchronously (runs on main thread)
    fn show_save_file_dialog_sync(
        options: &super::commands::FileDialogOptions,
    ) -> super::commands::FileDialogResult {
        use rfd::FileDialog;

        let mut dialog = FileDialog::new();

        if let Some(ref title) = options.title {
            dialog = dialog.set_title(title);
        }
        if let Some(ref dir) = options.directory {
            dialog = dialog.set_directory(dir);
        }
        if let Some(ref name) = options.default_name {
            dialog = dialog.set_file_name(name);
        }
        for (name, extensions) in &options.filters {
            let ext_refs: Vec<&str> = extensions.iter().map(|s| s.as_str()).collect();
            dialog = dialog.add_filter(name, &ext_refs);
        }

        match dialog.save_file() {
            Some(file) => super::commands::FileDialogResult {
                paths: vec![file.to_string_lossy().to_string()],
                cancelled: false,
            },
            None => super::commands::FileDialogResult { paths: vec![], cancelled: true },
        }
    }

    /// Show folder picker dialog synchronously (runs on main thread)
    fn show_folder_dialog_sync(
        options: &super::commands::FileDialogOptions,
    ) -> super::commands::FileDialogResult {
        use rfd::FileDialog;

        let mut dialog = FileDialog::new();

        if let Some(ref title) = options.title {
            dialog = dialog.set_title(title);
        }
        if let Some(ref dir) = options.directory {
            dialog = dialog.set_directory(dir);
        }

        match dialog.pick_folder() {
            Some(folder) => super::commands::FileDialogResult {
                paths: vec![folder.to_string_lossy().to_string()],
                cancelled: false,
            },
            None => super::commands::FileDialogResult { paths: vec![], cancelled: true },
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
                Ok(mut floating_window) => {
                    // Set widget content if provided in config
                    if let Some(widget_content) = config.widget_content.clone() {
                        floating_window.set_widget_content(Some(widget_content));
                    }

                    // Create the window
                    let margin = floating_window.effect_margin;
                    let window_width = config.size.width as f32 + margin * 2.0;
                    let window_height = config.size.height as f32 + margin * 2.0;

                    let attrs = WindowAttributes::default()
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
                            self.registry.register(
                                window_id,
                                window_name,
                                window_size,
                                effect_type,
                                effect_opts,
                            );

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

    /// Create screenshot mode window
    fn create_screenshot_window(
        &mut self,
        event_loop: &ActiveEventLoop,
        enabled_actions: Vec<String>,
    ) {
        // Get primary monitor from winit to get correct size and position
        let primary_monitor =
            event_loop.primary_monitor().or_else(|| event_loop.available_monitors().next());

        let (size, position) = if let Some(mon) = primary_monitor {
            // Use winit's monitor size (physical pixels, correct for the display)
            (mon.size(), mon.position())
        } else {
            log::error!("No monitors found for screenshot");
            return;
        };

        log::info!(
            "Creating screenshot window: pos=({}, {}), size={}x{} (physical from winit monitor)",
            position.x,
            position.y,
            size.width,
            size.height
        );

        // Create fullscreen transparent window
        // Start invisible, configure macOS window level, then show
        let attrs = WindowAttributes::default()
            .with_title("Screenshot")
            .with_inner_size(size)
            .with_position(position)
            .with_decorations(false)
            .with_transparent(true)
            .with_resizable(false)
            .with_visible(false); // Start invisible, show after configuration

        match event_loop.create_window(attrs) {
            Ok(window) => {
                // Configure macOS-specific window properties to cover menu bar
                #[cfg(target_os = "macos")]
                Self::configure_macos_screenshot_window(&window);

                // Log actual window size after creation
                let actual_size = window.inner_size();
                log::info!(
                    "Screenshot window created: actual_size={}x{}, requested_size={}x{}",
                    actual_size.width,
                    actual_size.height,
                    size.width,
                    size.height
                );

                let window = Arc::new(window);
                let scale_factor = window.scale_factor();

                // Create screenshot mode
                match ScreenshotMode::new(enabled_actions, scale_factor) {
                    Ok(mut mode) => {
                        // Set the correct screen size from window's actual size (logical pixels)
                        let actual_size = window.inner_size();
                        let logical_width = actual_size.width as f32 / scale_factor as f32;
                        let logical_height = actual_size.height as f32 / scale_factor as f32;
                        mode.set_screen_size(logical_width, logical_height);

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

                        // Now show the window
                        window.set_visible(true);

                        self.screenshot_state = Some(ScreenshotWindowState {
                            mode,
                            window,
                            gpu_state,
                            egui_ctx,
                            egui_state,
                        });

                        log::info!("Screenshot mode started");
                    }
                    Err(e) => {
                        log::error!("Failed to create screenshot mode: {}", e);
                    }
                }
            }
            Err(e) => {
                log::error!("Failed to create screenshot window: {}", e);
            }
        }
    }

    /// Configure macOS-specific window properties for screenshot overlay
    #[cfg(target_os = "macos")]
    fn configure_macos_screenshot_window(window: &Window) {
        use raw_window_handle::{HasWindowHandle, RawWindowHandle};

        let handle = match window.window_handle() {
            Ok(h) => h,
            Err(_) => return,
        };

        if let RawWindowHandle::AppKit(handle) = handle.as_raw() {
            // Allow unexpected_cfgs from objc crate's msg_send! macro
            #[allow(unexpected_cfgs)]
            unsafe {
                use objc::runtime::{Class, Object};
                use objc::{msg_send, sel, sel_impl};

                // Get NSWindow from NSView
                let ns_view: *mut Object = handle.ns_view.as_ptr() as *mut Object;
                let ns_window: *mut Object = msg_send![ns_view, window];

                // Set window level to overlay (25) to cover menu bar and dock
                // kCGOverlayWindowLevelKey = 25
                let _: () = msg_send![ns_window, setLevel: 25i64];

                // Set collection behavior to stay on all spaces
                // NSWindowCollectionBehaviorCanJoinAllSpaces = 128
                let _: () = msg_send![ns_window, setCollectionBehavior: 128u64];

                // Set up transparent window
                let _: () = msg_send![ns_window, setAlphaValue: 1.0];

                // Set backgroundColor to clearColor
                let ns_color_class = match Class::get("NSColor") {
                    Some(c) => c,
                    None => return,
                };
                let clear_color: *mut Object = msg_send![ns_color_class, clearColor];
                let _: () = msg_send![ns_window, setBackgroundColor: clear_color];

                // Turn off opacity so transparent parts are actually transparent
                let _: () = msg_send![ns_window, setOpaque: false];

                // Disable shadow for transparent windows
                let _: () = msg_send![ns_window, setHasShadow: false];

                log::info!("Configured macOS screenshot window with overlay level");
            }
        }
    }

    /// Configure macOS-specific window properties for palette window
    #[cfg(all(feature = "window_manager", target_os = "macos"))]
    fn configure_macos_palette_window(window: &Window) {
        use raw_window_handle::{HasWindowHandle, RawWindowHandle};

        let handle = match window.window_handle() {
            Ok(h) => h,
            Err(_) => return,
        };

        if let RawWindowHandle::AppKit(handle) = handle.as_raw() {
            #[allow(unexpected_cfgs)]
            unsafe {
                use objc::runtime::{Class, Object, YES};
                use objc::{msg_send, sel, sel_impl};

                // Get NSWindow from NSView
                let ns_view: *mut Object = handle.ns_view.as_ptr() as *mut Object;
                let ns_window: *mut Object = msg_send![ns_view, window];

                // Set window level to floating (3) to appear above normal windows
                // kCGFloatingWindowLevel = 3
                let _: () = msg_send![ns_window, setLevel: 3i64];

                // Set collection behavior to stay on all spaces
                // NSWindowCollectionBehaviorCanJoinAllSpaces = 128 (1 << 7)
                let _: () = msg_send![ns_window, setCollectionBehavior: 128u64];

                // Make window non-opaque for transparency
                let _: () = msg_send![ns_window, setOpaque: false];

                // Set clear background so our egui content shows
                let ns_color_class = match Class::get("NSColor") {
                    Some(c) => c,
                    None => return,
                };
                let clear_color: *mut Object = msg_send![ns_color_class, clearColor];
                let _: () = msg_send![ns_window, setBackgroundColor: clear_color];

                // Enable shadow for better visibility
                let _: () = msg_send![ns_window, setHasShadow: true];

                // Activate the application and bring window to front for keyboard focus
                if let Some(ns_app_class) = Class::get("NSApplication") {
                    let ns_app: *mut Object = msg_send![ns_app_class, sharedApplication];
                    // Activate the app, ignoring other apps
                    let _: () = msg_send![ns_app, activateIgnoringOtherApps: YES];
                }

                // Make window key and bring to front
                let _: () = msg_send![ns_window, makeKeyAndOrderFront: std::ptr::null::<Object>()];

                log::info!("Configured macOS palette window with floating level and focus");
            }
        }
    }

    /// Create Click Helper mode window
    #[cfg(all(feature = "click_helper", target_os = "macos"))]
    fn create_click_helper_window(&mut self, event_loop: &ActiveEventLoop) {
        // Get primary monitor from winit to get correct size and position
        let primary_monitor =
            event_loop.primary_monitor().or_else(|| event_loop.available_monitors().next());

        let (size, position) = if let Some(mon) = primary_monitor {
            (mon.size(), mon.position())
        } else {
            log::error!("No monitors found for Click Helper");
            return;
        };

        log::info!(
            "Creating Click Helper window: pos=({}, {}), size={}x{}",
            position.x,
            position.y,
            size.width,
            size.height
        );

        let attrs = WindowAttributes::default()
            .with_title("Click Helper")
            .with_inner_size(size)
            .with_position(position)
            .with_decorations(false)
            .with_transparent(true)
            .with_resizable(false)
            .with_visible(false);

        match event_loop.create_window(attrs) {
            Ok(window) => {
                // Configure macOS-specific window properties
                Self::configure_macos_screenshot_window(&window);

                let window = Arc::new(window);

                // Create Click Helper mode
                let mut mode = ClickHelperMode::new();
                match mode.activate() {
                    Ok(()) => {
                        if !mode.is_active() {
                            log::warn!("Click Helper mode did not activate (no elements found?)");
                            return;
                        }

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

                        window.set_visible(true);

                        self.click_helper_state = Some(ClickHelperWindowState {
                            mode,
                            window,
                            gpu_state,
                            egui_ctx,
                            egui_state,
                        });

                        log::info!("Click Helper mode started");
                    }
                    Err(e) => {
                        log::error!("Failed to activate Click Helper mode: {}", e);
                    }
                }
            }
            Err(e) => {
                log::error!("Failed to create Click Helper window: {}", e);
            }
        }
    }

    /// Create Window Manager palette window
    #[cfg(all(feature = "window_manager", target_os = "macos"))]
    fn create_window_manager_window(&mut self, event_loop: &ActiveEventLoop) {
        use crate::window_manager::{get_frontmost_app_pid_excluding, is_accessibility_trusted};

        // Toggle behavior: if palette is already open, close it
        if self.window_manager_state.is_some() {
            log::info!("Window Manager palette already open, closing (toggle)");
            self.window_manager_state = None;
            return;
        }

        // Check accessibility permission
        if !is_accessibility_trusted() {
            log::warn!("Window Manager requires accessibility permission");
            return;
        }

        // Get the frontmost app PID (excluding ourselves)
        let our_pid = std::process::id() as i32;
        let target_pid = match get_frontmost_app_pid_excluding(our_pid) {
            Some(pid) => pid,
            None => {
                log::warn!("No frontmost application found");
                return;
            }
        };

        log::info!("Window Manager targeting PID: {}", target_pid);

        // Get primary monitor for centering
        let primary_monitor =
            event_loop.primary_monitor().or_else(|| event_loop.available_monitors().next());

        let (screen_size, screen_pos) = if let Some(mon) = primary_monitor {
            (mon.size(), mon.position())
        } else {
            log::error!("No monitors found for Window Manager");
            return;
        };

        // Palette window size - 2x width, 3x height as requested
        let window_width = 1000;
        let window_height = 700;

        // Center on screen, slightly above center
        let x = screen_pos.x + (screen_size.width as i32 - window_width) / 2;
        let y = screen_pos.y + (screen_size.height as i32 - window_height) / 4;

        log::info!(
            "Creating Window Manager palette: pos=({}, {}), size={}x{}",
            x,
            y,
            window_width,
            window_height
        );

        let attrs = WindowAttributes::default()
            .with_title("Window Manager")
            .with_inner_size(winit::dpi::PhysicalSize::new(
                window_width as u32,
                window_height as u32,
            ))
            .with_position(winit::dpi::PhysicalPosition::new(x, y))
            .with_decorations(false)
            .with_transparent(true)
            .with_resizable(false)
            .with_visible(false);

        match event_loop.create_window(attrs) {
            Ok(window) => {
                // Configure macOS-specific window properties
                #[cfg(target_os = "macos")]
                Self::configure_macos_palette_window(&window);

                let window = Arc::new(window);

                // Create command palette
                let mut palette = CommandPalette::new();
                palette.show(target_pid);

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

                window.set_visible(true);

                self.window_manager_state = Some(WindowManagerWindowState {
                    palette,
                    window,
                    gpu_state,
                    egui_ctx,
                    egui_state,
                    was_focused: false,
                });

                log::info!("Window Manager palette started");
            }
            Err(e) => {
                log::error!("Failed to create Window Manager window: {}", e);
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

            let window =
                Arc::new(event_loop.create_window(attrs).expect("Failed to create window"));

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
            let controller_state =
                if self.controller_window_id.is_none() && self.command_sender.is_some() {
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
        // Handle screenshot window events
        let mut should_close_screenshot = false;
        if let Some(ref mut screenshot_state) = self.screenshot_state {
            if screenshot_state.window.id() == window_id {
                // Let egui handle the event first
                let _response =
                    screenshot_state.egui_state.on_window_event(&screenshot_state.window, &event);

                match &event {
                    WindowEvent::CloseRequested => {
                        should_close_screenshot = true;
                    }
                    WindowEvent::RedrawRequested => {
                        let size = screenshot_state.window.inner_size();
                        if size.width == 0 || size.height == 0 {
                            return;
                        }

                        let raw_input =
                            screenshot_state.egui_state.take_egui_input(&screenshot_state.window);

                        let full_output = screenshot_state.egui_ctx.run(raw_input, |ctx| {
                            screenshot_state.mode.render(ctx);
                        });

                        screenshot_state.egui_state.handle_platform_output(
                            &screenshot_state.window,
                            full_output.platform_output,
                        );

                        let primitives = screenshot_state
                            .egui_ctx
                            .tessellate(full_output.shapes, full_output.pixels_per_point);

                        let screen_descriptor = egui_wgpu::ScreenDescriptor {
                            size_in_pixels: [size.width, size.height],
                            pixels_per_point: full_output.pixels_per_point,
                        };

                        screenshot_state.gpu_state.render(
                            primitives,
                            full_output.textures_delta,
                            screen_descriptor,
                        );
                        // Check if mode should exit after render
                        if screenshot_state.mode.should_exit() {
                            should_close_screenshot = true;
                        }
                    }
                    WindowEvent::CursorMoved { position, .. } => {
                        // Convert physical to logical coordinates before passing to screenshot mode
                        let scale_factor = screenshot_state.window.scale_factor();
                        let logical_x = position.x / scale_factor;
                        let logical_y = position.y / scale_factor;
                        screenshot_state
                            .mode
                            .handle_cursor_move((logical_x as f32, logical_y as f32));
                    }
                    _ => {
                        // Forward other events to screenshot mode
                        let _ = screenshot_state.mode.handle_event(&event);
                        // Check if mode should exit after handling event
                        if screenshot_state.mode.should_exit() {
                            should_close_screenshot = true;
                        }
                    }
                }

                // Return early if we're still in screenshot mode and didn't need to close
                if !should_close_screenshot {
                    return;
                }
            }
        }

        // Close screenshot if needed (after borrow ends)
        if should_close_screenshot {
            self.screenshot_state = None;
            log::info!("Screenshot mode closed");
            return;
        }

        // Handle Click Helper window events
        #[cfg(all(feature = "click_helper", target_os = "macos"))]
        {
            let mut should_close_click_helper = false;
            let mut click_position: Option<(f32, f32)> = None;

            if let Some(ref mut click_helper_state) = self.click_helper_state {
                if click_helper_state.window.id() == window_id {
                    // Let egui handle the event first
                    let _response = click_helper_state
                        .egui_state
                        .on_window_event(&click_helper_state.window, &event);

                    match &event {
                        WindowEvent::CloseRequested => {
                            should_close_click_helper = true;
                        }
                        WindowEvent::RedrawRequested => {
                            let size = click_helper_state.window.inner_size();
                            if size.width == 0 || size.height == 0 {
                                return;
                            }

                            let raw_input = click_helper_state
                                .egui_state
                                .take_egui_input(&click_helper_state.window);

                            let full_output = click_helper_state.egui_ctx.run(raw_input, |ctx| {
                                click_helper_state.mode.render(ctx);
                            });

                            click_helper_state.egui_state.handle_platform_output(
                                &click_helper_state.window,
                                full_output.platform_output,
                            );

                            let primitives = click_helper_state
                                .egui_ctx
                                .tessellate(full_output.shapes, full_output.pixels_per_point);

                            let screen_descriptor = egui_wgpu::ScreenDescriptor {
                                size_in_pixels: [size.width, size.height],
                                pixels_per_point: full_output.pixels_per_point,
                            };

                            click_helper_state.gpu_state.render(
                                primitives,
                                full_output.textures_delta,
                                screen_descriptor,
                            );

                            if !click_helper_state.mode.is_active() {
                                should_close_click_helper = true;
                            }
                        }
                        WindowEvent::KeyboardInput { event: key_event, .. } => {
                            use winit::keyboard::{Key, NamedKey};
                            if key_event.state == ElementState::Pressed {
                                match &key_event.logical_key {
                                    Key::Named(NamedKey::Escape) => {
                                        click_helper_state.mode.handle_escape();
                                        should_close_click_helper = true;
                                    }
                                    Key::Named(NamedKey::Backspace) => {
                                        click_helper_state.mode.handle_backspace();
                                    }
                                    Key::Character(c) => {
                                        if let Some(ch) = c.chars().next() {
                                            let action = click_helper_state.mode.handle_key(ch);
                                            if let crate::click_helper::ClickHelperAction::Click(
                                                pos,
                                            ) = action
                                            {
                                                click_position = Some(pos);
                                                should_close_click_helper = true;
                                            }
                                        }
                                    }
                                    _ => {}
                                }
                            }
                        }
                        _ => {}
                    }

                    if !should_close_click_helper {
                        return;
                    }
                }
            }

            if should_close_click_helper {
                self.click_helper_state = None;
                log::info!("Click Helper mode closed");

                // Perform the click if we have a position
                if let Some((x, y)) = click_position {
                    log::info!("Performing click at ({}, {})", x, y);
                    // Use the Mouse struct to perform the click
                    if let Ok(mouse) = crate::input::Mouse::new() {
                        if let Err(e) = mouse.move_mouse(x as i32, y as i32) {
                            log::error!("Failed to move mouse: {}", e);
                        }
                        if let Err(e) = mouse.click(crate::input::MouseButton::Left) {
                            log::error!("Failed to click: {}", e);
                        }
                    } else {
                        log::error!("Failed to create Mouse instance");
                    }
                }
                return;
            }
        }

        // Handle Window Manager palette events
        #[cfg(all(feature = "window_manager", target_os = "macos"))]
        {
            let mut should_close_palette = false;
            let mut action_to_execute: Option<(&'static str, i32)> = None;

            if let Some(ref mut wm_state) = self.window_manager_state {
                if wm_state.window.id() == window_id {
                    // Let egui handle the event first
                    let _response = wm_state.egui_state.on_window_event(&wm_state.window, &event);

                    match &event {
                        WindowEvent::CloseRequested => {
                            should_close_palette = true;
                        }
                        WindowEvent::RedrawRequested => {
                            let size = wm_state.window.inner_size();
                            if size.width == 0 || size.height == 0 {
                                return;
                            }

                            let raw_input = wm_state.egui_state.take_egui_input(&wm_state.window);

                            let mut palette_result = PaletteResult::Continue;
                            let full_output = wm_state.egui_ctx.run(raw_input, |ctx| {
                                palette_result = wm_state.palette.render(ctx);
                            });

                            wm_state.egui_state.handle_platform_output(
                                &wm_state.window,
                                full_output.platform_output,
                            );

                            let primitives = wm_state
                                .egui_ctx
                                .tessellate(full_output.shapes, full_output.pixels_per_point);

                            let screen_descriptor = egui_wgpu::ScreenDescriptor {
                                size_in_pixels: [size.width, size.height],
                                pixels_per_point: full_output.pixels_per_point,
                            };

                            wm_state.gpu_state.render(
                                primitives,
                                full_output.textures_delta,
                                screen_descriptor,
                            );

                            // Handle palette result
                            match palette_result {
                                PaletteResult::Execute { action_id, target_pid } => {
                                    action_to_execute = Some((action_id, target_pid));
                                    should_close_palette = true;
                                }
                                PaletteResult::Cancel => {
                                    should_close_palette = true;
                                }
                                PaletteResult::Continue => {}
                            }
                        }
                        WindowEvent::Focused(focused) => {
                            if *focused {
                                // Track that the window has been focused
                                wm_state.was_focused = true;
                                log::debug!("Window Manager palette focused");
                            } else if wm_state.was_focused {
                                // Only close on focus loss if we were previously focused
                                log::debug!("Window Manager palette lost focus, closing");
                                should_close_palette = true;
                            } else {
                                log::debug!(
                                    "Window Manager palette got initial focus loss, ignoring"
                                );
                            }
                        }
                        _ => {}
                    }

                    if !should_close_palette {
                        return;
                    }
                }
            }

            if should_close_palette {
                self.window_manager_state = None;
                log::info!("Window Manager palette closed");

                // Execute the action if one was selected
                if let Some((action_id, target_pid)) = action_to_execute {
                    log::info!("Executing action '{}' on PID {}", action_id, target_pid);
                    if let Err(e) =
                        crate::window_manager::execute_window_action(action_id, target_pid)
                    {
                        log::error!("Failed to execute window action: {}", e);
                    }
                }
                return;
            }
        }

        let Some(state) = self.windows.get_mut(&window_id) else { return };

        // Let egui handle the event first
        let _response = state.egui_state.on_window_event(&state.window, &event);

        match event {
            WindowEvent::CloseRequested => {
                state.floating_window.event_handler.dispatch(&FloatingWindowEvent::Close);
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
                let rect =
                    Rect::from_min_size(Pos2::ZERO, Vec2::new(logical_width, logical_height));

                let mut widget_events = Vec::new();
                let full_output = state.egui_ctx.run(raw_input, |ctx| {
                    // Check if this is a controller window
                    if let Some(ref mut controller) = state.controller_state {
                        controller.render(ctx);
                    } else {
                        widget_events = state.floating_window.render(ctx, rect);
                    }
                });

                // Dispatch widget events to registered callback (if any)
                if !widget_events.is_empty() {
                    let window_name =
                        state.floating_window.config.title.clone().unwrap_or_default();

                    if let Some(sender) = self.event_senders.get(&window_name) {
                        for event in widget_events {
                            log::debug!("Dispatching widget event: {:?}", event);
                            if sender.send((window_name.clone(), event)).is_err() {
                                log::warn!("Event receiver dropped for window: {}", window_name);
                            }
                        }
                    } else {
                        // Just log if no callback registered
                        for event in &widget_events {
                            log::debug!("Widget event (no callback): {:?}", event);
                        }
                    }
                }

                state.egui_state.handle_platform_output(&state.window, full_output.platform_output);

                // Tessellate
                let primitives =
                    state.egui_ctx.tessellate(full_output.shapes, full_output.pixels_per_point);

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
                let inside_shape =
                    state.floating_window.shape_mask.contains(logical_x as f32, logical_y as f32);

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

                        if let Some((new_x, new_y)) =
                            state.floating_window.on_drag_move(screen_x, screen_y, scale_factor)
                        {
                            state.window.set_outer_position(PhysicalPosition::new(
                                new_x as i32,
                                new_y as i32,
                            ));
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
                state.floating_window.event_handler.dispatch(&FloatingWindowEvent::Resize {
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

        // Create screenshot window if requested
        if let Some(enabled_actions) = self.pending_screenshot_start.take() {
            self.create_screenshot_window(event_loop, enabled_actions);
        }

        // Create Click Helper window if requested
        #[cfg(all(feature = "click_helper", target_os = "macos"))]
        if self.pending_click_helper_start {
            log::info!("Creating Click Helper window");
            self.pending_click_helper_start = false;
            self.create_click_helper_window(event_loop);
        }

        // Create Window Manager palette if requested
        #[cfg(all(feature = "window_manager", target_os = "macos"))]
        if self.pending_window_manager_start {
            log::info!("Creating Window Manager palette");
            self.pending_window_manager_start = false;
            self.create_window_manager_window(event_loop);
        }

        // Check if screenshot mode should exit
        if let Some(ref state) = self.screenshot_state {
            if state.mode.should_exit() {
                self.screenshot_state = None;
                log::info!("Screenshot mode exited");
            }
        }

        // Check if Click Helper mode should exit
        #[cfg(all(feature = "click_helper", target_os = "macos"))]
        if let Some(ref state) = self.click_helper_state {
            if !state.mode.is_active() {
                self.click_helper_state = None;
                log::info!("Click Helper mode exited");
            }
        }

        // Request redraw for all windows
        for state in self.windows.values() {
            state.window.request_redraw();
        }

        // Request redraw for screenshot window
        if let Some(ref state) = self.screenshot_state {
            state.window.request_redraw();
        }

        // Request redraw for Click Helper window
        #[cfg(all(feature = "click_helper", target_os = "macos"))]
        if let Some(ref state) = self.click_helper_state {
            state.window.request_redraw();
        }

        // Request redraw for Window Manager palette
        #[cfg(all(feature = "window_manager", target_os = "macos"))]
        if let Some(ref state) = self.window_manager_state {
            state.window.request_redraw();
        }
    }
}
