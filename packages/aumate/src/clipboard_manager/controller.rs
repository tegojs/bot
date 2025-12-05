//! Clipboard manager feature for the controller
//!
//! Provides clipboard history monitoring and management functionality.

use std::collections::HashMap;
use std::sync::{Arc, Mutex};

use egui::TextureHandle;

use crate::clipboard_manager::{
    CategoryFilter, ClipboardContent, ClipboardDb, ClipboardEntry, ClipboardMonitor, ExportData,
};
use crate::error::Result;
use crate::gui::controller::{ControllerContext, ControllerFeature, TabInfo, format_time_ago};

/// Actions that can be performed on clipboard entries
enum ClipboardAction {
    ToggleFavorite(String),
    Copy(String),
    Delete(String),
}

/// Clipboard manager feature for clipboard history
pub struct ClipboardFeature {
    /// Clipboard database connection
    clipboard_db: Option<Arc<Mutex<ClipboardDb>>>,
    /// Clipboard monitor for background tracking
    clipboard_monitor: Option<ClipboardMonitor>,
    /// Cached clipboard entries for display
    clipboard_entries: Vec<ClipboardEntry>,
    /// Search filter text
    clipboard_search: String,
    /// Category filter
    clipboard_filter: CategoryFilter,
    /// Currently selected entry ID
    clipboard_selected: Option<String>,
    /// Flag indicating entries need to be reloaded
    clipboard_needs_refresh: bool,
    /// Total entry count for display
    clipboard_entry_count: usize,
    /// Image texture cache for clipboard images
    clipboard_image_cache: HashMap<String, TextureHandle>,
}

impl ClipboardFeature {
    pub fn new() -> Self {
        Self {
            clipboard_db: None,
            clipboard_monitor: None,
            clipboard_entries: Vec::new(),
            clipboard_search: String::new(),
            clipboard_filter: CategoryFilter::default(),
            clipboard_selected: None,
            clipboard_needs_refresh: true,
            clipboard_entry_count: 0,
            clipboard_image_cache: HashMap::new(),
        }
    }

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
}

impl Default for ClipboardFeature {
    fn default() -> Self {
        Self::new()
    }
}

impl ControllerFeature for ClipboardFeature {
    fn id(&self) -> &'static str {
        "clipboard"
    }

    fn tab_info(&self) -> TabInfo {
        TabInfo::new("clipboard", "Clipboard", 30) // After menu bar (20)
    }

    fn render(&mut self, ui: &mut egui::Ui, _ctx: &mut ControllerContext) {
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
                                            egui::RichText::new("P").color(egui::Color32::WHITE),
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
                                if let ClipboardContent::Image { data, width, height } =
                                    &entry.content
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

    fn initialize(&mut self, _ctx: &mut ControllerContext) -> Result<()> {
        log::info!("Clipboard feature initialized");
        Ok(())
    }

    fn shutdown(&mut self) {
        // Stop the monitor if running
        if let Some(monitor) = &mut self.clipboard_monitor {
            monitor.stop();
        }
        log::info!("Clipboard feature shutdown");
    }
}
