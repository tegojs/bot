//! STT (Speech-to-Text) feature for the controller
//!
//! Provides speech-to-text functionality with model management and hotkey support.

use std::sync::{Arc, Mutex};
use std::thread;

use crate::error::Result;
use crate::gui::controller::{ControllerContext, ControllerFeature, TabInfo};
use crate::stt::{
    DownloadProgress, DownloadStatus, HotkeyEvent as SttHotkeyEvent,
    HotkeyManager as SttHotkeyManager, HotkeyMode, ModelInfo, ModelManager, OutputMode, SttConfig,
};

/// STT feature for speech-to-text
pub struct SttFeature {
    /// STT configuration
    stt_config: SttConfig,
    /// Model manager for downloading/managing models
    stt_model_manager: Option<ModelManager>,
    /// List of available models
    stt_available_models: Vec<ModelInfo>,
    /// Flag indicating models list needs refresh
    stt_models_need_refresh: bool,
    /// Download progress for async downloads
    stt_download_progress: Arc<Mutex<Option<DownloadProgress>>>,
    /// Whether STT is initialized with a valid model
    stt_initialized: bool,
    /// Whether currently recording
    stt_is_recording: bool,
    /// Last transcription result
    stt_last_transcription: Option<String>,
    /// Status message
    stt_status: String,
    /// Hotkey manager for global hotkeys
    stt_hotkey_manager: Option<SttHotkeyManager>,
}

impl SttFeature {
    pub fn new() -> Self {
        Self {
            stt_config: SttConfig::load().unwrap_or_default(),
            stt_model_manager: None,
            stt_available_models: Vec::new(),
            stt_models_need_refresh: true,
            stt_download_progress: Arc::new(Mutex::new(None)),
            stt_initialized: false,
            stt_is_recording: false,
            stt_last_transcription: None,
            stt_status: "Not initialized".to_string(),
            stt_hotkey_manager: None,
        }
    }

    /// Initialize STT hotkey manager
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

    /// Initialize the STT model manager if not already done
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
}

impl Default for SttFeature {
    fn default() -> Self {
        Self::new()
    }
}

impl ControllerFeature for SttFeature {
    fn id(&self) -> &'static str {
        "stt"
    }

    fn tab_info(&self) -> TabInfo {
        TabInfo::new("stt", "Speech to Text", 40) // After clipboard (30)
    }

    fn render(&mut self, ui: &mut egui::Ui, _ctx: &mut ControllerContext) {
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

    fn initialize(&mut self, _ctx: &mut ControllerContext) -> Result<()> {
        log::info!("STT feature initialized");

        // Start hotkey manager if enabled
        if self.stt_config.hotkey_enabled {
            self.init_stt_hotkey();
        }

        Ok(())
    }

    fn shutdown(&mut self) {
        // Stop hotkey manager if running
        if let Some(ref mut manager) = self.stt_hotkey_manager {
            manager.stop();
        }
        log::info!("STT feature shutdown");
    }
}
