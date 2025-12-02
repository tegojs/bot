//! Background clipboard monitoring

use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};
use std::thread::{self, JoinHandle};
use std::time::Duration;

use chrono::Utc;
use sha2::{Digest, Sha256};
use uuid::Uuid;

use super::db::ClipboardDb;
use super::entry::{ClipboardContent, ClipboardEntry};
use super::sensitive::SensitiveDetector;

/// Polling interval for clipboard changes (milliseconds)
const POLL_INTERVAL_MS: u64 = 500;

/// Background clipboard monitor
///
/// Monitors the system clipboard for changes and saves new content to the database.
pub struct ClipboardMonitor {
    /// Background thread handle
    handle: Option<JoinHandle<()>>,
    /// Signal to stop the monitoring thread
    stop_signal: Arc<AtomicBool>,
    /// Last seen content hash (for change detection)
    last_hash: Arc<Mutex<String>>,
    /// Whether the monitor is currently running
    running: Arc<AtomicBool>,
}

impl ClipboardMonitor {
    /// Create a new clipboard monitor
    pub fn new() -> Self {
        Self {
            handle: None,
            stop_signal: Arc::new(AtomicBool::new(false)),
            last_hash: Arc::new(Mutex::new(String::new())),
            running: Arc::new(AtomicBool::new(false)),
        }
    }

    /// Start the background monitoring thread
    ///
    /// The monitor will poll the clipboard every 500ms and save new content to the database.
    pub fn start(&mut self, db: Arc<Mutex<ClipboardDb>>) -> Result<(), String> {
        if self.running.load(Ordering::SeqCst) {
            return Err("Monitor is already running".to_string());
        }

        self.stop_signal.store(false, Ordering::SeqCst);
        self.running.store(true, Ordering::SeqCst);

        let stop_signal = Arc::clone(&self.stop_signal);
        let last_hash = Arc::clone(&self.last_hash);
        let running = Arc::clone(&self.running);

        let handle = thread::spawn(move || {
            let detector = SensitiveDetector::new();

            while !stop_signal.load(Ordering::SeqCst) {
                // Try to get clipboard content
                if let Some((content, hash)) = read_clipboard() {
                    // Check if content changed
                    let mut last = last_hash.lock().unwrap();
                    if *last != hash {
                        *last = hash.clone();
                        drop(last); // Release lock before database operation

                        // Create entry
                        let mut entry = ClipboardEntry::new(
                            Uuid::new_v4().to_string(),
                            content,
                            &hash,
                            Utc::now().to_rfc3339(),
                        );

                        // Check for sensitive data (text only)
                        if let ClipboardContent::Text(ref text) = entry.content {
                            if let Some(sensitive_type) = detector.detect(text) {
                                entry.is_sensitive = true;
                                entry.sensitive_type = Some(sensitive_type);
                            }
                        }

                        // Save to database
                        if let Ok(db) = db.lock() {
                            let _ = db.insert_entry(&entry);
                        }
                    }
                }

                thread::sleep(Duration::from_millis(POLL_INTERVAL_MS));
            }

            running.store(false, Ordering::SeqCst);
        });

        self.handle = Some(handle);
        Ok(())
    }

    /// Stop the background monitoring thread
    pub fn stop(&mut self) {
        self.stop_signal.store(true, Ordering::SeqCst);

        if let Some(handle) = self.handle.take() {
            // Wait for thread to finish (with timeout)
            let _ = handle.join();
        }
    }

    /// Check if the monitor is currently running
    pub fn is_running(&self) -> bool {
        self.running.load(Ordering::SeqCst)
    }

    /// Get the hash of the last seen clipboard content
    pub fn last_hash(&self) -> String {
        self.last_hash.lock().unwrap().clone()
    }
}

impl Default for ClipboardMonitor {
    fn default() -> Self {
        Self::new()
    }
}

impl Drop for ClipboardMonitor {
    fn drop(&mut self) {
        self.stop();
    }
}

/// Read current clipboard content and compute its hash
///
/// Returns None if clipboard is empty or inaccessible.
fn read_clipboard() -> Option<(ClipboardContent, String)> {
    use arboard::Clipboard;

    let mut clipboard = Clipboard::new().ok()?;

    // Try to get text first
    if let Ok(text) = clipboard.get_text() {
        if !text.is_empty() {
            let hash = compute_hash(text.as_bytes());
            return Some((ClipboardContent::Text(text), hash));
        }
    }

    // Try to get image
    if let Ok(image) = clipboard.get_image() {
        // Convert to PNG
        let rgba_data = image.bytes.into_owned();
        let width = image.width as u32;
        let height = image.height as u32;

        // Create image buffer and encode to PNG
        if let Some(img) = image::RgbaImage::from_raw(width, height, rgba_data.clone()) {
            let mut png_data = Vec::new();
            if let Ok(()) =
                img.write_to(&mut std::io::Cursor::new(&mut png_data), image::ImageFormat::Png)
            {
                let hash = compute_hash(&png_data);
                return Some((ClipboardContent::Image { data: png_data, width, height }, hash));
            }
        }

        // Fallback: just hash the raw data
        let hash = compute_hash(&rgba_data);
        return Some((ClipboardContent::Image { data: rgba_data, width, height }, hash));
    }

    None
}

/// Compute SHA-256 hash of data
fn compute_hash(data: &[u8]) -> String {
    let mut hasher = Sha256::new();
    hasher.update(data);
    let result = hasher.finalize();
    hex::encode(result)
}

// Simple hex encoding (avoid adding another dependency)
mod hex {
    pub fn encode(bytes: impl AsRef<[u8]>) -> String {
        bytes.as_ref().iter().map(|b| format!("{:02x}", b)).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_compute_hash() {
        let hash1 = compute_hash(b"Hello, World!");
        let hash2 = compute_hash(b"Hello, World!");
        let hash3 = compute_hash(b"Different content");

        assert_eq!(hash1, hash2);
        assert_ne!(hash1, hash3);
        assert_eq!(hash1.len(), 64); // SHA-256 produces 64 hex chars
    }

    #[test]
    fn test_hex_encode() {
        assert_eq!(hex::encode([0x00]), "00");
        assert_eq!(hex::encode([0xff]), "ff");
        assert_eq!(hex::encode([0x01, 0x02, 0x03]), "010203");
    }

    #[test]
    fn test_monitor_lifecycle() {
        let monitor = ClipboardMonitor::new();
        assert!(!monitor.is_running());
    }
}
