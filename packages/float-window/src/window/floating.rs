//! FloatingWindow implementation

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
    hide_animation: Option<AnimationController>,
    shape_mask: ShapeMask,
    visible: bool,
    // Runtime state
    drag_start: Option<(f64, f64)>,
    mouse_pos: Option<(f32, f32)>,
    is_dragging: bool,
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

        let particle_system = config.effect.as_ref().map(|(effect, options)| {
            ParticleSystem::new(
                *effect,
                options.clone(),
                config.size.width as f32,
                config.size.height as f32,
            )
        });

        let shape_mask = ShapeMask::new(
            config.shape.clone(),
            config.size.width as f32,
            config.size.height as f32,
        );

        Ok(Self {
            config,
            event_handler,
            particle_system,
            show_animation: None,
            hide_animation: None,
            shape_mask,
            visible: false,
            drag_start: None,
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

        CentralPanel::default()
            .frame(egui::Frame::NONE)
            .show(ctx, |ui| {
                // Apply shape clipping
                WindowPainter::clip_to_shape(ui, &self.config.shape, rect);

                // Draw background based on shape
                let bg_color = Color32::from_rgba_unmultiplied(30, 30, 30, 200);
                match &self.config.shape {
                    WindowShape::Rectangle => {
                        ui.painter().rect_filled(rect, 0.0, bg_color);
                    }
                    WindowShape::Circle => {
                        let radius = rect.width().min(rect.height()) / 2.0;
                        ui.painter().circle_filled(rect.center(), radius, bg_color);
                    }
                    WindowShape::Custom { .. } => {
                        ui.painter().rect_filled(rect, 0.0, bg_color);
                    }
                }

                // Render content
                if let Some(ref content) = self.config.content {
                    WindowPainter::render_content(ui, content, rect);
                }

                // Render particles
                if let Some(ref system) = self.particle_system {
                    WindowPainter::render_particles(ui, system, rect.min);
                }
            });
    }

    /// Handle mouse press
    pub fn on_mouse_press(&mut self, x: f64, y: f64) {
        if self.config.draggable && self.shape_mask.contains(x as f32, y as f32) {
            self.drag_start = Some((x, y));
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
            self.drag_start = None;
            self.event_handler.dispatch(&FloatingWindowEvent::DragEnd {
                x: x as f32,
                y: y as f32,
            });
        }
    }

    /// Handle mouse move, returns new window position if dragging
    pub fn on_mouse_move(&mut self, x: f64, y: f64) -> Option<(f64, f64)> {
        self.mouse_pos = Some((x as f32, y as f32));

        if self.is_dragging {
            if let Some((start_x, start_y)) = self.drag_start {
                let dx = x - start_x;
                let dy = y - start_y;
                let new_x = self.config.position.x + dx;
                let new_y = self.config.position.y + dy;

                self.event_handler.dispatch(&FloatingWindowEvent::Drag {
                    x: new_x as f32,
                    y: new_y as f32,
                });

                return Some((new_x, new_y));
            }
        }

        self.event_handler.dispatch(&FloatingWindowEvent::MouseMove {
            x: x as f32,
            y: y as f32,
        });

        None
    }

    /// Update window position after drag
    pub fn update_position(&mut self, x: f64, y: f64) {
        self.config.position = Position::new(x, y);
        self.drag_start = self.mouse_pos.map(|(mx, my)| (mx as f64, my as f64));
    }
}

/// Application handler for the floating window
struct FloatingWindowApp {
    floating_window: FloatingWindow,
    window: Option<Arc<Window>>,
    egui_ctx: Context,
    egui_state: Option<egui_winit::State>,
}

impl FloatingWindowApp {
    fn new(floating_window: FloatingWindow) -> Self {
        Self {
            floating_window,
            window: None,
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

        let mut attrs = WindowAttributes::default()
            .with_title(config.title.as_deref().unwrap_or("Float Window"))
            .with_inner_size(LogicalSize::new(config.size.width, config.size.height))
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

        // Initialize egui state
        let egui_state = egui_winit::State::new(
            self.egui_ctx.clone(),
            self.egui_ctx.viewport_id(),
            &window,
            None,
            None,
            None,
        );

        self.egui_state = Some(egui_state);
        self.window = Some(window);
        self.floating_window.set_visible(true);
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
                let rect = Rect::from_min_size(
                    Pos2::ZERO,
                    Vec2::new(size.width as f32, size.height as f32),
                );

                let raw_input = egui_state.take_egui_input(&window);
                let output = self.egui_ctx.run(raw_input, |ctx| {
                    self.floating_window.render(ctx, rect);
                });

                egui_state.handle_platform_output(&window, output.platform_output);
                window.request_redraw();
            }
            WindowEvent::CursorMoved { position, .. } => {
                if let Some((new_x, new_y)) =
                    self.floating_window
                        .on_mouse_move(position.x, position.y)
                {
                    window.set_outer_position(PhysicalPosition::new(new_x, new_y));
                    self.floating_window.update_position(new_x, new_y);
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
                self.floating_window
                    .event_handler
                    .dispatch(&FloatingWindowEvent::Resize {
                        width: size.width,
                        height: size.height,
                    });
                if let Some(ref mut system) = self.floating_window.particle_system {
                    system.set_size(size.width as f32, size.height as f32);
                }
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
