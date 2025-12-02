//! Clipboard Manager - history monitoring and persistent storage
//!
//! This module provides clipboard history management with:
//! - Background monitoring for clipboard changes
//! - SQLite-based persistent storage
//! - Support for text, images, and files
//! - Search and filter functionality
//! - Tags and categories
//! - Sensitive data detection
//! - Export/import capabilities

mod db;
mod entry;
mod export;
mod monitor;
mod sensitive;

pub use db::ClipboardDb;
pub use entry::{
    CategoryFilter, ClipboardContent, ClipboardEntry, ContentType, SensitiveDataType, Tag,
};
pub use export::{ExportData, ExportFormat};
pub use monitor::ClipboardMonitor;
pub use sensitive::SensitiveDetector;
