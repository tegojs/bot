//! Font management for widget rendering

use egui::{Context, FontData, FontDefinitions, FontFamily};
use font_kit::family_name::FamilyName;
use font_kit::properties::Properties;
use font_kit::source::SystemSource;
use std::collections::HashSet;
use std::sync::{Arc, Mutex};

/// Manages loaded fonts for the widget system
pub struct FontManager {
    /// Set of already loaded font families
    loaded_fonts: Mutex<HashSet<String>>,
}

impl FontManager {
    /// Create a new FontManager
    pub fn new() -> Self {
        Self { loaded_fonts: Mutex::new(HashSet::new()) }
    }

    /// Ensure a font family is loaded into the egui context.
    /// Returns the FontFamily to use for rendering.
    pub fn ensure_font_loaded(&self, ctx: &Context, family_name: &str) -> FontFamily {
        let mut loaded = self.loaded_fonts.lock().unwrap();

        // Already loaded?
        if loaded.contains(family_name) {
            return FontFamily::Name(family_name.into());
        }

        // Try to load the font
        if let Ok(font_data) = self.load_system_font(family_name) {
            let mut fonts = FontDefinitions::default();

            // Copy existing fonts
            ctx.fonts(|f| {
                fonts = f.definitions().clone();
            });

            // Add new font data
            fonts
                .font_data
                .insert(family_name.to_string(), Arc::new(FontData::from_owned(font_data)));

            // Register as a named family with proportional fallback
            fonts.families.insert(
                FontFamily::Name(family_name.into()),
                vec![family_name.to_string(), "Proportional".to_string()],
            );

            ctx.set_fonts(fonts);

            loaded.insert(family_name.to_string());
            log::debug!("Loaded font: {}", family_name);
            FontFamily::Name(family_name.into())
        } else {
            log::warn!("Failed to load font '{}', using fallback", family_name);
            FontFamily::Proportional
        }
    }

    /// Load font data from system
    fn load_system_font(&self, family_name: &str) -> Result<Vec<u8>, String> {
        let source = SystemSource::new();

        let handle = source
            .select_best_match(&[FamilyName::Title(family_name.to_string())], &Properties::new())
            .map_err(|e| format!("Font not found: {}", e))?;

        let font = handle.load().map_err(|e| format!("Failed to load font: {}", e))?;

        font.copy_font_data()
            .ok_or_else(|| "Font data not available".to_string())
            .map(|arc| (*arc).clone())
    }
}

impl Default for FontManager {
    fn default() -> Self {
        Self::new()
    }
}
