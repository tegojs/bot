// Settings Use Cases
use aumate_core_domain::settings::Settings;
use aumate_core_shared::{ApplicationError, DomainError};
use aumate_core_traits::settings::SettingsStoragePort;
use std::sync::Arc;

/// Get Settings Use Case
pub struct GetSettingsUseCase<P: SettingsStoragePort> {
    storage: Arc<P>,
}

impl<P: SettingsStoragePort> GetSettingsUseCase<P> {
    pub fn new(storage: Arc<P>) -> Self {
        Self { storage }
    }

    pub async fn execute(&self) -> Result<Settings, ApplicationError> {
        log::info!("GetSettingsUseCase: execute");

        self.storage.load().await.map_err(|e| e.into())
    }
}

/// Save Settings Use Case
pub struct SaveSettingsUseCase<P: SettingsStoragePort> {
    storage: Arc<P>,
}

impl<P: SettingsStoragePort> SaveSettingsUseCase<P> {
    pub fn new(storage: Arc<P>) -> Self {
        Self { storage }
    }

    pub async fn execute(&self, settings: Settings) -> Result<(), ApplicationError> {
        log::info!("SaveSettingsUseCase: execute");

        self.storage.save(settings).await.map_err(|e| e.into())
    }
}
