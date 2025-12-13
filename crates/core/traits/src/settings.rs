// Settings Storage Port
use async_trait::async_trait;
use aumate_core_domain::settings::Settings;
use aumate_core_shared::InfrastructureError;

/// Settings storage port
///
/// Responsible for loading and saving application settings
#[async_trait]
pub trait SettingsStoragePort: Send + Sync {
    /// Load settings from storage
    async fn load(&self) -> Result<Settings, InfrastructureError>;

    /// Save settings to storage
    async fn save(&self, settings: Settings) -> Result<(), InfrastructureError>;
}
