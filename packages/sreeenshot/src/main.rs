use glam::Vec2;
use winit::{
    application::ApplicationHandler,
    event::{ElementState, KeyEvent, MouseButton, WindowEvent},
    keyboard::{Key, NamedKey},
};

mod window;
mod renderer;
mod selection;
mod capture;
mod ui;
mod plugins;

use window::create_fullscreen_window;
use renderer::EguiRenderer;
use selection::Selection;
use ui::Toolbar;
use plugins::{PluginRegistry, PluginContext, PluginResult};
use plugins::{SavePlugin, CopyPlugin, CancelPlugin, AnnotatePlugin, TextPlugin};

struct App {
    renderer: Option<EguiRenderer>,
    selection: Selection,
    monitor: Option<xcap::Monitor>,
    screenshot: Option<image::ImageBuffer<image::Rgba<u8>, Vec<u8>>>,
    mouse_pos: Vec2,
    mouse_pressed: bool, // Track if mouse button is currently pressed
    should_exit: bool,
    selection_completed: bool,
    toolbar: Option<Toolbar>,
    plugin_registry: PluginRegistry,
    // 绘图相关
    drawing_points: Vec<Vec2>, // 绘制的点列表
    is_drawing: bool, // 是否正在绘图
    // 文本相关
    text_items: Vec<(f32, f32, String)>, // 文本列表：(x, y, text)
    text_mode_active: bool, // 文本工具是否激活
    text_input_active: bool, // 是否正在输入文本
    text_input_pos: Option<(f32, f32)>, // 文本输入位置
    text_input_buffer: String, // 文本输入缓冲区
}

impl App {
    fn render_frame(&mut self, event_loop: &winit::event_loop::ActiveEventLoop) {
        if let Some(renderer) = &mut self.renderer {
            // Get scale factor before mutable borrow
            let scale_factor = renderer.window().scale_factor();
            
            // Show selection if active (during dragging) or completed
            let rect = if self.selection.is_active() || self.selection_completed {
                self.selection.rect()
            } else {
                None
            };
            // Only show toolbar and info when selection is completed
            let toolbar = if self.selection_completed {
                self.toolbar.as_ref()
            } else {
                None
            };
            // Pass mouse position and button state to renderer
            let mouse_pos = Some((self.mouse_pos.x, self.mouse_pos.y));
            match renderer.render(
                rect, 
                toolbar, 
                mouse_pos, 
                self.mouse_pressed, 
                &self.drawing_points,
                &self.text_items,
                self.text_input_active,
                self.text_input_pos,
                &mut self.text_input_buffer,
            ) {
                Ok((Some(button_id), _, _)) => {
                    // Toolbar button was clicked, execute plugin
                    // Convert selection coordinates from logical points to physical pixels
                    let context = PluginContext {
                        selection_coords: self.selection.coords_with_scale(scale_factor),
                        screenshot: self.screenshot.clone(),
                        monitor: self.monitor.clone(),
                    };
                    
                    let result = self.plugin_registry.execute_plugin(&button_id, &context);
                    
                    match result {
                        PluginResult::Exit => {
                            self.should_exit = true;
                            event_loop.exit();
                        }
                        PluginResult::Continue => {
                            // Handle cancel plugin
                            if button_id == "cancel" {
                                self.selection.cancel();
                                self.selection_completed = false;
                                self.toolbar = None;
                                self.text_mode_active = false;
                                self.text_input_active = false;
                                renderer.window().request_redraw();
                            } else if button_id == "text" {
                                // 切换文本模式
                                self.text_mode_active = !self.text_mode_active;
                                self.text_input_active = false;
                                renderer.window().request_redraw();
                            }
                        }
                        PluginResult::Success => {
                            // Plugin executed successfully
                            renderer.window().request_redraw();
                        }
                        PluginResult::Failure(msg) => {
                            eprintln!("Plugin error: {}", msg);
                        }
                    }
                }
                Ok((None, text_confirmed, text_cancelled)) => {
                    // 处理文本输入确认或取消
                    if text_confirmed && !self.text_input_buffer.is_empty() {
                        // 确认文本输入，添加到文本列表
                        if let Some((x, y)) = self.text_input_pos {
                            if let Some(rect) = self.selection.rect() {
                                let (sel_x, sel_y, _, _) = rect;
                                // 转换为相对于选择区域的坐标
                                let rel_x = x - sel_x;
                                let rel_y = y - sel_y;
                                self.text_items.push((rel_x, rel_y, self.text_input_buffer.clone()));
                            }
                        }
                        self.text_input_active = false;
                        self.text_input_pos = None;
                        self.text_input_buffer.clear();
                        if let Some(renderer) = &self.renderer {
                            renderer.window().request_redraw();
                        }
                    } else if text_cancelled {
                        // 取消文本输入
                        self.text_input_active = false;
                        self.text_input_pos = None;
                        self.text_input_buffer.clear();
                        if let Some(renderer) = &self.renderer {
                            renderer.window().request_redraw();
                        }
                    }
                }
                Err(e) => {
                    eprintln!("Render error: {}", e);
                }
            }
        }
    }
}

impl ApplicationHandler for App {
    fn resumed(&mut self, event_loop: &winit::event_loop::ActiveEventLoop) {
        // Capture screenshot first
        let monitors = match xcap::Monitor::all() {
            Ok(monitors) => monitors,
            Err(e) => {
                eprintln!("Failed to get monitors: {}", e);
                event_loop.exit();
                return;
            }
        };

        let monitor = match monitors
            .into_iter()
            .find(|m| m.is_primary().unwrap_or(false))
        {
            Some(m) => m,
            None => {
                eprintln!("Could not find primary monitor");
                event_loop.exit();
                return;
            }
        };

        let screenshot = match monitor.capture_image() {
            Ok(img) => img,
            Err(e) => {
                eprintln!("Failed to capture screen: {}", e);
                event_loop.exit();
                return;
            }
        };

        // Get primary monitor from winit to get correct logical size (DPI-aware)
        let primary_monitor = event_loop
            .primary_monitor()
            .or_else(|| event_loop.available_monitors().next());
        
        let size = if let Some(winit_monitor) = primary_monitor {
            // Use winit's monitor size (logical pixels, DPI-aware)
            winit_monitor.size()
        } else {
            // Fallback: convert physical pixels to logical pixels
            // On macOS Retina displays, DPI scale is typically 2.0
            let width = monitor.width().unwrap_or(1920);
            let height = monitor.height().unwrap_or(1080);
            // Assume 2x scale for Retina, but this is a fallback
            winit::dpi::PhysicalSize::new(width / 2, height / 2)
        };

        let window = match create_fullscreen_window(event_loop, size) {
            Ok(w) => w,
            Err(e) => {
                eprintln!("Failed to create window: {}", e);
                event_loop.exit();
                return;
            }
        };

        // Create Arc<Window> for renderer
        let window_arc = std::sync::Arc::new(window);
        
        // 创建 egui 渲染器
        let renderer = match EguiRenderer::new(window_arc, screenshot.clone()) {
            Ok(r) => r,
            Err(e) => {
                eprintln!("Failed to create egui renderer: {}", e);
                event_loop.exit();
                return;
            }
        };

        renderer.window().set_visible(true);
        renderer.window().request_redraw();

        self.renderer = Some(renderer);
        self.monitor = Some(monitor);
        self.screenshot = Some(screenshot);
    }

    fn window_event(
        &mut self,
        event_loop: &winit::event_loop::ActiveEventLoop,
        _window_id: winit::window::WindowId,
        event: WindowEvent,
    ) {
        if self.should_exit {
            return;
        }

        match event {
            WindowEvent::RedrawRequested => {
                self.render_frame(event_loop);
            }
            WindowEvent::Resized(new_size) => {
                // Handle window resize
                if let Some(renderer) = &mut self.renderer {
                    renderer.resize(new_size.width, new_size.height);
                }
            }
            WindowEvent::CursorMoved { position, .. } => {
                // Convert physical pixel position to logical point position for egui
                // winit's position is in physical pixels, but egui uses logical points
                let scale_factor = if let Some(renderer) = &self.renderer {
                    renderer.window().scale_factor()
                } else {
                    1.0
                };
                let logical_pos: winit::dpi::LogicalPosition<f64> = position.to_logical(scale_factor);
                self.mouse_pos = Vec2::new(logical_pos.x as f32, logical_pos.y as f32);
                
                // Only update selection if not completed (allow dragging during selection)
                if self.selection.is_active() && !self.selection_completed {
                    self.selection.update(self.mouse_pos);
                    if let Some(renderer) = &self.renderer {
                        renderer.window().request_redraw();
                    }
                }
                
                // 处理绘图：如果选择完成且鼠标按下，记录绘图点
                if self.selection_completed && self.mouse_pressed {
                    if let Some(rect) = self.selection.rect() {
                        let (sel_x, sel_y, sel_w, sel_h) = rect;
                        // 检查鼠标是否在选择区域内
                        if self.mouse_pos.x >= sel_x 
                            && self.mouse_pos.x <= sel_x + sel_w
                            && self.mouse_pos.y >= sel_y 
                            && self.mouse_pos.y <= sel_y + sel_h {
                            if !self.is_drawing {
                                self.is_drawing = true;
                                self.drawing_points.clear();
                            }
                            // 将坐标转换为相对于选择区域的坐标
                            let relative_pos = Vec2::new(
                                self.mouse_pos.x - sel_x,
                                self.mouse_pos.y - sel_y,
                            );
                            self.drawing_points.push(relative_pos);
                            if let Some(renderer) = &self.renderer {
                                renderer.window().request_redraw();
                            }
                        }
                    }
                }
            }
            WindowEvent::MouseInput { state, button, .. } => {
                // Mouse input is handled by egui in the render method
                
                match (state, button) {
                    (ElementState::Pressed, MouseButton::Left) => {
                        self.mouse_pressed = true;
                        // Only allow starting new selection if not completed
                        if !self.selection_completed {
                            self.selection.start(self.mouse_pos);
                            if let Some(renderer) = &self.renderer {
                                renderer.window().request_redraw();
                            }
                        } else {
                            // 如果选择完成，检查是否在选择区域内
                            if let Some(rect) = self.selection.rect() {
                                let (sel_x, sel_y, sel_w, sel_h) = rect;
                                if self.mouse_pos.x >= sel_x 
                                    && self.mouse_pos.x <= sel_x + sel_w
                                    && self.mouse_pos.y >= sel_y 
                                    && self.mouse_pos.y <= sel_y + sel_h {
                                    // 如果文本模式激活，开始文本输入
                                    if self.text_mode_active {
                                        self.text_input_active = true;
                                        self.text_input_pos = Some((self.mouse_pos.x, self.mouse_pos.y));
                                        self.text_input_buffer.clear();
                                    } else {
                                        // 否则开始绘图
                                        self.is_drawing = true;
                                        self.drawing_points.clear();
                                        let relative_pos = Vec2::new(
                                            self.mouse_pos.x - sel_x,
                                            self.mouse_pos.y - sel_y,
                                        );
                                        self.drawing_points.push(relative_pos);
                                    }
                                }
                            }
                            // If selection is completed, trigger redraw to handle egui button clicks
                            if let Some(renderer) = &self.renderer {
                                renderer.window().request_redraw();
                            }
                        }
                    }
                    (ElementState::Released, MouseButton::Left) => {
                        self.mouse_pressed = false;
                        self.is_drawing = false;
                        // Button clicks are now handled via egui in render() method
                        // Just finish selection if not completed
                        
                        if self.selection.finish().is_some() {
                            // Selection completed, but don't exit - allow further operations
                            self.selection_completed = true;
                            
                            // Create toolbar when selection is completed
                            if let Some(rect) = self.selection.rect() {
                                // Get screen height for toolbar positioning (in logical points)
                                let screen_height = if let Some(renderer) = &self.renderer {
                                    let physical_size = renderer.window().inner_size();
                                    let scale_factor = renderer.window().scale_factor();
                                    (physical_size.height as f64 / scale_factor) as f32
                                } else {
                                    1920.0 // Fallback
                                };
                                let plugin_info = self.plugin_registry.get_enabled_plugin_info();
                                self.toolbar = Some(Toolbar::new(rect.0, rect.1, rect.2, rect.3, screen_height, &plugin_info));
                            }
                            
                            // Immediately render to show toolbar
                            // Don't wait for next RedrawRequested event
                            self.render_frame(event_loop);
                        } else {
                            // Even if selection didn't finish, trigger redraw for egui button clicks
                            if let Some(renderer) = &self.renderer {
                                renderer.window().request_redraw();
                            }
                        }
                    }
                    (_, MouseButton::Right) => {
                        self.selection.cancel();
                        self.selection_completed = false;
                        self.toolbar = None;
                        self.text_mode_active = false;
                        self.text_input_active = false;
                        self.text_input_pos = None;
                        self.text_input_buffer.clear();
                        if let Some(renderer) = &self.renderer {
                            renderer.window().request_redraw();
                        }
                    }
                    _ => {}
                }
            }
            WindowEvent::Ime(ime) => {
                // 处理输入法事件
                if self.text_input_active {
                    if let Some(renderer) = &mut self.renderer {
                        renderer.push_ime_event(ime);
                        renderer.window().request_redraw();
                    }
                }
            }
            WindowEvent::KeyboardInput { event, .. } => {
                // 如果文本输入激活，将键盘事件传递给渲染器
                if self.text_input_active {
                    if let Some(renderer) = &mut self.renderer {
                        // Escape 键仍然用于退出（只在按下时）
                        if matches!(event.state, ElementState::Pressed) 
                            && matches!(event.logical_key, Key::Named(NamedKey::Escape)) {
                            self.should_exit = true;
                            event_loop.exit();
                        } else {
                            // 其他键传递给 egui 处理
                            renderer.push_keyboard_event(event);
                            renderer.window().request_redraw();
                        }
                    }
                } else {
                    // 文本输入未激活时，处理特殊键
                    if matches!(event.state, ElementState::Pressed) {
                        match event.logical_key {
                            Key::Named(NamedKey::Escape) => {
                                self.should_exit = true;
                                event_loop.exit();
                            }
                            Key::Named(NamedKey::Enter) | Key::Named(NamedKey::Space) => {
                                if self.selection.coords().is_some() {
                                    // Mark selection as completed, but don't exit
                                    self.selection_completed = true;
                                    if let Some(renderer) = &self.renderer {
                                        renderer.window().request_redraw();
                                    }
                                }
                            }
                            _ => {}
                        }
                    }
                }
            }
            WindowEvent::CloseRequested => {
                self.should_exit = true;
                event_loop.exit();
            }
            _ => {}
        }
    }
}

fn main() -> anyhow::Result<()> {
    // Initialize plugin registry and register plugins
    let mut plugin_registry = PluginRegistry::new();
    
    // Register plugins
    plugin_registry.register(Box::new(SavePlugin::new()));
    plugin_registry.register(Box::new(CopyPlugin::new()));
    plugin_registry.register(Box::new(CancelPlugin::new()));
    plugin_registry.register(Box::new(AnnotatePlugin::new()));
    plugin_registry.register(Box::new(TextPlugin::new()));
    
    // Enable plugins via configuration array
    let enabled_plugins = vec!["save", "copy", "cancel", "annotate", "text"];
    for plugin_id in enabled_plugins {
        plugin_registry.enable(plugin_id);
    }
    
    let mut app = App {
        renderer: None,
        selection: Selection::new(),
        monitor: None,
        screenshot: None,
        mouse_pos: Vec2::ZERO,
        mouse_pressed: false,
        should_exit: false,
        selection_completed: false,
        toolbar: None,
    plugin_registry,
    drawing_points: Vec::new(),
    is_drawing: false,
    text_items: Vec::new(),
    text_mode_active: false,
    text_input_active: false,
    text_input_pos: None,
    text_input_buffer: String::new(),
};

    let event_loop = winit::event_loop::EventLoop::new()?;
    event_loop.run_app(&mut app)?;
    Ok(())
}
