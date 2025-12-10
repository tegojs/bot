use serde::{Deserialize, Serialize};

/// Rectangle representing an element's bounds
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct ElementRect {
    pub min_x: i32,
    pub min_y: i32,
    pub max_x: i32,
    pub max_y: i32,
}

impl ElementRect {
    pub fn new(min_x: i32, min_y: i32, max_x: i32, max_y: i32) -> Self {
        Self {
            min_x,
            min_y,
            max_x,
            max_y,
        }
    }

    pub fn width(&self) -> i32 {
        self.max_x - self.min_x
    }

    pub fn height(&self) -> i32 {
        self.max_y - self.min_y
    }

    pub fn contains_point(&self, x: i32, y: i32) -> bool {
        x >= self.min_x && x <= self.max_x && y >= self.min_y && y <= self.max_y
    }
}

/// Window element with its bounds and ID
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WindowElement {
    pub rect: ElementRect,
    pub window_id: u32,
    pub title: String,
    pub app_name: String,
}

/// Image format for saving screenshots
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ImageFormat {
    Png,
    Webp,
    Jpeg,
}

impl Default for ImageFormat {
    fn default() -> Self {
        Self::Png
    }
}

/// Screenshot capture region
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CaptureRegion {
    pub x: u32,
    pub y: u32,
    pub width: u32,
    pub height: u32,
}
