// Settings Storage Adapter
use async_trait::async_trait;
use aumate_core_domain::settings::Settings;
use aumate_core_shared::InfrastructureError;
use aumate_core_traits::settings::SettingsStoragePort;
use std::path::PathBuf;
use tokio::fs;

/// File system based settings storage adapter
pub struct FileSystemSettingsAdapter {
    settings_path: PathBuf,
}

impl FileSystemSettingsAdapter {
    /// Create a new settings adapter with default path (~/.aumate/settings.json)
    pub fn new() -> Self {
        let home = dirs::home_dir().expect("Could not find home directory");
        let settings_path = home.join(".aumate").join("settings.json");
        Self { settings_path }
    }

    /// Create with custom path
    pub fn with_path(path: PathBuf) -> Self {
        Self { settings_path: path }
    }

    /// Ensure the settings directory exists
    async fn ensure_settings_dir(&self) -> Result<(), InfrastructureError> {
        if let Some(parent) = self.settings_path.parent() {
            fs::create_dir_all(parent).await.map_err(|e| {
                InfrastructureError::IoFailed(format!("Failed to create settings dir: {}", e))
            })?;
        }
        Ok(())
    }
}

impl Default for FileSystemSettingsAdapter {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl SettingsStoragePort for FileSystemSettingsAdapter {
    async fn load(&self) -> Result<Settings, InfrastructureError> {
        log::info!("Loading settings from {:?}", self.settings_path);

        if !self.settings_path.exists() {
            log::info!("Settings file not found, returning default settings");
            return Ok(Settings::default());
        }

        let content = fs::read_to_string(&self.settings_path).await.map_err(|e| {
            InfrastructureError::IoFailed(format!("Failed to read settings: {}", e))
        })?;

        let settings: Settings = serde_json::from_str(&content).unwrap_or_else(|e| {
            log::warn!("Failed to parse settings: {}, using default", e);
            Settings::default()
        });

        Ok(settings)
    }

    async fn save(&self, settings: Settings) -> Result<(), InfrastructureError> {
        log::info!("Saving settings to {:?}", self.settings_path);

        self.ensure_settings_dir().await?;

        let content = serde_json::to_string_pretty(&settings).map_err(|e| {
            InfrastructureError::SerializationFailed(format!("Failed to serialize settings: {}", e))
        })?;

        fs::write(&self.settings_path, content).await.map_err(|e| {
            InfrastructureError::IoFailed(format!("Failed to write settings: {}", e))
        })?;

        Ok(())
    }
}

