//! Controller window UI for managing flow windows

use super::commands::{CommandSender, WindowCommand, WindowRegistry};
use super::config::{Position, Size, WindowConfig};
#[cfg(feature = "stt")]
use crate::stt::{
    DownloadProgress, DownloadStatus, HotkeyEvent as SttHotkeyEvent,
    HotkeyManager as SttHotkeyManager, HotkeyMode, ModelInfo, ModelManager, OutputMode, SttConfig,
};

#[cfg(all(feature = "click_helper", target_os = "macos"))]
use crate::click_helper::{
    ClickHelperConfig, ClickHelperHotkeyManager, ClickHelperMode, Modifier as ClickHelperModifier,
    accessibility::{is_input_monitoring_enabled, open_input_monitoring_settings},
};

/// Navigation tabs for the controller UI
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum NavigationTab {
    /// Floating windows management
    #[default]
    FloatingWindows,
    /// Region capture (screenshot)
    RegionCapture,
    /// Menu bar items
    MenuBarItems,
    /// Speech to text
    SpeechToText,
    /// Click Helper (EasyMotion-style)
    ClickHelper,
    /// Clipboard manager
    Clipboard,
    /// Controller settings
    Settings,
}
use crate::clipboard_manager::{CategoryFilter, ClipboardDb, ClipboardEntry, ClipboardMonitor};
use crate::gui::content::{Content, ImageDisplayOptions, ScaleMode};
use crate::gui::effect::{PresetEffect, PresetEffectOptions};
use crate::gui::menu_bar::{MenuBarIcon, MenuBarItem, MenuBarMenu};
use crate::gui::shape::WindowShape;
use crate::screenshot::icons;
use crate::screenshot::registry::create_default_registry;
use egui::{Context, TextureHandle, Ui};
use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::{Arc, Mutex};
use std::thread;
use winit::window::WindowId;

/// Icon size for controller screenshot actions (logical pixels)
const CONTROLLER_ICON_SIZE: f32 = 20.0;

/// Actions that can be performed on clipboard entries
enum ClipboardAction {
    ToggleFavorite(String),
    Copy(String),
    Delete(String),
}

/// Format a timestamp as relative time (e.g., "2m ago", "1h ago")
fn format_time_ago(dt: chrono::DateTime<chrono::Utc>) -> String {
    let now = chrono::Utc::now();
    let duration = now.signed_duration_since(dt);

    if duration.num_seconds() < 60 {
        "just now".to_string()
    } else if duration.num_minutes() < 60 {
        format!("{}m ago", duration.num_minutes())
    } else if duration.num_hours() < 24 {
        format!("{}h ago", duration.num_hours())
    } else if duration.num_days() < 7 {
        format!("{}d ago", duration.num_days())
    } else {
        format!("{}w ago", duration.num_weeks())
    }
}

/// Default background image embedded at compile time
const DEFAULT_BACKGROUND: &[u8] = include_bytes!("../../assets/background.png");

/// Available shapes for selection
const ALL_SHAPES: &[(&str, WindowShape)] =
    &[("Circle", WindowShape::Circle), ("Rectangle", WindowShape::Rectangle)];

/// All available preset effects
const ALL_EFFECTS: &[PresetEffect] = &[
    PresetEffect::RotatingHalo,
    PresetEffect::PulseRipple,
    PresetEffect::FlowingLight,
    PresetEffect::StardustScatter,
    PresetEffect::ElectricSpark,
    PresetEffect::SmokeWisp,
    PresetEffect::RainDrop,
    PresetEffect::LaserBeam,
    PresetEffect::LightningArc,
    PresetEffect::MeteorShower,
    PresetEffect::SonarPulse,
    PresetEffect::MatrixRain,
    PresetEffect::AuroraWave,
    PresetEffect::OrbitRings,
    PresetEffect::HeartbeatPulse,
    PresetEffect::CosmicStrings,
    PresetEffect::SilkRibbon,
];

/// Controller state for managing windows
pub struct ControllerState {
    /// Command sender to communicate with the event loop
    command_sender: CommandSender,
    /// Registry of managed windows
    registry: WindowRegistry,
    /// Current navigation tab
    active_tab: NavigationTab,
    /// Selected effect for new window
    selected_effect: PresetEffect,
    /// Effect options for new window
    effect_options: PresetEffectOptions,
    /// Selected shape for new window
    selected_shape: WindowShape,
    /// Window size for new window
    new_window_size: u32,
    /// Position X for new window
    new_window_x: f32,
    /// Position Y for new window
    new_window_y: f32,
    /// Name for new menu bar item
    new_menu_bar_name: String,
    /// Tooltip for new menu bar item
    new_menu_bar_tooltip: String,
    /// Counter for menu bar items
    menu_bar_counter: u32,
    /// List of created menu bar item IDs
    menu_bar_items: Vec<String>,
    /// Selected tray icon color (RGB)
    tray_icon_color: [u8; 3],
    /// Selected image path for flow window content
    flow_window_image_path: Option<PathBuf>,
    /// Selected image path for tray icon
    tray_icon_image_path: Option<PathBuf>,
    /// Window ID and size pending image update (when user clicks Set Image on existing window)
    pending_image_update_window: Option<(WindowId, (u32, u32))>,
    /// Controller window background content
    controller_background: Option<Content>,
    /// Pending controller background update
    pending_controller_background: bool,
    /// Async loading state for background image
    async_background_load: Arc<Mutex<Option<Content>>>,
    /// Whether we're currently loading a background
    is_loading_background: bool,
    /// Enabled state for each screenshot action
    screenshot_actions_enabled: HashMap<String, bool>,
    /// Whether to show the app in the dock (macOS)
    #[cfg(target_os = "macos")]
    show_dock_icon: bool,
    /// Cached icon textures for screenshot actions
    icon_cache: HashMap<String, TextureHandle>,
    // ==================== Clipboard Manager State ====================
    /// Clipboard database
    clipboard_db: Option<Arc<Mutex<ClipboardDb>>>,
    /// Clipboard monitor
    clipboard_monitor: Option<ClipboardMonitor>,
    /// Cached clipboard entries for display
    clipboard_entries: Vec<ClipboardEntry>,
    /// Search query
    clipboard_search: String,
    /// Active category filter
    clipboard_filter: CategoryFilter,
    /// Selected entry ID
    clipboard_selected: Option<String>,
    /// Whether to refresh entries on next render
    clipboard_needs_refresh: bool,
    /// Entry count for current filter
    clipboard_entry_count: usize,
    /// Image preview texture cache (entry_id -> texture)
    clipboard_image_cache: HashMap<String, TextureHandle>,
    // ==================== Speech to Text State ====================
    #[cfg(feature = "stt")]
    /// STT configuration
    stt_config: SttConfig,
    #[cfg(feature = "stt")]
    /// Model manager for downloading models
    stt_model_manager: Option<ModelManager>,
    #[cfg(feature = "stt")]
    /// Available models list (cached)
    stt_available_models: Vec<ModelInfo>,
    #[cfg(feature = "stt")]
    /// Whether models need refresh
    stt_models_need_refresh: bool,
    #[cfg(feature = "stt")]
    /// Current download progress (model_id -> progress)
    stt_download_progress: Arc<Mutex<Option<DownloadProgress>>>,
    #[cfg(feature = "stt")]
    /// Whether STT is initialized
    stt_initialized: bool,
    #[cfg(feature = "stt")]
    /// Whether currently recording
    stt_is_recording: bool,
    #[cfg(feature = "stt")]
    /// Last transcription result
    stt_last_transcription: Option<String>,
    #[cfg(feature = "stt")]
    /// Status message for STT
    stt_status: String,
    #[cfg(feature = "stt")]
    /// STT hotkey manager
    stt_hotkey_manager: Option<SttHotkeyManager>,
    // ==================== Click Helper State ====================
    #[cfg(all(feature = "click_helper", target_os = "macos"))]
    /// Click Helper configuration
    click_helper_config: ClickHelperConfig,
    #[cfg(all(feature = "click_helper", target_os = "macos"))]
    /// Whether accessibility is trusted
    click_helper_trusted: bool,
    #[cfg(all(feature = "click_helper", target_os = "macos"))]
    /// Click Helper hotkey manager
    click_helper_hotkey_manager: Option<ClickHelperHotkeyManager>,
    #[cfg(all(feature = "click_helper", target_os = "macos"))]
    /// Whether hotkey listening is enabled by user
    click_helper_hotkey_enabled: bool,
}

impl ControllerState {
    /// Create a new controller state
    pub fn new(command_sender: CommandSender, registry: WindowRegistry) -> Self {
        // Load default background from embedded asset
        let controller_background = Self::load_default_background();

        let state_init = Self {
            command_sender,
            registry,
            active_tab: NavigationTab::default(),
            selected_effect: PresetEffect::SilkRibbon,
            effect_options: PresetEffectOptions::default(),
            selected_shape: WindowShape::Circle,
            new_window_size: 50,
            new_window_x: 500.0,
            new_window_y: 300.0,
            new_menu_bar_name: "My App".to_string(),
            new_menu_bar_tooltip: "Click for menu".to_string(),
            menu_bar_counter: 1,
            menu_bar_items: Vec::new(),
            tray_icon_color: [100, 200, 255], // Cyan default
            flow_window_image_path: None,
            tray_icon_image_path: None,
            pending_image_update_window: None,
            controller_background,
            pending_controller_background: false,
            async_background_load: Arc::new(Mutex::new(None)),
            is_loading_background: false,
            screenshot_actions_enabled: Self::default_screenshot_actions(),
            #[cfg(target_os = "macos")]
            show_dock_icon: true,
            icon_cache: HashMap::new(),
            // Clipboard manager state
            clipboard_db: None,
            clipboard_monitor: None,
            clipboard_entries: Vec::new(),
            clipboard_search: String::new(),
            clipboard_filter: CategoryFilter::default(),
            clipboard_selected: None,
            clipboard_needs_refresh: true,
            clipboard_entry_count: 0,
            clipboard_image_cache: HashMap::new(),
            // STT state
            #[cfg(feature = "stt")]
            stt_config: SttConfig::load().unwrap_or_default(),
            #[cfg(feature = "stt")]
            stt_model_manager: None,
            #[cfg(feature = "stt")]
            stt_available_models: Vec::new(),
            #[cfg(feature = "stt")]
            stt_models_need_refresh: true,
            #[cfg(feature = "stt")]
            stt_download_progress: Arc::new(Mutex::new(None)),
            #[cfg(feature = "stt")]
            stt_initialized: false,
            #[cfg(feature = "stt")]
            stt_is_recording: false,
            #[cfg(feature = "stt")]
            stt_last_transcription: None,
            #[cfg(feature = "stt")]
            stt_status: "Not initialized".to_string(),
            #[cfg(feature = "stt")]
            stt_hotkey_manager: None,
            // Click Helper state
            #[cfg(all(feature = "click_helper", target_os = "macos"))]
            click_helper_config: ClickHelperConfig::load().unwrap_or_default(),
            #[cfg(all(feature = "click_helper", target_os = "macos"))]
            click_helper_trusted: false,
            #[cfg(all(feature = "click_helper", target_os = "macos"))]
            click_helper_hotkey_manager: None,
            #[cfg(all(feature = "click_helper", target_os = "macos"))]
            click_helper_hotkey_enabled: false,
        };

        #[cfg(any(feature = "stt", all(feature = "click_helper", target_os = "macos")))]
        let mut state = state_init;
        #[cfg(not(any(feature = "stt", all(feature = "click_helper", target_os = "macos"))))]
        let state = state_init;

        // Auto-start hotkey listeners if enabled in config
        #[cfg(all(feature = "click_helper", target_os = "macos"))]
        if state.click_helper_config.hotkey_enabled {
            state.init_click_helper_hotkey();
            state.click_helper_hotkey_enabled = true;
        }

        #[cfg(feature = "stt")]
        if state.stt_config.hotkey_enabled {
            state.init_stt_hotkey();
        }

        state
    }

    /// Initialize STT hotkey manager
    #[cfg(feature = "stt")]
    fn init_stt_hotkey(&mut self) {
        let config = self.stt_config.hotkey.clone();
        let is_recording = std::sync::Arc::new(std::sync::atomic::AtomicBool::new(false));
        let is_recording_for_callback = is_recording.clone();

        let mut manager = SttHotkeyManager::new();
        manager.set_config(config);
        manager.set_callback(move |event| match event {
            SttHotkeyEvent::RecordStart => {
                is_recording_for_callback.store(true, std::sync::atomic::Ordering::Relaxed);
                log::info!("STT recording started via hotkey");
            }
            SttHotkeyEvent::RecordStop => {
                is_recording_for_callback.store(false, std::sync::atomic::Ordering::Relaxed);
                log::info!("STT recording stopped via hotkey");
            }
        });

        if let Err(e) = manager.start() {
            log::error!("Failed to start STT hotkey manager: {}", e);
        } else {
            log::info!("STT hotkey manager started");
            self.stt_hotkey_manager = Some(manager);
        }
    }

    /// Initialize Click Helper hotkey manager with the command sender
    #[cfg(all(feature = "click_helper", target_os = "macos"))]
    pub fn init_click_helper_hotkey(&mut self) {
        let config = self.click_helper_config.hotkey.clone();
        let sender = self.command_sender.clone();

        let mut manager = ClickHelperHotkeyManager::new();
        manager.set_config(config);
        manager.set_callback(move || {
            log::info!("Click Helper hotkey callback triggered, sending command");
            if let Err(e) = sender.send(super::commands::WindowCommand::StartClickHelperMode) {
                log::error!("Failed to send Click Helper command: {}", e);
            }
        });

        if let Err(e) = manager.start() {
            log::error!("Failed to start Click Helper hotkey manager: {}", e);
        } else {
            log::info!("Click Helper hotkey manager started");
            self.click_helper_hotkey_manager = Some(manager);
        }
    }

    /// Load default background from embedded PNG asset
    fn load_default_background() -> Option<Content> {
        match image::load_from_memory(DEFAULT_BACKGROUND) {
            Ok(img) => {
                let rgba = img.to_rgba8();
                let (width, height) = rgba.dimensions();
                Some(Content::Image {
                    data: rgba.into_raw(),
                    width,
                    height,
                    options: ImageDisplayOptions::new().with_scale_mode(ScaleMode::Stretch),
                })
            }
            Err(e) => {
                log::warn!("Failed to load default background: {}", e);
                None
            }
        }
    }

    /// Create default screenshot action enabled states
    fn default_screenshot_actions() -> HashMap<String, bool> {
        let mut actions = HashMap::new();
        // Drawing tools - most enabled by default (in Snipaste order)
        actions.insert("rectangle".to_string(), true);
        actions.insert("ellipse".to_string(), true);
        actions.insert("polyline".to_string(), true);
        actions.insert("arrow".to_string(), true);
        actions.insert("annotate".to_string(), true);
        actions.insert("highlighter".to_string(), true);
        actions.insert("mosaic".to_string(), true);
        actions.insert("text".to_string(), true);
        actions.insert("sequence".to_string(), true);
        actions.insert("eraser".to_string(), true);
        // Privacy tools
        actions.insert("blur".to_string(), true);
        // Terminal actions
        actions.insert("cancel".to_string(), true);
        actions.insert("save".to_string(), true);
        actions.insert("copy".to_string(), true);
        actions
    }

    /// Get the window registry
    pub fn registry(&self) -> &WindowRegistry {
        &self.registry
    }

    /// Render the controller UI
    pub fn render(&mut self, ctx: &Context) {
        // Check for async-loaded background
        if let Ok(mut pending) = self.async_background_load.try_lock()
            && let Some(content) = pending.take()
        {
            self.controller_background = Some(content);
            self.is_loading_background = false;
            log::info!("Background image loaded asynchronously");
        }

        // Glassmorphism color scheme
        let has_bg = self.controller_background.is_some();

        // Title bar frame
        let title_frame = if has_bg {
            egui::Frame::NONE
                .fill(egui::Color32::from_rgba_unmultiplied(15, 20, 35, 230))
                .inner_margin(egui::Margin::symmetric(12, 8))
        } else {
            egui::Frame::NONE
                .fill(egui::Color32::from_rgb(20, 22, 32))
                .inner_margin(egui::Margin::symmetric(12, 8))
        };

        // Navigation panel frame: more opaque for better readability
        let nav_frame = if has_bg {
            egui::Frame::NONE
                .fill(egui::Color32::from_rgba_unmultiplied(15, 20, 35, 220))
                .inner_margin(egui::Margin::same(8))
        } else {
            egui::Frame::NONE
                .fill(egui::Color32::from_rgb(25, 28, 40))
                .inner_margin(egui::Margin::same(8))
        };

        // Content panel frame: more transparent for glassmorphism effect
        let content_frame = if has_bg {
            egui::Frame::NONE
                .fill(egui::Color32::from_rgba_unmultiplied(20, 25, 45, 160))
                .inner_margin(egui::Margin::same(12))
        } else {
            egui::Frame::NONE
                .fill(egui::Color32::from_rgb(30, 32, 48))
                .inner_margin(egui::Margin::same(12))
        };

        // Draw background image if set (behind everything using layer painter)
        if let Some(Content::Image { data, width, height, .. }) = &self.controller_background {
            #[allow(deprecated)]
            let screen_rect = ctx.input(|i| i.screen_rect());
            let texture = ctx.load_texture(
                "controller_bg",
                egui::ColorImage::from_rgba_unmultiplied([*width as usize, *height as usize], data),
                egui::TextureOptions::LINEAR,
            );
            // Use layer painter at Background level
            let painter = ctx.layer_painter(egui::LayerId::background());
            painter.image(
                texture.id(),
                screen_rect,
                egui::Rect::from_min_max(egui::Pos2::ZERO, egui::Pos2::new(1.0, 1.0)),
                egui::Color32::WHITE,
            );
        }

        // TOP PANEL: Title bar with "aumate" on left and Exit button on right
        egui::TopBottomPanel::top("title_bar").exact_height(40.0).frame(title_frame).show(
            ctx,
            |ui| {
                self.render_title_bar(ui);
            },
        );

        // LEFT PANEL: Navigation sidebar (narrow)
        egui::SidePanel::left("nav_panel")
            .resizable(false)
            .exact_width(160.0)
            .frame(nav_frame)
            .show(ctx, |ui| {
                self.render_navigation(ui);
            });

        // CENTRAL PANEL: Content based on active tab
        egui::CentralPanel::default().frame(content_frame).show(ctx, |ui| {
            self.render_tab_content(ui);
        });
    }

    /// Render the title bar with app name and exit button
    fn render_title_bar(&mut self, ui: &mut Ui) {
        ui.horizontal(|ui| {
            ui.heading("aumate");
            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                // Allocate space for close button
                let (rect, response) =
                    ui.allocate_exact_size(egui::vec2(28.0, 28.0), egui::Sense::click());

                // Draw background on hover
                if response.hovered() {
                    ui.painter().rect_filled(
                        rect,
                        egui::CornerRadius::same(4),
                        egui::Color32::from_rgba_unmultiplied(200, 60, 60, 180),
                    );
                }

                // Draw X using lines (more reliable than Unicode)
                let padding = 8.0;
                let stroke = egui::Stroke::new(2.0, egui::Color32::WHITE);
                ui.painter().line_segment(
                    [
                        egui::pos2(rect.min.x + padding, rect.min.y + padding),
                        egui::pos2(rect.max.x - padding, rect.max.y - padding),
                    ],
                    stroke,
                );
                ui.painter().line_segment(
                    [
                        egui::pos2(rect.max.x - padding, rect.min.y + padding),
                        egui::pos2(rect.min.x + padding, rect.max.y - padding),
                    ],
                    stroke,
                );

                if response.clicked() {
                    let _ = self.command_sender.send(WindowCommand::ExitApplication);
                }
            });
        });
    }

    /// Render the navigation sidebar
    fn render_navigation(&mut self, ui: &mut Ui) {
        ui.add_space(8.0);

        let nav_items = [
            (NavigationTab::FloatingWindows, "Floating Windows"),
            (NavigationTab::RegionCapture, "Region Capture"),
            (NavigationTab::MenuBarItems, "Menu Bar Items"),
            (NavigationTab::SpeechToText, "Speech to Text"),
            (NavigationTab::ClickHelper, "Click Helper"),
            (NavigationTab::Clipboard, "Clipboard"),
            (NavigationTab::Settings, "Settings"),
        ];

        for (tab, label) in nav_items {
            let selected = self.active_tab == tab;
            let response = ui.add_sized(
                [ui.available_width(), 32.0],
                egui::Button::new(label).selected(selected),
            );
            if response.clicked() {
                self.active_tab = tab;
            }
        }
    }

    /// Render content based on the active tab
    fn render_tab_content(&mut self, ui: &mut Ui) {
        egui::ScrollArea::vertical().auto_shrink([false; 2]).show(ui, |ui| match self.active_tab {
            NavigationTab::FloatingWindows => {
                self.render_create_section(ui);
                ui.add_space(16.0);
                ui.separator();
                ui.add_space(8.0);
                self.render_manage_section(ui);
            }
            NavigationTab::RegionCapture => {
                self.render_screenshot_section(ui);
            }
            NavigationTab::MenuBarItems => {
                self.render_create_menu_bar_section(ui);
                ui.add_space(16.0);
                ui.separator();
                ui.add_space(8.0);
                self.render_active_menu_bar_section(ui);
            }
            #[cfg(feature = "stt")]
            NavigationTab::SpeechToText => {
                self.render_stt_section(ui);
            }
            #[cfg(not(feature = "stt"))]
            NavigationTab::SpeechToText => {
                ui.label(
                    "Speech to Text feature is not enabled. Please compile with --features stt",
                );
            }
            #[cfg(all(feature = "click_helper", target_os = "macos"))]
            NavigationTab::ClickHelper => {
                self.render_click_helper_section(ui);
            }
            #[cfg(not(all(feature = "click_helper", target_os = "macos")))]
            NavigationTab::ClickHelper => {
                ui.label("Click Helper is only supported on macOS.");
                ui.label("Please compile with --features click_helper on macOS.");
            }
            NavigationTab::Clipboard => {
                self.render_clipboard_section(ui);
            }
            NavigationTab::Settings => {
                self.render_controller_settings(ui);
            }
        });
    }

    /// Render the controller settings section
    fn render_controller_settings(&mut self, ui: &mut Ui) {
        ui.heading("Controller Settings");
        ui.add_space(8.0);

        ui.group(|ui| {
            ui.label("Background Image:");
            ui.horizontal(|ui| {
                if self.is_loading_background {
                    ui.label("Loading...");
                    ui.spinner();
                } else if self.controller_background.is_some() {
                    ui.label("Set");
                    if ui.button("Clear").clicked() {
                        self.controller_background = None;
                    }
                } else {
                    ui.label("None");
                }
                if !self.is_loading_background && ui.button("Browse...").clicked() {
                    self.pending_controller_background = true;
                }
            });
        });

        // Handle pending controller background update (file picker)
        if self.pending_controller_background {
            self.pending_controller_background = false;
            if let Some(path) = rfd::FileDialog::new()
                .add_filter("Images", &["png", "svg", "jpg", "jpeg"])
                .pick_file()
            {
                // Load asynchronously to avoid blocking UI
                self.is_loading_background = true;
                let async_load = self.async_background_load.clone();
                let path_clone = path.clone();

                thread::spawn(move || {
                    log::info!("Loading controller background async: {:?}", path_clone);
                    match Content::from_path_sized(&path_clone, 1200, 1200) {
                        Ok(content) => {
                            if let Ok(mut pending) = async_load.lock() {
                                *pending = Some(content);
                            }
                        }
                        Err(e) => {
                            log::error!("Failed to load controller background: {}", e);
                        }
                    }
                });
            }
        }

        // macOS: Dock icon visibility setting
        #[cfg(target_os = "macos")]
        {
            ui.add_space(8.0);
            ui.group(|ui| {
                ui.label("Application:");
                if ui.checkbox(&mut self.show_dock_icon, "Show Dock Icon").changed() {
                    set_dock_icon_visibility(self.show_dock_icon);
                }
            });
        }
    }

    /// Get the controller background content
    pub fn get_background(&self) -> Option<&Content> {
        self.controller_background.as_ref()
    }

    // ==================== Clipboard Manager Methods ====================

    /// Initialize the clipboard database if not already done
    fn ensure_clipboard_db(&mut self) {
        if self.clipboard_db.is_none() {
            match ClipboardDb::open() {
                Ok(db) => {
                    self.clipboard_db = Some(Arc::new(Mutex::new(db)));
                    self.clipboard_needs_refresh = true;
                    log::info!("Clipboard database initialized");
                }
                Err(e) => {
                    log::error!("Failed to open clipboard database: {}", e);
                }
            }
        }
    }

    /// Refresh clipboard entries from database
    fn refresh_clipboard_entries(&mut self) {
        if !self.clipboard_needs_refresh {
            return;
        }

        if let Some(db) = &self.clipboard_db {
            if let Ok(db) = db.lock() {
                let search = if self.clipboard_search.is_empty() {
                    None
                } else {
                    Some(self.clipboard_search.as_str())
                };

                match db.get_entries(self.clipboard_filter, search, 50, 0) {
                    Ok(entries) => {
                        self.clipboard_entries = entries;
                    }
                    Err(e) => {
                        log::error!("Failed to load clipboard entries: {}", e);
                    }
                }

                if let Ok(count) = db.count_entries(self.clipboard_filter) {
                    self.clipboard_entry_count = count;
                }
            }
        }

        self.clipboard_needs_refresh = false;
    }

    /// Render the clipboard manager section
    fn render_clipboard_section(&mut self, ui: &mut Ui) {
        // Ensure database is initialized
        self.ensure_clipboard_db();
        self.refresh_clipboard_entries();

        ui.heading("Clipboard Manager");
        ui.add_space(8.0);

        // Top bar: Search, Start/Stop Monitor, Export
        ui.horizontal(|ui| {
            // Search box
            ui.label("Search:");
            let search_response = ui.add(
                egui::TextEdit::singleline(&mut self.clipboard_search)
                    .desired_width(200.0)
                    .hint_text("Filter entries..."),
            );
            if search_response.changed() {
                self.clipboard_needs_refresh = true;
            }

            ui.add_space(8.0);

            // Monitor toggle
            let is_running = self.clipboard_monitor.as_ref().is_some_and(|m| m.is_running());

            if is_running {
                if ui.button("Stop Monitor").clicked() {
                    if let Some(monitor) = &mut self.clipboard_monitor {
                        monitor.stop();
                    }
                }
            } else if ui.button("Start Monitor").clicked() {
                self.start_clipboard_monitor();
            }

            // Export button
            if ui.button("Export...").clicked() {
                self.export_clipboard_history();
            }
        });

        ui.add_space(8.0);

        // Main content: sidebar + entry list
        ui.horizontal(|ui| {
            // Left sidebar: Categories
            ui.vertical(|ui| {
                ui.set_min_width(120.0);
                ui.label("Categories");
                ui.add_space(4.0);

                for filter in CategoryFilter::all_options() {
                    let selected = self.clipboard_filter == *filter;
                    if ui.selectable_label(selected, filter.display_name()).clicked() {
                        self.clipboard_filter = *filter;
                        self.clipboard_needs_refresh = true;
                    }
                }

                ui.add_space(16.0);

                // Entry count
                ui.label(format!("{} entries", self.clipboard_entry_count));

                ui.add_space(8.0);

                // Clear all button (with confirmation)
                if ui.button("Clear All").clicked() {
                    self.clear_all_clipboard_entries();
                }
            });

            ui.separator();

            // Right: Entry list
            ui.vertical(|ui| {
                ui.set_min_width(400.0);

                if self.clipboard_entries.is_empty() {
                    ui.label("No clipboard entries yet.");
                    ui.label("Click 'Start Monitor' to begin tracking clipboard.");
                } else {
                    egui::ScrollArea::vertical().max_height(400.0).show(ui, |ui| {
                        let mut action = None;

                        for entry in &self.clipboard_entries {
                            let is_selected =
                                self.clipboard_selected.as_ref().is_some_and(|id| id == &entry.id);

                            ui.group(|ui| {
                                ui.horizontal(|ui| {
                                    // Type indicator
                                    let type_label = match entry.content_type {
                                        crate::clipboard_manager::ContentType::Text => "T",
                                        crate::clipboard_manager::ContentType::Image => "I",
                                        crate::clipboard_manager::ContentType::Files => "F",
                                    };
                                    ui.label(
                                        egui::RichText::new(type_label)
                                            .monospace()
                                            .color(egui::Color32::LIGHT_BLUE),
                                    );

                                    // Sensitive indicator
                                    if entry.is_sensitive {
                                        ui.label(
                                            egui::RichText::new("!").color(egui::Color32::YELLOW),
                                        );
                                    }

                                    // Pinned indicator
                                    if entry.is_pinned {
                                        ui.label(
                                            egui::RichText::new("📌").color(egui::Color32::WHITE),
                                        );
                                    }

                                    // Preview text (clickable to select)
                                    let preview = if entry.preview_text.len() > 50 {
                                        format!("{}...", &entry.preview_text[..50])
                                    } else {
                                        entry.preview_text.clone()
                                    };

                                    let response = ui.selectable_label(is_selected, &preview);
                                    if response.clicked() {
                                        self.clipboard_selected = Some(entry.id.clone());
                                    }

                                    // Timestamp
                                    if let Ok(dt) =
                                        chrono::DateTime::parse_from_rfc3339(&entry.created_at)
                                    {
                                        let ago = format_time_ago(dt.into());
                                        ui.with_layout(
                                            egui::Layout::right_to_left(egui::Align::Center),
                                            |ui| {
                                                ui.label(
                                                    egui::RichText::new(ago)
                                                        .small()
                                                        .color(egui::Color32::GRAY),
                                                );
                                            },
                                        );
                                    }
                                });

                                // Image preview for image entries
                                if let crate::clipboard_manager::ClipboardContent::Image {
                                    data,
                                    width,
                                    height,
                                } = &entry.content
                                {
                                    // Check if texture is already cached
                                    let texture_id = format!("clipboard_img_{}", entry.id);
                                    if !self.clipboard_image_cache.contains_key(&entry.id) {
                                        // Decode PNG and create texture
                                        if let Ok(img) = image::load_from_memory(data) {
                                            let rgba = img.to_rgba8();
                                            let (w, h) = rgba.dimensions();
                                            let texture = ui.ctx().load_texture(
                                                &texture_id,
                                                egui::ColorImage::from_rgba_unmultiplied(
                                                    [w as usize, h as usize],
                                                    &rgba.into_raw(),
                                                ),
                                                egui::TextureOptions::LINEAR,
                                            );
                                            self.clipboard_image_cache
                                                .insert(entry.id.clone(), texture);
                                        }
                                    }

                                    // Display the image preview with max width 400px
                                    if let Some(texture) = self.clipboard_image_cache.get(&entry.id)
                                    {
                                        const MAX_PREVIEW_WIDTH: f32 = 400.0;
                                        let aspect_ratio = *height as f32 / *width as f32;
                                        let display_width = (*width as f32).min(MAX_PREVIEW_WIDTH);
                                        let display_height = display_width * aspect_ratio;

                                        ui.add_space(4.0);
                                        ui.add(
                                            egui::Image::new(texture)
                                                .fit_to_exact_size(egui::vec2(
                                                    display_width,
                                                    display_height,
                                                ))
                                                .corner_radius(4.0),
                                        );
                                    }
                                }

                                // Action buttons (when selected)
                                if is_selected {
                                    ui.horizontal(|ui| {
                                        // Favorite toggle
                                        let fav_text = if entry.is_favorite {
                                            "Unfavorite"
                                        } else {
                                            "Favorite"
                                        };
                                        if ui.small_button(fav_text).clicked() {
                                            action = Some(ClipboardAction::ToggleFavorite(
                                                entry.id.clone(),
                                            ));
                                        }

                                        // Copy button
                                        if ui.small_button("Copy").clicked() {
                                            action = Some(ClipboardAction::Copy(entry.id.clone()));
                                        }

                                        // Delete button
                                        if ui.small_button("Delete").clicked() {
                                            action =
                                                Some(ClipboardAction::Delete(entry.id.clone()));
                                        }
                                    });
                                }
                            });
                        }

                        // Handle actions after iteration
                        if let Some(act) = action {
                            self.handle_clipboard_action(act);
                        }
                    });
                }
            });
        });
    }

    /// Start the clipboard monitor
    fn start_clipboard_monitor(&mut self) {
        self.ensure_clipboard_db();

        if let Some(db) = &self.clipboard_db {
            let mut monitor = ClipboardMonitor::new();
            if let Err(e) = monitor.start(Arc::clone(db)) {
                log::error!("Failed to start clipboard monitor: {}", e);
            } else {
                log::info!("Clipboard monitor started");
                self.clipboard_monitor = Some(monitor);
            }
        }
    }

    /// Export clipboard history to file
    fn export_clipboard_history(&self) {
        use crate::clipboard_manager::ExportData;

        if let Some(db) = &self.clipboard_db {
            if let Ok(db) = db.lock() {
                match ExportData::from_db(&db) {
                    Ok(export_data) => {
                        if let Some(path) = rfd::FileDialog::new()
                            .add_filter("JSON", &["json"])
                            .set_file_name("clipboard_history.json")
                            .save_file()
                        {
                            if let Err(e) = export_data.to_file(&path) {
                                log::error!("Failed to export clipboard history: {}", e);
                            } else {
                                log::info!("Exported clipboard history to {:?}", path);
                            }
                        }
                    }
                    Err(e) => {
                        log::error!("Failed to create export data: {}", e);
                    }
                }
            }
        }
    }

    /// Clear all clipboard entries
    fn clear_all_clipboard_entries(&mut self) {
        if let Some(db) = &self.clipboard_db {
            if let Ok(db) = db.lock() {
                if let Err(e) = db.clear_all() {
                    log::error!("Failed to clear clipboard entries: {}", e);
                } else {
                    log::info!("Cleared all clipboard entries");
                    self.clipboard_needs_refresh = true;
                    // Clear image cache
                    self.clipboard_image_cache.clear();
                }
            }
        }
    }

    /// Handle clipboard entry actions
    fn handle_clipboard_action(&mut self, action: ClipboardAction) {
        if let Some(db) = &self.clipboard_db {
            if let Ok(db) = db.lock() {
                match action {
                    ClipboardAction::ToggleFavorite(id) => {
                        if let Err(e) = db.toggle_favorite(&id) {
                            log::error!("Failed to toggle favorite: {}", e);
                        }
                        self.clipboard_needs_refresh = true;
                    }
                    ClipboardAction::Copy(id) => {
                        if let Ok(Some(entry)) = db.get_entry(&id) {
                            self.copy_entry_to_clipboard(&entry);
                        }
                    }
                    ClipboardAction::Delete(id) => {
                        if let Err(e) = db.delete_entry(&id) {
                            log::error!("Failed to delete entry: {}", e);
                        }
                        self.clipboard_selected = None;
                        self.clipboard_needs_refresh = true;
                        // Remove from image cache if present
                        self.clipboard_image_cache.remove(&id);
                    }
                }
            }
        }
    }

    /// Copy a clipboard entry back to the system clipboard
    fn copy_entry_to_clipboard(&self, entry: &ClipboardEntry) {
        use crate::clipboard_manager::ClipboardContent;

        match &entry.content {
            ClipboardContent::Text(text) => {
                if let Err(e) = crate::clipboard::set_text(text) {
                    log::error!("Failed to copy text to clipboard: {}", e);
                }
            }
            ClipboardContent::Image { data, .. } => {
                // Data is already PNG-encoded, pass directly to clipboard
                if let Err(e) = crate::clipboard::set_image(data) {
                    log::error!("Failed to copy image to clipboard: {}", e);
                }
            }
            ClipboardContent::Files(_files) => {
                // File copying is platform-specific, just log for now
                log::warn!("File copying not yet implemented");
            }
        }
    }

    /// Render the region capture (screenshot) section
    fn render_screenshot_section(&mut self, ui: &mut Ui) {
        ui.heading("Region Capture");
        ui.add_space(8.0);

        // Get action info from registry
        let registry = create_default_registry();
        let actions = registry.get_all();

        // Action checkboxes - styled grid with icon and description
        ui.label("Enabled Actions:");
        ui.add_space(4.0);

        // Get scale factor for DPI-aware icon rendering
        let scale_factor = ui.ctx().pixels_per_point();
        // Render icons at 2x minimum for crisp display on Retina
        let render_scale = scale_factor.max(2.0);
        let render_size = (CONTROLLER_ICON_SIZE * render_scale).ceil() as u32;

        // Create styled action items in a grid layout
        egui::Grid::new("screenshot_actions_grid")
            .num_columns(3)
            .spacing([12.0, 6.0])
            .min_col_width(28.0)
            .show(ui, |ui| {
                for action in &actions {
                    if let Some(enabled) = self.screenshot_actions_enabled.get_mut(&action.id) {
                        // Checkbox with custom styling
                        ui.checkbox(enabled, "");

                        // Get or create icon texture at high resolution
                        let icon_id = action.icon_id.as_deref().unwrap_or(&action.id);
                        let cache_key = format!("{}_{}", icon_id, render_size);
                        if !self.icon_cache.contains_key(&cache_key) {
                            if let Some(texture) = icons::create_icon_texture(
                                ui.ctx(),
                                icon_id,
                                render_size,
                                egui::Color32::WHITE,
                            ) {
                                self.icon_cache.insert(cache_key.clone(), texture);
                            }
                        }

                        // Render icon in a styled container
                        let (rect, _response) =
                            ui.allocate_exact_size(egui::vec2(28.0, 28.0), egui::Sense::hover());

                        // Draw subtle background for icon
                        ui.painter().rect_filled(
                            rect,
                            egui::CornerRadius::same(4),
                            egui::Color32::from_rgba_unmultiplied(60, 65, 80, 180),
                        );

                        // Center and draw the icon at logical size (texture is high-res)
                        if let Some(texture) = self.icon_cache.get(&cache_key) {
                            let icon_rect = egui::Rect::from_center_size(
                                rect.center(),
                                egui::vec2(CONTROLLER_ICON_SIZE, CONTROLLER_ICON_SIZE),
                            );
                            ui.painter().image(
                                texture.id(),
                                icon_rect,
                                egui::Rect::from_min_max(
                                    egui::Pos2::ZERO,
                                    egui::Pos2::new(1.0, 1.0),
                                ),
                                egui::Color32::WHITE,
                            );
                        } else {
                            // Fallback text
                            ui.painter().text(
                                rect.center(),
                                egui::Align2::CENTER_CENTER,
                                action.id[..1].to_uppercase(),
                                egui::FontId::proportional(12.0),
                                egui::Color32::WHITE,
                            );
                        }

                        // Label with proper color
                        let label_color = if *enabled {
                            egui::Color32::WHITE
                        } else {
                            egui::Color32::from_gray(140)
                        };
                        ui.label(egui::RichText::new(&action.name).color(label_color));

                        ui.end_row();
                    }
                }
            });

        ui.add_space(12.0);

        // Region Capture button with prominent styling
        if ui
            .add_sized(
                [140.0, 32.0],
                egui::Button::new(egui::RichText::new("Region Capture").size(14.0)),
            )
            .clicked()
        {
            let enabled: Vec<String> = self
                .screenshot_actions_enabled
                .iter()
                .filter(|&(_, v)| *v)
                .map(|(k, _)| k.clone())
                .collect();

            let _ = self
                .command_sender
                .send(WindowCommand::StartScreenshotMode { enabled_actions: enabled });
        }
    }

    /// Render the create new window section
    fn render_create_section(&mut self, ui: &mut Ui) {
        ui.heading("Create New Window");
        ui.add_space(8.0);

        // Effect selection
        ui.horizontal(|ui| {
            ui.label("Effect:");
            egui::ComboBox::from_id_salt("effect_selector")
                .selected_text(format!("{:?}", self.selected_effect))
                .show_ui(ui, |ui| {
                    for effect in ALL_EFFECTS {
                        ui.selectable_value(
                            &mut self.selected_effect,
                            *effect,
                            format!("{:?}", effect),
                        );
                    }
                });
        });

        ui.add_space(4.0);

        // Effect options based on selected effect
        ui.collapsing("Effect Options", |ui| {
            self.render_effect_options(ui);
        });

        ui.add_space(4.0);

        // Shape selection
        ui.horizontal(|ui| {
            ui.label("Shape:");
            egui::ComboBox::from_id_salt("shape_selector")
                .selected_text(format!("{:?}", self.selected_shape))
                .show_ui(ui, |ui| {
                    for (name, shape) in ALL_SHAPES {
                        ui.selectable_value(&mut self.selected_shape, shape.clone(), *name);
                    }
                });
        });

        ui.add_space(4.0);

        // Image Content Section
        ui.horizontal(|ui| {
            ui.label("Image:");
            if let Some(path) = &self.flow_window_image_path {
                let filename = path
                    .file_name()
                    .map(|s| s.to_string_lossy().to_string())
                    .unwrap_or_else(|| "Unknown".to_string());
                ui.label(&filename);
            } else {
                ui.label("None");
            }
            if ui.button("Browse...").clicked() {
                self.open_image_picker_for_flow_window();
            }
            if self.flow_window_image_path.is_some() && ui.button("Clear").clicked() {
                self.flow_window_image_path = None;
            }
        });

        ui.add_space(4.0);

        // Window size
        ui.horizontal(|ui| {
            ui.label("Size:");
            ui.add(egui::Slider::new(&mut self.new_window_size, 30..=200).suffix("px"));
        });

        // Position
        ui.horizontal(|ui| {
            ui.label("Position X:");
            ui.add(egui::DragValue::new(&mut self.new_window_x).range(0.0..=2000.0));
            ui.label("Y:");
            ui.add(egui::DragValue::new(&mut self.new_window_y).range(0.0..=2000.0));
        });

        ui.add_space(8.0);

        // Create button
        if ui.button("Create Window").clicked() {
            self.create_window();
        }
    }

    /// Render effect-specific options
    fn render_effect_options(&mut self, ui: &mut Ui) {
        // Common options
        ui.horizontal(|ui| {
            ui.label("Intensity:");
            ui.add(egui::Slider::new(&mut self.effect_options.intensity, 0.0..=1.0));
        });

        ui.horizontal(|ui| {
            ui.label("Speed:");
            ui.add(egui::Slider::new(&mut self.effect_options.speed, 0.1..=3.0));
        });

        // Effect-specific options
        match self.selected_effect {
            PresetEffect::SilkRibbon => {
                ui.horizontal(|ui| {
                    ui.label("Ribbon Count:");
                    let mut count = self.effect_options.ribbon_count as i32;
                    if ui.add(egui::Slider::new(&mut count, 1..=6)).changed() {
                        self.effect_options.ribbon_count = count as usize;
                    }
                });
                ui.horizontal(|ui| {
                    ui.label("Petal Amplitude:");
                    ui.add(egui::Slider::new(&mut self.effect_options.petal_amplitude, 5.0..=50.0));
                });
            }
            _ => {
                // Generic options for other effects
                ui.horizontal(|ui| {
                    ui.label("Edge Width:");
                    ui.add(egui::Slider::new(&mut self.effect_options.edge_width, 5.0..=30.0));
                });
            }
        }

        // Color presets
        ui.horizontal(|ui| {
            ui.label("Colors:");
            if ui.button("Cyan/Purple").clicked() {
                self.effect_options.particle_colors =
                    vec![[0.4, 0.8, 1.0, 1.0], [0.8, 0.4, 1.0, 1.0]];
            }
            if ui.button("Fire").clicked() {
                self.effect_options.particle_colors =
                    vec![[1.0, 0.3, 0.0, 1.0], [1.0, 0.6, 0.0, 1.0], [1.0, 0.9, 0.2, 1.0]];
            }
            if ui.button("Rainbow").clicked() {
                self.effect_options.particle_colors = vec![
                    [1.0, 0.0, 0.0, 1.0],
                    [1.0, 0.5, 0.0, 1.0],
                    [1.0, 1.0, 0.0, 1.0],
                    [0.0, 1.0, 0.0, 1.0],
                    [0.0, 0.5, 1.0, 1.0],
                    [0.5, 0.0, 1.0, 1.0],
                ];
            }
        });
    }

    /// Render the manage existing windows section
    fn render_manage_section(&mut self, ui: &mut Ui) {
        ui.heading("Managed Windows");
        ui.add_space(8.0);

        let windows = self.registry.list();

        if windows.is_empty() {
            ui.label("No windows created yet.");
        } else {
            // Table of windows
            egui::Grid::new("window_grid").num_columns(3).spacing([20.0, 4.0]).striped(true).show(
                ui,
                |ui| {
                    ui.label("Name");
                    ui.label("Effect");
                    ui.label("Actions");
                    ui.end_row();

                    for window in &windows {
                        ui.label(&window.name);
                        ui.label(format!(
                            "{:?}",
                            window.effect.unwrap_or(PresetEffect::RotatingHalo)
                        ));

                        ui.horizontal(|ui| {
                            if ui.button("Close").clicked() {
                                let _ = self
                                    .command_sender
                                    .send(WindowCommand::Close { id: window.id });
                            }
                            if ui.button("Set Image").clicked() {
                                self.pending_image_update_window = Some((window.id, window.size));
                            }
                        });
                        ui.end_row();
                    }
                },
            );
        }

        // Handle pending image update (file picker)
        if let Some((window_id, size)) = self.pending_image_update_window.take()
            && let Some(path) = rfd::FileDialog::new()
                .add_filter("Images", &["png", "svg", "jpg", "jpeg"])
                .pick_file()
        {
            // Load the image content with the window's size
            match Content::from_path_sized(&path, size.0, size.1) {
                Ok(content) => {
                    let _ = self.command_sender.send(WindowCommand::UpdateContent {
                        id: window_id,
                        content: Some(content),
                    });
                    log::info!("Set image for window {:?}: {:?}", window_id, path);
                }
                Err(e) => {
                    log::error!("Failed to load image for existing window: {}", e);
                }
            }
        }

        ui.add_space(8.0);

        // Close all button
        if !windows.is_empty() && ui.button("Close All Windows").clicked() {
            let _ = self.command_sender.send(WindowCommand::CloseAll);
        }
    }

    /// Create a new window with current settings
    fn create_window(&mut self) {
        // Load image content if path is selected
        let content = if let Some(path) = &self.flow_window_image_path {
            match Content::from_path_sized(path, self.new_window_size, self.new_window_size) {
                Ok(content) => Some(content),
                Err(e) => {
                    log::error!("Failed to load image: {}", e);
                    None
                }
            }
        } else {
            None
        };

        let config = WindowConfig {
            id: None,
            title: Some(self.registry.generate_name()),
            position: Position::new(self.new_window_x as f64, self.new_window_y as f64),
            size: Size::new(self.new_window_size, self.new_window_size),
            effect_margin: 0.0, // Will be calculated by builder
            shape: self.selected_shape.clone(),
            draggable: true,
            resizable: false,
            click_through: false,
            level: super::config::WindowLevel::AlwaysOnTop,
            opacity: 1.0,
            icon: None,
            content,
            widget_content: None,
            effect: None, // Effect is passed separately
            show_animation: None,
            hide_animation: None,
        };

        let _ = self.command_sender.send(WindowCommand::Create {
            config,
            effect: Some((self.selected_effect, self.effect_options.clone())),
        });

        // Move position for next window
        self.new_window_x += 60.0;
        if self.new_window_x > 1000.0 {
            self.new_window_x = 500.0;
            self.new_window_y += 60.0;
        }
    }

    /// Render the create menu bar item section (for left panel)
    fn render_create_menu_bar_section(&mut self, ui: &mut Ui) {
        ui.heading("Create Menu Bar Item");
        ui.add_space(8.0);

        ui.horizontal(|ui| {
            ui.label("Name:");
            ui.text_edit_singleline(&mut self.new_menu_bar_name);
        });

        ui.horizontal(|ui| {
            ui.label("Tooltip:");
            ui.text_edit_singleline(&mut self.new_menu_bar_tooltip);
        });

        ui.add_space(4.0);

        // Icon selection - color buttons or image file
        ui.label("Icon Color:");
        ui.horizontal(|ui| {
            if ui.button("Cyan").clicked() {
                self.tray_icon_color = [100, 200, 255];
                self.tray_icon_image_path = None;
            }
            if ui.button("Green").clicked() {
                self.tray_icon_color = [100, 255, 150];
                self.tray_icon_image_path = None;
            }
            if ui.button("Purple").clicked() {
                self.tray_icon_color = [200, 100, 255];
                self.tray_icon_image_path = None;
            }
            if ui.button("Orange").clicked() {
                self.tray_icon_color = [255, 150, 50];
                self.tray_icon_image_path = None;
            }
        });

        ui.horizontal(|ui| {
            if ui.button("Image...").clicked() {
                self.open_image_picker_for_tray();
            }
            // Show selected image or color preview
            ui.label("Selected:");
            if let Some(path) = &self.tray_icon_image_path {
                let filename = path
                    .file_name()
                    .map(|s| s.to_string_lossy().to_string())
                    .unwrap_or_else(|| "Unknown".to_string());
                ui.label(&filename);
                if ui.button("Clear").clicked() {
                    self.tray_icon_image_path = None;
                }
            } else {
                let color = egui::Color32::from_rgb(
                    self.tray_icon_color[0],
                    self.tray_icon_color[1],
                    self.tray_icon_color[2],
                );
                let (rect, _) =
                    ui.allocate_exact_size(egui::vec2(20.0, 20.0), egui::Sense::hover());
                ui.painter().circle_filled(rect.center(), 8.0, color);
            }
        });

        ui.add_space(8.0);

        if ui.button("Add Menu Bar Item").clicked() {
            self.create_menu_bar_item();
        }
    }

    /// Render the active menu bar items section (for right panel)
    fn render_active_menu_bar_section(&mut self, ui: &mut Ui) {
        ui.heading("Menu Bar Items");
        ui.add_space(8.0);

        if !self.menu_bar_items.is_empty() {
            let mut to_remove = None;
            for (idx, id) in self.menu_bar_items.iter().enumerate() {
                ui.horizontal(|ui| {
                    ui.label(format!("• {}", id));
                    if ui.small_button("Remove").clicked() {
                        to_remove = Some(idx);
                    }
                });
            }

            if let Some(idx) = to_remove {
                let id = self.menu_bar_items.remove(idx);
                let _ = self.command_sender.send(WindowCommand::RemoveMenuBarItem { id });
            }
        } else {
            ui.label("No menu bar items created.");
        }
    }

    /// Create a new menu bar item
    fn create_menu_bar_item(&mut self) {
        let id = format!("tray_{}", self.menu_bar_counter);
        self.menu_bar_counter += 1;

        // Create a simple menu with some items
        let menu = MenuBarMenu::new()
            .add_item("show_windows", "Show All Windows")
            .add_item("hide_windows", "Hide All Windows")
            .add_separator()
            .add_item("settings", "Settings...")
            .add_separator()
            .add_quit();

        // Use image icon if selected, otherwise use color-based icon
        let icon = if let Some(path) = &self.tray_icon_image_path {
            MenuBarIcon::Path(path.clone())
        } else {
            self.create_tray_icon()
        };

        let item = MenuBarItem::builder(&self.new_menu_bar_name)
            .id(&id)
            .icon(icon)
            .tooltip(&self.new_menu_bar_tooltip)
            .menu(menu)
            .build();

        let _ = self.command_sender.send(WindowCommand::AddMenuBarItem { item });
        self.menu_bar_items.push(id);

        // Update name for next item
        self.new_menu_bar_name = format!("App {}", self.menu_bar_counter);
    }

    /// Create a tray icon with the selected color
    fn create_tray_icon(&self) -> MenuBarIcon {
        let size = 22u32; // Standard macOS menu bar icon size
        let mut data = Vec::with_capacity((size * size * 4) as usize);
        let center = size as f32 / 2.0;
        let radius = center - 2.0;

        for y in 0..size {
            for x in 0..size {
                let dx = x as f32 - center;
                let dy = y as f32 - center;
                let distance = (dx * dx + dy * dy).sqrt();

                if distance <= radius {
                    // Inside circle - use selected color
                    data.extend_from_slice(&[
                        self.tray_icon_color[0],
                        self.tray_icon_color[1],
                        self.tray_icon_color[2],
                        255,
                    ]);
                } else {
                    // Outside - transparent
                    data.extend_from_slice(&[0, 0, 0, 0]);
                }
            }
        }

        MenuBarIcon::Rgba { data, width: size, height: size }
    }

    /// Open file picker for flow window image
    fn open_image_picker_for_flow_window(&mut self) {
        if let Some(path) =
            rfd::FileDialog::new().add_filter("Images", &["png", "svg", "jpg", "jpeg"]).pick_file()
        {
            self.flow_window_image_path = Some(path);
        }
    }

    /// Open file picker for tray icon image
    fn open_image_picker_for_tray(&mut self) {
        if let Some(path) =
            rfd::FileDialog::new().add_filter("Images", &["png", "svg", "jpg", "jpeg"]).pick_file()
        {
            self.tray_icon_image_path = Some(path);
        }
    }

    /// Render the Speech to Text section
    #[cfg(feature = "stt")]
    fn render_stt_section(&mut self, ui: &mut Ui) {
        // Initialize model manager if needed
        self.ensure_stt_model_manager();
        self.refresh_stt_models();

        ui.heading("Speech to Text");
        ui.add_space(8.0);

        // Status section
        ui.group(|ui| {
            ui.horizontal(|ui| {
                ui.label("Status:");
                let status_color = if self.stt_is_recording {
                    egui::Color32::RED
                } else if self.stt_initialized {
                    egui::Color32::GREEN
                } else {
                    egui::Color32::GRAY
                };
                ui.label(egui::RichText::new(&self.stt_status).color(status_color));
            });

            // Show last transcription if available
            if let Some(ref text) = self.stt_last_transcription {
                ui.add_space(4.0);
                ui.horizontal(|ui| {
                    ui.label("Last:");
                    ui.label(egui::RichText::new(text).italics());
                });
            }

            // Recording indicator
            if self.stt_is_recording {
                ui.add_space(4.0);
                ui.horizontal(|ui| {
                    ui.spinner();
                    ui.label("Recording...");
                });
            }
        });

        ui.add_space(16.0);
        ui.separator();
        ui.add_space(8.0);

        // Settings section
        ui.heading("Settings");
        ui.add_space(8.0);

        let mut config_changed = false;

        ui.group(|ui| {
            // Hotkey display
            ui.horizontal(|ui| {
                ui.label("Hotkey:");
                ui.label(self.stt_config.hotkey.display_string());
            });

            ui.add_space(4.0);

            // Hotkey enabled checkbox
            ui.horizontal(|ui| {
                ui.label("Global Hotkey:");
                let is_running = self.stt_hotkey_manager.as_ref().is_some_and(|m| m.is_running());

                let mut enabled = self.stt_config.hotkey_enabled;
                if ui.checkbox(&mut enabled, "Enabled").changed() {
                    self.stt_config.hotkey_enabled = enabled;
                    config_changed = true;

                    if enabled && !is_running {
                        self.init_stt_hotkey();
                    } else if !enabled && is_running {
                        if let Some(ref mut manager) = self.stt_hotkey_manager {
                            manager.stop();
                        }
                        self.stt_hotkey_manager = None;
                    }
                }

                // Status indicator
                if is_running {
                    ui.label(egui::RichText::new("Running").color(egui::Color32::GREEN));
                } else {
                    ui.label(egui::RichText::new("Stopped").color(egui::Color32::GRAY));
                }
            });

            ui.add_space(8.0);

            // Mode selection
            ui.label("Mode:");
            ui.horizontal(|ui| {
                if ui
                    .selectable_label(
                        self.stt_config.hotkey.mode == HotkeyMode::PushToTalk,
                        "Push to Talk",
                    )
                    .clicked()
                {
                    self.stt_config.hotkey.mode = HotkeyMode::PushToTalk;
                    config_changed = true;
                }
                if ui
                    .selectable_label(self.stt_config.hotkey.mode == HotkeyMode::Toggle, "Toggle")
                    .clicked()
                {
                    self.stt_config.hotkey.mode = HotkeyMode::Toggle;
                    config_changed = true;
                }
            });

            ui.add_space(8.0);

            // Output mode selection
            ui.label("Output:");
            ui.horizontal(|ui| {
                for mode in OutputMode::all() {
                    if ui
                        .selectable_label(self.stt_config.output_mode == *mode, mode.display_name())
                        .clicked()
                    {
                        self.stt_config.output_mode = *mode;
                        config_changed = true;
                    }
                }
            });

            ui.add_space(8.0);

            // VAD settings
            ui.horizontal(|ui| {
                if ui
                    .checkbox(&mut self.stt_config.vad_enabled, "Voice Activity Detection")
                    .changed()
                {
                    config_changed = true;
                }
            });

            if self.stt_config.vad_enabled {
                ui.horizontal(|ui| {
                    ui.label("Silence timeout:");
                    let mut seconds = self.stt_config.vad_silence_duration_ms as f32 / 1000.0;
                    if ui.add(egui::Slider::new(&mut seconds, 0.5..=5.0).suffix("s")).changed() {
                        self.stt_config.vad_silence_duration_ms = (seconds * 1000.0) as u32;
                        config_changed = true;
                    }
                });
            }
        });

        // Save config if changed
        if config_changed {
            if let Err(e) = self.stt_config.save() {
                log::error!("Failed to save STT config: {}", e);
            }
        }

        ui.add_space(16.0);
        ui.separator();
        ui.add_space(8.0);

        // Models section
        ui.heading("Models");
        ui.add_space(8.0);

        // Check for download progress updates
        let download_progress = self.stt_download_progress.lock().unwrap().clone();

        ui.group(|ui| {
            // Model selection dropdown
            ui.horizontal(|ui| {
                ui.label("Selected model:");
                egui::ComboBox::from_id_salt("stt_model_selector")
                    .selected_text(&self.stt_config.model_id)
                    .show_ui(ui, |ui| {
                        for model in &self.stt_available_models {
                            if model.is_downloaded
                                && ui
                                    .selectable_label(
                                        self.stt_config.model_id == model.id,
                                        &model.id,
                                    )
                                    .clicked()
                            {
                                self.stt_config.model_id = model.id.clone();
                                let _ = self.stt_config.save();
                            }
                        }
                    });
            });

            ui.add_space(8.0);

            // Download progress bar if downloading
            if let Some(ref progress) = download_progress {
                if progress.status == DownloadStatus::Downloading {
                    ui.horizontal(|ui| {
                        ui.label(format!("Downloading {}:", progress.model_id));
                        ui.add(
                            egui::ProgressBar::new(progress.progress())
                                .text(progress.progress_percent()),
                        );
                    });
                    ui.add_space(4.0);
                }
            }

            // Models table
            ui.label("Available models:");
            ui.add_space(4.0);

            egui::Grid::new("stt_models_grid")
                .num_columns(4)
                .striped(true)
                .spacing([8.0, 4.0])
                .show(ui, |ui| {
                    // Header
                    ui.label(egui::RichText::new("Model").strong());
                    ui.label(egui::RichText::new("Size").strong());
                    ui.label(egui::RichText::new("Status").strong());
                    ui.label(egui::RichText::new("Action").strong());
                    ui.end_row();

                    let mut model_to_download: Option<String> = None;
                    let mut model_to_delete: Option<String> = None;

                    for model in &self.stt_available_models {
                        ui.label(&model.name);
                        ui.label(model.size_display());

                        // Status
                        if model.is_downloaded {
                            ui.label(egui::RichText::new("Downloaded").color(egui::Color32::GREEN));
                        } else if let Some(ref progress) = download_progress {
                            if progress.model_id == model.id {
                                match &progress.status {
                                    DownloadStatus::Downloading => {
                                        ui.label(egui::RichText::new(progress.progress_percent()));
                                    }
                                    DownloadStatus::Failed(err) => {
                                        ui.label(
                                            egui::RichText::new("Failed").color(egui::Color32::RED),
                                        )
                                        .on_hover_text(err);
                                    }
                                    _ => {
                                        ui.label("-");
                                    }
                                }
                            } else {
                                ui.label("-");
                            }
                        } else {
                            ui.label("-");
                        }

                        // Action button
                        let is_downloading = download_progress
                            .as_ref()
                            .is_some_and(|p| p.status == DownloadStatus::Downloading);

                        if model.is_downloaded {
                            if ui.small_button("Delete").clicked() {
                                model_to_delete = Some(model.id.clone());
                            }
                        } else if is_downloading {
                            ui.add_enabled(false, egui::Button::new("..."));
                        } else if ui.small_button("Download").clicked() {
                            model_to_download = Some(model.id.clone());
                        }

                        ui.end_row();
                    }

                    // Handle download action
                    if let Some(model_id) = model_to_download {
                        self.start_stt_model_download(&model_id);
                    }

                    // Handle delete action
                    if let Some(model_id) = model_to_delete {
                        self.delete_stt_model(&model_id);
                    }
                });

            // VAD model section
            ui.add_space(8.0);
            ui.separator();
            ui.add_space(4.0);

            if let Some(ref manager) = self.stt_model_manager {
                let vad_info = manager.get_vad_model_info();
                ui.horizontal(|ui| {
                    ui.label("VAD Model:");
                    if vad_info.is_downloaded {
                        ui.label(egui::RichText::new("Downloaded").color(egui::Color32::GREEN));
                    } else {
                        ui.label(format!("Not downloaded ({})", vad_info.size_display()));
                        let is_downloading = download_progress
                            .as_ref()
                            .is_some_and(|p| p.status == DownloadStatus::Downloading);
                        if !is_downloading && ui.small_button("Download").clicked() {
                            self.start_stt_model_download("silero-vad");
                        }
                    }
                });
            }
        });
    }

    // ==================== STT Helper Methods ====================

    /// Initialize the STT model manager if not already done
    #[cfg(feature = "stt")]
    fn ensure_stt_model_manager(&mut self) {
        if self.stt_model_manager.is_none() {
            match ModelManager::new() {
                Ok(manager) => {
                    self.stt_model_manager = Some(manager);
                    self.stt_models_need_refresh = true;
                    log::info!("STT model manager initialized");
                }
                Err(e) => {
                    log::error!("Failed to create STT model manager: {}", e);
                    self.stt_status = format!("Error: {}", e);
                }
            }
        }
    }

    /// Refresh the available models list
    #[cfg(feature = "stt")]
    fn refresh_stt_models(&mut self) {
        if !self.stt_models_need_refresh {
            return;
        }

        if let Some(ref manager) = self.stt_model_manager {
            self.stt_available_models = manager.list_available_models();

            // Update status based on model availability
            let has_model = self
                .stt_available_models
                .iter()
                .any(|m| m.is_downloaded && m.id == self.stt_config.model_id);

            if has_model {
                self.stt_status = "Ready".to_string();
                self.stt_initialized = true;
            } else {
                let any_downloaded = self.stt_available_models.iter().any(|m| m.is_downloaded);
                if any_downloaded {
                    self.stt_status = "Model available (select one)".to_string();
                } else {
                    self.stt_status = "No models downloaded".to_string();
                }
                self.stt_initialized = false;
            }
        }

        self.stt_models_need_refresh = false;
    }

    /// Start downloading a model in the background
    #[cfg(feature = "stt")]
    fn start_stt_model_download(&mut self, model_id: &str) {
        let Some(ref manager) = self.stt_model_manager else {
            return;
        };

        // Clone what we need for the background thread
        let model_id_owned = model_id.to_string();
        let progress_arc = self.stt_download_progress.clone();

        // Find model info
        let model_info =
            manager.list_available_models().into_iter().find(|m| m.id == model_id).or_else(|| {
                if model_id == "silero-vad" { Some(manager.get_vad_model_info()) } else { None }
            });

        let Some(model_info) = model_info else {
            log::error!("Unknown model: {}", model_id);
            return;
        };

        // Set initial progress
        {
            let mut progress = progress_arc.lock().unwrap();
            *progress = Some(DownloadProgress {
                model_id: model_id_owned.clone(),
                downloaded_bytes: 0,
                total_bytes: model_info.size_bytes,
                status: DownloadStatus::Pending,
            });
        }

        // Spawn download thread
        let progress_for_thread = progress_arc.clone();
        thread::spawn(move || {
            log::info!("Starting download of model: {}", model_id_owned);

            // Create a new model manager in this thread
            let manager = match ModelManager::new() {
                Ok(m) => m,
                Err(e) => {
                    log::error!("Failed to create model manager: {}", e);
                    let mut progress = progress_for_thread.lock().unwrap();
                    if let Some(ref mut p) = *progress {
                        p.status = DownloadStatus::Failed(e.to_string());
                    }
                    return;
                }
            };

            // Create progress callback
            let progress_callback = progress_for_thread.clone();
            let callback = Box::new(move |p: DownloadProgress| {
                let mut progress = progress_callback.lock().unwrap();
                *progress = Some(p);
            });

            // Download the model
            match manager.download_model_sync(&model_id_owned, Some(callback)) {
                Ok(path) => {
                    log::info!("Model downloaded to: {:?}", path);
                    let mut progress = progress_for_thread.lock().unwrap();
                    if let Some(ref mut p) = *progress {
                        p.status = DownloadStatus::Completed;
                    }
                }
                Err(e) => {
                    log::error!("Failed to download model: {}", e);
                    let mut progress = progress_for_thread.lock().unwrap();
                    if let Some(ref mut p) = *progress {
                        p.status = DownloadStatus::Failed(e.to_string());
                    }
                }
            }
        });

        self.stt_models_need_refresh = true;
    }

    /// Delete a downloaded model
    #[cfg(feature = "stt")]
    fn delete_stt_model(&mut self, model_id: &str) {
        if let Some(ref manager) = self.stt_model_manager {
            if let Err(e) = manager.delete_model(model_id) {
                log::error!("Failed to delete model {}: {}", model_id, e);
            } else {
                log::info!("Deleted model: {}", model_id);
                self.stt_models_need_refresh = true;
            }
        }
    }

    // ==================== Click Helper Methods ====================

    /// Render the Click Helper section
    #[cfg(all(feature = "click_helper", target_os = "macos"))]
    fn render_click_helper_section(&mut self, ui: &mut Ui) {
        // Check accessibility permission status
        let mode = ClickHelperMode::new();
        self.click_helper_trusted = mode.is_accessibility_trusted();

        ui.heading("Click Helper");
        ui.add_space(8.0);

        // Status section
        ui.group(|ui| {
            ui.horizontal(|ui| {
                ui.label("Accessibility Permission:");
                if self.click_helper_trusted {
                    ui.label(egui::RichText::new("Granted").color(egui::Color32::GREEN));
                } else {
                    ui.label(egui::RichText::new("Not Granted").color(egui::Color32::RED));
                    if ui.button("Request").clicked() {
                        mode.request_accessibility_permission();
                    }
                }
            });

            ui.add_space(4.0);

            // Input Monitoring permission status (required for global hotkeys)
            let input_monitoring_enabled = is_input_monitoring_enabled();
            ui.horizontal(|ui| {
                ui.label("Input Monitoring:");
                if input_monitoring_enabled {
                    ui.label(egui::RichText::new("Granted").color(egui::Color32::GREEN));
                } else {
                    ui.label(egui::RichText::new("Not Granted").color(egui::Color32::RED));
                    if ui.button("Open Settings").clicked() {
                        open_input_monitoring_settings();
                    }
                }
            });

            ui.add_space(4.0);
            ui.horizontal(|ui| {
                ui.label("Hotkey:");
                ui.label(self.click_helper_config.hotkey.display_string());
            });

            ui.add_space(4.0);
            ui.horizontal(|ui| {
                ui.label("Global Hotkey:");
                let is_running =
                    self.click_helper_hotkey_manager.as_ref().is_some_and(|m| m.is_running());

                // Enabled checkbox
                let mut enabled = self.click_helper_config.hotkey_enabled;
                let checkbox = ui.add_enabled(
                    input_monitoring_enabled || enabled,
                    egui::Checkbox::new(&mut enabled, "Enabled"),
                );

                if checkbox.changed() {
                    self.click_helper_config.hotkey_enabled = enabled;
                    if let Err(e) = self.click_helper_config.save() {
                        log::error!("Failed to save Click Helper config: {}", e);
                    }

                    if enabled && !is_running {
                        self.init_click_helper_hotkey();
                        self.click_helper_hotkey_enabled = true;
                    } else if !enabled && is_running {
                        if let Some(ref mut manager) = self.click_helper_hotkey_manager {
                            manager.stop();
                        }
                        self.click_helper_hotkey_manager = None;
                        self.click_helper_hotkey_enabled = false;
                    }
                }

                // Status indicator
                if is_running {
                    ui.label(egui::RichText::new("Running").color(egui::Color32::GREEN));
                } else {
                    ui.label(egui::RichText::new("Stopped").color(egui::Color32::GRAY));
                }
            });

            // Warning about Input Monitoring
            if !input_monitoring_enabled {
                ui.add_space(4.0);
                ui.label(
                    egui::RichText::new(
                        "⚠️ Input Monitoring permission required for global hotkeys.\n\
                         Click 'Open Settings', add this app, then RESTART the app.",
                    )
                    .small()
                    .color(egui::Color32::from_rgb(255, 180, 100)),
                );
            }

            ui.add_space(8.0);
            ui.separator();
            ui.add_space(4.0);

            // Test button to manually trigger Click Helper
            if ui.button("Test Click Helper (Manual Trigger)").clicked() {
                log::info!("Manual Click Helper trigger from UI");
                if let Err(e) =
                    self.command_sender.send(super::commands::WindowCommand::StartClickHelperMode)
                {
                    log::error!("Failed to send Click Helper command: {}", e);
                }
            }
        });

        ui.add_space(16.0);
        ui.separator();
        ui.add_space(8.0);

        // Settings section
        ui.heading("Settings");
        ui.add_space(8.0);

        let mut config_changed = false;

        ui.group(|ui| {
            // Hotkey configuration
            ui.horizontal(|ui| {
                ui.label("Activation Key:");
                let mut key = self.click_helper_config.hotkey.key.clone();
                if ui.text_edit_singleline(&mut key).changed() {
                    self.click_helper_config.hotkey.key = key;
                    config_changed = true;
                }
            });

            ui.add_space(8.0);

            // Modifier checkboxes
            ui.label("Modifiers:");
            ui.horizontal(|ui| {
                let mut has_ctrl =
                    self.click_helper_config.hotkey.modifiers.contains(&ClickHelperModifier::Ctrl);
                if ui.checkbox(&mut has_ctrl, "Ctrl").changed() {
                    self.toggle_click_helper_modifier(ClickHelperModifier::Ctrl);
                    config_changed = true;
                }

                let mut has_alt =
                    self.click_helper_config.hotkey.modifiers.contains(&ClickHelperModifier::Alt);
                if ui.checkbox(&mut has_alt, "Alt").changed() {
                    self.toggle_click_helper_modifier(ClickHelperModifier::Alt);
                    config_changed = true;
                }

                let mut has_shift =
                    self.click_helper_config.hotkey.modifiers.contains(&ClickHelperModifier::Shift);
                if ui.checkbox(&mut has_shift, "Shift").changed() {
                    self.toggle_click_helper_modifier(ClickHelperModifier::Shift);
                    config_changed = true;
                }

                let mut has_meta =
                    self.click_helper_config.hotkey.modifiers.contains(&ClickHelperModifier::Meta);
                if ui.checkbox(&mut has_meta, "Cmd").changed() {
                    self.toggle_click_helper_modifier(ClickHelperModifier::Meta);
                    config_changed = true;
                }
            });

            ui.add_space(8.0);
            ui.separator();
            ui.add_space(8.0);

            // Hint characters configuration
            ui.label("Hint Characters:");
            ui.horizontal(|ui| {
                ui.label("Tier 1 (groups):");
                if ui.text_edit_singleline(&mut self.click_helper_config.tier1_chars).changed() {
                    config_changed = true;
                }
            });

            ui.horizontal(|ui| {
                ui.label("Tier 2 (selection):");
                if ui.text_edit_singleline(&mut self.click_helper_config.tier2_chars).changed() {
                    config_changed = true;
                }
            });

            ui.add_space(8.0);
            ui.separator();
            ui.add_space(8.0);

            // Appearance settings
            ui.label("Appearance:");

            ui.horizontal(|ui| {
                ui.label("Font Size:");
                if ui
                    .add(egui::Slider::new(
                        &mut self.click_helper_config.hint_font_size,
                        10.0..=24.0,
                    ))
                    .changed()
                {
                    config_changed = true;
                }
            });

            ui.horizontal(|ui| {
                ui.label("Overlay Opacity:");
                if ui
                    .add(egui::Slider::new(&mut self.click_helper_config.overlay_opacity, 50..=200))
                    .changed()
                {
                    config_changed = true;
                }
            });

            // Color pickers
            ui.horizontal(|ui| {
                ui.label("Hint Background:");
                let mut color = egui::Color32::from_rgba_unmultiplied(
                    self.click_helper_config.hint_bg_color[0],
                    self.click_helper_config.hint_bg_color[1],
                    self.click_helper_config.hint_bg_color[2],
                    self.click_helper_config.hint_bg_color[3],
                );
                if ui.color_edit_button_srgba(&mut color).changed() {
                    self.click_helper_config.hint_bg_color =
                        [color.r(), color.g(), color.b(), color.a()];
                    config_changed = true;
                }
            });

            ui.horizontal(|ui| {
                ui.label("Hint Text:");
                let mut color = egui::Color32::from_rgba_unmultiplied(
                    self.click_helper_config.hint_fg_color[0],
                    self.click_helper_config.hint_fg_color[1],
                    self.click_helper_config.hint_fg_color[2],
                    self.click_helper_config.hint_fg_color[3],
                );
                if ui.color_edit_button_srgba(&mut color).changed() {
                    self.click_helper_config.hint_fg_color =
                        [color.r(), color.g(), color.b(), color.a()];
                    config_changed = true;
                }
            });
        });

        // Save config if changed
        if config_changed {
            if let Err(e) = self.click_helper_config.save() {
                log::error!("Failed to save Click Helper config: {}", e);
            }
        }

        ui.add_space(16.0);
        ui.separator();
        ui.add_space(8.0);

        // Instructions
        ui.heading("Usage");
        ui.add_space(8.0);
        ui.label(format!(
            "Press {} to activate Click Helper mode.",
            self.click_helper_config.hotkey.display_string()
        ));
        ui.label("Type hint characters to click elements.");
        ui.label("Press ESC or Backspace to cancel.");
    }

    /// Toggle a modifier in the click helper config
    #[cfg(all(feature = "click_helper", target_os = "macos"))]
    fn toggle_click_helper_modifier(&mut self, modifier: ClickHelperModifier) {
        if let Some(pos) =
            self.click_helper_config.hotkey.modifiers.iter().position(|m| *m == modifier)
        {
            self.click_helper_config.hotkey.modifiers.remove(pos);
        } else {
            self.click_helper_config.hotkey.modifiers.push(modifier);
        }
    }
}

/// Set the dock icon visibility on macOS
///
/// Uses NSApplication setActivationPolicy to control dock visibility:
/// - Regular (0): App appears in dock and can be activated
/// - Accessory (1): App doesn't appear in dock but can be activated
#[cfg(target_os = "macos")]
#[allow(unexpected_cfgs)]
fn set_dock_icon_visibility(visible: bool) {
    unsafe {
        use objc::runtime::{Class, Object};
        use objc::{msg_send, sel, sel_impl};

        let ns_app_class = match Class::get("NSApplication") {
            Some(c) => c,
            None => {
                log::error!("Failed to get NSApplication class");
                return;
            }
        };

        let app: *mut Object = msg_send![ns_app_class, sharedApplication];

        // NSApplicationActivationPolicyRegular = 0 (dock icon visible)
        // NSApplicationActivationPolicyAccessory = 1 (dock icon hidden)
        let policy: i64 = if visible { 0 } else { 1 };

        let _: () = msg_send![app, setActivationPolicy: policy];
        log::info!("Set dock icon visibility: {}", visible);
    }
}
