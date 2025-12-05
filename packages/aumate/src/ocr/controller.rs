//! OCR (Optical Character Recognition) feature for the controller
//!
//! Provides OCR functionality with async model loading and recognition
//! to avoid freezing the UI during heavy operations.

use std::path::PathBuf;
use std::sync::{Arc, Mutex};
use std::thread;

use egui::TextureHandle;

use crate::error::Result;
use crate::gui::controller::{AsyncTask, ControllerContext, ControllerFeature, TabInfo};
use crate::ml::{DeviceConfig, device_name, is_gpu_available};
use crate::ocr::{
    DownloadProgress, DownloadStatus, ModelInfo, ModelManager, ModelType, OcrEngine, OcrResult,
};

/// Available device options for OCR inference
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum OcrDevice {
    /// CPU (always available)
    #[default]
    Cpu,
    /// GPU acceleration (Metal on macOS, CUDA on Linux/Windows)
    Gpu,
}

/// OCR feature for optical character recognition
pub struct OcrFeature {
    /// OCR engine (loaded asynchronously)
    ocr_engine: Option<OcrEngine>,
    /// Model manager for downloading/managing models
    ocr_model_manager: Option<ModelManager>,
    /// List of available models
    ocr_available_models: Vec<ModelInfo>,
    /// Flag indicating models list needs refresh
    ocr_models_need_refresh: bool,
    /// Download progress for async downloads
    ocr_download_progress: Arc<Mutex<Option<DownloadProgress>>>,
    /// Currently selected model ID
    ocr_selected_model_id: String,
    /// Whether OCR is initialized with a valid model
    ocr_initialized: bool,
    /// Last recognition result
    ocr_last_result: Option<OcrResult>,
    /// Status message
    ocr_status: String,
    /// Current image data to process
    ocr_image_data: Option<Vec<u8>>,
    /// Texture for image preview
    ocr_image_texture: Option<TextureHandle>,
    /// Whether currently processing recognition
    ocr_is_processing: bool,
    /// Async task for model loading (to avoid UI freeze)
    load_model_task: Option<AsyncTask<std::result::Result<OcrEngine, String>>>,
    /// Async task for recognition (to avoid UI freeze)
    /// Returns (result, engine) so we can recover the engine after recognition
    recognize_task: Option<AsyncTask<(std::result::Result<OcrResult, String>, OcrEngine)>>,
    /// Selected device for inference
    selected_device: OcrDevice,
    /// Whether GPU is available on this system
    gpu_available: bool,
}

impl OcrFeature {
    pub fn new() -> Self {
        Self {
            ocr_engine: None,
            ocr_model_manager: None,
            ocr_available_models: Vec::new(),
            ocr_models_need_refresh: true,
            ocr_download_progress: Arc::new(Mutex::new(None)),
            ocr_selected_model_id: "trocr-base-printed".to_string(),
            ocr_initialized: false,
            ocr_last_result: None,
            ocr_status: "Not initialized".to_string(),
            ocr_image_data: None,
            ocr_image_texture: None,
            ocr_is_processing: false,
            load_model_task: None,
            recognize_task: None,
            selected_device: OcrDevice::default(),
            gpu_available: is_gpu_available(),
        }
    }

    /// Initialize the OCR model manager if not already done
    fn ensure_ocr_model_manager(&mut self) {
        if self.ocr_model_manager.is_none() {
            match ModelManager::new() {
                Ok(manager) => {
                    self.ocr_model_manager = Some(manager);
                }
                Err(e) => {
                    log::error!("Failed to create OCR model manager: {}", e);
                    self.ocr_status = format!("Error: {}", e);
                }
            }
        }
    }

    /// Refresh the available models list
    fn refresh_ocr_models(&mut self) {
        if !self.ocr_models_need_refresh {
            return;
        }

        if let Some(ref manager) = self.ocr_model_manager {
            self.ocr_available_models = manager.list_trocr_models();

            // Update status based on model availability
            let has_model = self
                .ocr_available_models
                .iter()
                .any(|m| m.is_downloaded && m.id == self.ocr_selected_model_id);

            if has_model && self.ocr_engine.is_some() {
                self.ocr_status = "Ready".to_string();
                self.ocr_initialized = true;
            } else if has_model {
                self.ocr_status = "Model available (click Load Model)".to_string();
                self.ocr_initialized = false;
            } else {
                let any_downloaded = self.ocr_available_models.iter().any(|m| m.is_downloaded);
                if any_downloaded {
                    self.ocr_status = "Model available (select one)".to_string();
                } else {
                    self.ocr_status = "No models downloaded".to_string();
                }
                self.ocr_initialized = false;
            }
        }

        self.ocr_models_need_refresh = false;
    }

    /// Start downloading a model in the background
    fn start_ocr_model_download(&mut self, model_id: &str) {
        let Some(ref manager) = self.ocr_model_manager else {
            return;
        };

        let model_id_owned = model_id.to_string();
        let progress_arc = self.ocr_download_progress.clone();

        // Find model info
        let model_info = manager.list_trocr_models().into_iter().find(|m| m.id == model_id);

        let Some(model_info) = model_info else {
            log::error!("Unknown model: {}", model_id);
            return;
        };

        // Set initial progress
        {
            let mut progress = progress_arc.lock().unwrap();
            *progress = Some(DownloadProgress {
                model_id: model_id_owned.clone(),
                current_file: String::new(),
                file_index: 0,
                total_files: model_info.files.len(),
                downloaded_bytes: 0,
                total_bytes: model_info.size_bytes,
                status: DownloadStatus::Pending,
            });
        }

        // Spawn download thread
        let progress_for_thread = progress_arc.clone();
        thread::spawn(move || {
            log::info!("Starting download of OCR model: {}", model_id_owned);

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
            match manager.download_model_sync(ModelType::TrOCR, &model_id_owned, Some(callback)) {
                Ok(path) => {
                    log::info!("OCR model downloaded to: {:?}", path);
                    let mut progress = progress_for_thread.lock().unwrap();
                    if let Some(ref mut p) = *progress {
                        p.status = DownloadStatus::Completed;
                    }
                }
                Err(e) => {
                    log::error!("Failed to download OCR model: {}", e);
                    let mut progress = progress_for_thread.lock().unwrap();
                    if let Some(ref mut p) = *progress {
                        p.status = DownloadStatus::Failed(e.to_string());
                    }
                }
            }
        });

        self.ocr_models_need_refresh = true;
    }

    /// Start loading model asynchronously (non-blocking)
    fn start_model_load(&mut self, model_path: PathBuf) {
        let task = AsyncTask::new();
        let callback = task.callback();
        let use_gpu = self.selected_device == OcrDevice::Gpu;

        self.ocr_status = "Loading model...".to_string();

        thread::spawn(move || {
            log::info!("Loading OCR model async: {:?} (GPU: {})", model_path, use_gpu);
            let device_config =
                if use_gpu { DeviceConfig::with_gpu() } else { DeviceConfig::cpu_only() };
            let result = match OcrEngine::with_device(device_config) {
                Ok(mut engine) => {
                    engine.load_model(&model_path).map(|_| engine).map_err(|e| e.to_string())
                }
                Err(e) => Err(e.to_string()),
            };
            callback(result);
        });

        self.load_model_task = Some(task);
    }

    /// Start recognition asynchronously (non-blocking)
    fn start_recognition(&mut self) {
        let Some(image_data) = self.ocr_image_data.clone() else {
            self.ocr_status = "No image loaded".to_string();
            return;
        };

        let Some(engine) = self.ocr_engine.take() else {
            self.ocr_status = "No model loaded".to_string();
            return;
        };

        let task = AsyncTask::new();
        let callback = task.callback();

        self.ocr_status = "Recognizing...".to_string();
        self.ocr_is_processing = true;

        thread::spawn(move || {
            log::info!("Starting OCR recognition in background thread");
            let mut engine = engine;

            let result = engine.recognize_bytes(&image_data).map_err(|e| e.to_string());

            // Return both the result and the engine so we can recover it
            callback((result, engine));
        });

        self.recognize_task = Some(task);
    }

    /// Check for async task completion (model loading and recognition)
    fn check_async_tasks(&mut self, ctx: &mut ControllerContext) {
        // Check model loading task
        if let Some(ref task) = self.load_model_task {
            if let Some(result) = task.take() {
                match result {
                    Ok(engine) => {
                        let device_name = device_name(engine.device());
                        self.ocr_engine = Some(engine);
                        self.ocr_status = format!("Model loaded ({}) - Ready", device_name);
                        self.ocr_initialized = true;
                        log::info!("OCR model loaded successfully on {}", device_name);
                    }
                    Err(e) => {
                        self.ocr_status = format!("Failed to load model: {}", e);
                        log::error!("Failed to load OCR model: {}", e);
                    }
                }
                self.load_model_task = None;
                ctx.request_repaint();
            }
        }

        // Check recognition task
        if let Some(ref task) = self.recognize_task {
            if let Some((result, engine)) = task.take() {
                // Recover the engine
                self.ocr_engine = Some(engine);
                self.ocr_is_processing = false;

                match result {
                    Ok(ocr_result) => {
                        log::info!("OCR recognition completed: {}ms", ocr_result.duration_ms);
                        self.ocr_status =
                            format!("Recognition complete ({}ms)", ocr_result.duration_ms);
                        self.ocr_last_result = Some(ocr_result);
                    }
                    Err(e) => {
                        self.ocr_status = format!("Recognition failed: {}", e);
                        log::error!("OCR recognition failed: {}", e);
                        self.ocr_last_result = None;
                    }
                }
                self.recognize_task = None;
                ctx.request_repaint();
            }
        }
    }
}

impl Default for OcrFeature {
    fn default() -> Self {
        Self::new()
    }
}

impl ControllerFeature for OcrFeature {
    fn id(&self) -> &'static str {
        "ocr"
    }

    fn tab_info(&self) -> TabInfo {
        TabInfo::new("ocr", "OCR", 50) // After STT (40)
    }

    fn update(&mut self, ctx: &mut ControllerContext) {
        // Check for async task completion
        self.check_async_tasks(ctx);
    }

    fn render(&mut self, ui: &mut egui::Ui, ctx: &mut ControllerContext) {
        // Initialize model manager if needed
        self.ensure_ocr_model_manager();
        self.refresh_ocr_models();

        ui.heading("Optical Character Recognition");
        ui.add_space(8.0);

        // Status section
        ui.group(|ui| {
            ui.horizontal(|ui| {
                ui.label("Status:");
                let status_color = if self.ocr_is_processing || self.load_model_task.is_some() {
                    egui::Color32::YELLOW
                } else if self.ocr_initialized {
                    egui::Color32::GREEN
                } else {
                    egui::Color32::GRAY
                };
                ui.label(egui::RichText::new(&self.ocr_status).color(status_color));
            });

            // Show last result if available
            if let Some(ref result) = self.ocr_last_result {
                ui.add_space(4.0);
                ui.horizontal(|ui| {
                    ui.label("Last:");
                    ui.label(
                        egui::RichText::new(if result.text.len() > 50 {
                            format!("{}...", &result.text[..50])
                        } else {
                            result.text.clone()
                        })
                        .italics(),
                    );
                });
            }

            // Processing indicator
            if self.ocr_is_processing || self.load_model_task.is_some() {
                ui.add_space(4.0);
                ui.horizontal(|ui| {
                    ui.spinner();
                    if self.load_model_task.is_some() {
                        ui.label("Loading model...");
                    } else {
                        ui.label("Recognizing...");
                    }
                });
            }
        });

        ui.add_space(16.0);
        ui.separator();
        ui.add_space(8.0);

        // Models section
        ui.heading("Models");
        ui.add_space(8.0);

        // Check for download progress updates
        let download_progress = self.ocr_download_progress.lock().unwrap().clone();

        ui.group(|ui| {
            // Model selection dropdown
            ui.horizontal(|ui| {
                ui.label("Selected model:");
                egui::ComboBox::from_id_salt("ocr_model_selector")
                    .selected_text(&self.ocr_selected_model_id)
                    .show_ui(ui, |ui| {
                        for model in &self.ocr_available_models {
                            if model.is_downloaded
                                && ui
                                    .selectable_label(
                                        self.ocr_selected_model_id == model.id,
                                        &model.id,
                                    )
                                    .clicked()
                            {
                                self.ocr_selected_model_id = model.id.clone();
                                // Unload current engine when switching models
                                self.ocr_engine = None;
                                self.ocr_initialized = false;
                            }
                        }
                    });
            });

            ui.add_space(4.0);

            // Device selection
            ui.horizontal(|ui| {
                ui.label("Device:");

                // CPU option (always available)
                let cpu_selected = self.selected_device == OcrDevice::Cpu;
                if ui.selectable_label(cpu_selected, "CPU").clicked()
                    && self.selected_device != OcrDevice::Cpu
                {
                    self.selected_device = OcrDevice::Cpu;
                    // Unload engine when switching device
                    self.ocr_engine = None;
                    self.ocr_initialized = false;
                    self.ocr_status = "Device changed - reload model".to_string();
                }

                // GPU option (only if available)
                let gpu_text = if cfg!(target_os = "macos") { "Metal" } else { "CUDA" };

                ui.add_enabled_ui(self.gpu_available, |ui| {
                    let gpu_selected = self.selected_device == OcrDevice::Gpu;
                    if ui.selectable_label(gpu_selected, gpu_text).clicked()
                        && self.selected_device != OcrDevice::Gpu
                    {
                        self.selected_device = OcrDevice::Gpu;
                        // Unload engine when switching device
                        self.ocr_engine = None;
                        self.ocr_initialized = false;
                        self.ocr_status = "Device changed - reload model".to_string();
                    }
                });

                if !self.gpu_available {
                    ui.label(
                        egui::RichText::new("(GPU not available)")
                            .small()
                            .color(egui::Color32::GRAY),
                    );
                }
            });

            ui.add_space(8.0);

            // Download progress bar if downloading
            if let Some(ref progress) = download_progress {
                if progress.status == DownloadStatus::Downloading {
                    ui.horizontal(|ui| {
                        ui.label(format!("Downloading {}:", progress.model_id));
                        ui.add(egui::ProgressBar::new(progress.overall_progress()).text(format!(
                            "{} ({}/{})",
                            progress.progress_percent(),
                            progress.file_index,
                            progress.total_files
                        )));
                    });
                    ui.add_space(4.0);
                }
            }

            // Models table
            ui.label("Available models:");
            ui.add_space(4.0);

            egui::Grid::new("ocr_models_grid")
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

                    for model in &self.ocr_available_models {
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
                        if model.is_downloaded {
                            if ui.button("Delete").clicked() {
                                model_to_delete = Some(model.id.clone());
                            }
                        } else {
                            let is_downloading = download_progress.as_ref().is_some_and(|p| {
                                p.model_id == model.id && p.status == DownloadStatus::Downloading
                            });

                            if is_downloading {
                                ui.add_enabled(false, egui::Button::new("..."));
                            } else if ui.button("Download").clicked() {
                                model_to_download = Some(model.id.clone());
                            }
                        }

                        ui.end_row();
                    }

                    // Handle actions after the grid
                    if let Some(model_id) = model_to_download {
                        self.start_ocr_model_download(&model_id);
                    }
                    if let Some(model_id) = model_to_delete {
                        if let Some(ref manager) = self.ocr_model_manager {
                            let _ = manager.delete_model(ModelType::TrOCR, &model_id);
                            self.ocr_models_need_refresh = true;
                            if self.ocr_selected_model_id == model_id {
                                self.ocr_engine = None;
                                self.ocr_initialized = false;
                            }
                        }
                    }
                });

            // Refresh when download completes
            if download_progress.as_ref().is_some_and(|p| p.status == DownloadStatus::Completed) {
                self.ocr_models_need_refresh = true;
                let mut progress = self.ocr_download_progress.lock().unwrap();
                *progress = None;
            }
        });

        ui.add_space(16.0);
        ui.separator();
        ui.add_space(8.0);

        // Image input section
        ui.heading("Image Input");
        ui.add_space(8.0);

        ui.group(|ui| {
            ui.horizontal(|ui| {
                if ui.button("Load Image...").clicked() {
                    if let Some(path) = rfd::FileDialog::new()
                        .add_filter("Images", &["png", "jpg", "jpeg", "bmp", "gif", "webp"])
                        .pick_file()
                    {
                        match std::fs::read(&path) {
                            Ok(data) => {
                                self.ocr_image_data = Some(data);
                                self.ocr_image_texture = None;
                                self.ocr_status = format!(
                                    "Loaded: {}",
                                    path.file_name().unwrap_or_default().to_string_lossy()
                                );
                            }
                            Err(e) => {
                                self.ocr_status = format!("Failed to load image: {}", e);
                            }
                        }
                    }
                }

                if ui.button("Paste from Clipboard").clicked() {
                    if let Ok(image_data) = crate::clipboard::get_image() {
                        self.ocr_image_data = Some(image_data);
                        self.ocr_image_texture = None;
                        self.ocr_status = "Image pasted from clipboard".to_string();
                    } else {
                        self.ocr_status = "No image in clipboard".to_string();
                    }
                }

                if self.ocr_image_data.is_some() && ui.button("Clear").clicked() {
                    self.ocr_image_data = None;
                    self.ocr_image_texture = None;
                    self.ocr_last_result = None;
                }
            });

            // Show image preview if loaded
            if let Some(ref image_data) = self.ocr_image_data {
                ui.add_space(8.0);

                if self.ocr_image_texture.is_none() {
                    if let Ok(image) = image::load_from_memory(image_data) {
                        let rgba = image.to_rgba8();
                        let size = [rgba.width() as usize, rgba.height() as usize];
                        let color_image = egui::ColorImage::from_rgba_unmultiplied(size, &rgba);
                        let texture = ui.ctx().load_texture(
                            "ocr_preview",
                            color_image,
                            egui::TextureOptions::default(),
                        );
                        self.ocr_image_texture = Some(texture);
                    }
                }

                if let Some(ref texture) = self.ocr_image_texture {
                    let available_width = ui.available_width();
                    let max_height = 150.0;
                    let aspect = texture.size()[0] as f32 / texture.size()[1] as f32;
                    let width = available_width.min(max_height * aspect);
                    let height = width / aspect;
                    ui.image((texture.id(), egui::vec2(width, height)));
                }
            }
        });

        ui.add_space(16.0);
        ui.separator();
        ui.add_space(8.0);

        // Recognition section
        ui.heading("Recognition");
        ui.add_space(8.0);

        ui.group(|ui| {
            // Load model button if not loaded
            let has_downloaded_model = self
                .ocr_available_models
                .iter()
                .any(|m| m.is_downloaded && m.id == self.ocr_selected_model_id);

            let is_loading_model = self.load_model_task.is_some();

            if self.ocr_engine.is_none() && has_downloaded_model && !is_loading_model {
                if ui.button("Load Model").clicked() {
                    if let Some(ref manager) = self.ocr_model_manager {
                        if let Some(model_path) =
                            manager.get_model_path(ModelType::TrOCR, &self.ocr_selected_model_id)
                        {
                            self.start_model_load(model_path);
                        }
                    }
                }
                ui.add_space(4.0);
            }

            let can_recognize = self.ocr_image_data.is_some()
                && self.ocr_engine.is_some()
                && !self.ocr_is_processing;

            ui.add_enabled_ui(can_recognize, |ui| {
                if ui.button("Recognize Text").clicked() {
                    self.start_recognition();
                }
            });

            if self.ocr_image_data.is_none() {
                ui.label(egui::RichText::new("Load an image first").color(egui::Color32::GRAY));
            } else if self.ocr_engine.is_none() {
                if !has_downloaded_model {
                    ui.label(
                        egui::RichText::new("Download a model first").color(egui::Color32::GRAY),
                    );
                } else if is_loading_model {
                    ui.label(egui::RichText::new("Loading model...").color(egui::Color32::YELLOW));
                }
            }

            // Show results
            if let Some(ref result) = self.ocr_last_result {
                ui.add_space(8.0);
                ui.separator();
                ui.add_space(4.0);

                ui.horizontal(|ui| {
                    ui.label(egui::RichText::new("Result:").strong());
                    ui.label(format!("({}ms)", result.duration_ms));
                    if ui.button("Copy").clicked() {
                        let _ = crate::clipboard::set_text(&result.text);
                    }
                });

                ui.add_space(4.0);

                egui::ScrollArea::vertical().max_height(120.0).show(ui, |ui| {
                    ui.add(
                        egui::TextEdit::multiline(&mut result.text.as_str())
                            .desired_width(f32::INFINITY)
                            .interactive(false),
                    );
                });
            }
        });

        // Request repaint if we have pending async tasks
        if self.load_model_task.is_some() || self.recognize_task.is_some() {
            ctx.request_repaint();
        }
    }

    fn initialize(&mut self, _ctx: &mut ControllerContext) -> Result<()> {
        log::info!("OCR feature initialized");
        Ok(())
    }
}
