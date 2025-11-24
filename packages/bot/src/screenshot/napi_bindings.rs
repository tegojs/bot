// N-API bindings for screenshot functionality

use super::*;
use napi::bindgen_prelude::*;
use napi_derive::napi;

// Export types for N-API
#[napi(object)]
pub struct NapiScreenshotToolOptions {
    pub default_save_path: Option<String>,
    pub auto_copy_to_clipboard: Option<bool>,
}

impl From<NapiScreenshotToolOptions> for ScreenshotToolOptions {
    fn from(opts: NapiScreenshotToolOptions) -> Self {
        Self {
            default_save_path: opts.default_save_path,
            auto_copy_to_clipboard: opts.auto_copy_to_clipboard.unwrap_or(false),
        }
    }
}

#[napi(object)]
pub struct NapiInteractiveCaptureOptions {
    pub show_grid: Option<bool>,
    pub show_coordinates: Option<bool>,
    pub show_size: Option<bool>,
    pub hint_text: Option<String>,
    pub enable_window_snap: Option<bool>,
    pub snap_threshold: Option<u32>,
}

impl From<NapiInteractiveCaptureOptions> for InteractiveCaptureOptions {
    fn from(opts: NapiInteractiveCaptureOptions) -> Self {
        Self {
            show_grid: opts.show_grid.unwrap_or(false),
            show_coordinates: opts.show_coordinates.unwrap_or(true),
            show_size: opts.show_size.unwrap_or(true),
            hint_text: opts.hint_text,
            enable_window_snap: opts.enable_window_snap.unwrap_or(true),
            snap_threshold: opts.snap_threshold.unwrap_or(10),
        }
    }
}

#[napi(object)]
pub struct NapiScreenRegion {
    pub x: i32,
    pub y: i32,
    pub width: u32,
    pub height: u32,
}

impl From<NapiScreenRegion> for ScreenRegion {
    fn from(region: NapiScreenRegion) -> Self {
        Self { x: region.x, y: region.y, width: region.width, height: region.height }
    }
}

impl From<ScreenRegion> for NapiScreenRegion {
    fn from(region: ScreenRegion) -> Self {
        Self { x: region.x, y: region.y, width: region.width, height: region.height }
    }
}

#[napi(object)]
pub struct NapiScreenshotResult {
    pub image: Buffer,
    pub region: NapiScreenRegion,
    pub timestamp: i64,
}

impl From<ScreenshotResult> for NapiScreenshotResult {
    fn from(result: ScreenshotResult) -> Self {
        Self {
            image: Buffer::from(result.image),
            region: result.region.into(),
            timestamp: result.timestamp,
        }
    }
}

#[napi(object)]
pub struct NapiColorInfo {
    pub rgb: NapiRgbColor,
    pub rgba: NapiRgbaColor,
    pub hex: String,
    pub hsl: NapiHslColor,
    pub position: NapiPosition,
}

#[napi(object)]
pub struct NapiRgbColor {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

#[napi(object)]
pub struct NapiRgbaColor {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: f64,
}

#[napi(object)]
pub struct NapiHslColor {
    pub h: f64,
    pub s: f64,
    pub l: f64,
}

#[napi(object)]
pub struct NapiPosition {
    pub x: u32,
    pub y: u32,
}

impl From<ColorInfo> for NapiColorInfo {
    fn from(info: ColorInfo) -> Self {
        Self {
            rgb: NapiRgbColor { r: info.rgb.r, g: info.rgb.g, b: info.rgb.b },
            rgba: NapiRgbaColor {
                r: info.rgba.r,
                g: info.rgba.g,
                b: info.rgba.b,
                a: info.rgba.a as f64,
            },
            hex: info.hex,
            hsl: NapiHslColor { h: info.hsl.h as f64, s: info.hsl.s as f64, l: info.hsl.l as f64 },
            position: NapiPosition { x: info.position.x, y: info.position.y },
        }
    }
}

#[napi(object)]
pub struct NapiColorPickerOptions {
    pub magnifier_size: Option<u32>,
    pub zoom: Option<u32>,
    pub show_history: Option<bool>,
}

impl From<NapiColorPickerOptions> for ColorPickerOptions {
    fn from(opts: NapiColorPickerOptions) -> Self {
        Self {
            magnifier_size: opts.magnifier_size.unwrap_or(150),
            zoom: opts.zoom.unwrap_or(8),
            show_history: opts.show_history.unwrap_or(true),
        }
    }
}

#[napi(object)]
pub struct NapiSaveImageOptions {
    pub format: Option<String>,
    pub quality: Option<u8>,
}

/// Screenshot tool class
#[napi]
pub struct NapiScreenshotTool {
    tool: ScreenshotTool,
}

#[napi]
impl NapiScreenshotTool {
    #[napi(constructor)]
    pub fn new(options: Option<NapiScreenshotToolOptions>) -> Self {
        let tool_options = options.map(|o| o.into());
        Self { tool: ScreenshotTool::new(tool_options) }
    }

    /// Capture screenshot interactively (with UI overlay)
    #[napi]
    pub async fn capture_interactive(
        &self,
        options: Option<NapiInteractiveCaptureOptions>,
    ) -> Result<NapiScreenshotResult> {
        let opts = options.map(|o| o.into());
        let result =
            self.tool.capture_interactive(opts).await.map_err(|e| Error::from_reason(e))?;
        Ok(result.into())
    }

    /// Quick screenshot without interaction
    #[napi]
    pub async fn capture_quick(
        &self,
        region: Option<NapiScreenRegion>,
    ) -> Result<NapiScreenshotResult> {
        let screen_region = region.map(|r| r.into());
        let result =
            self.tool.capture_quick(screen_region).await.map_err(|e| Error::from_reason(e))?;
        Ok(result.into())
    }

    /// Get pixel color at specific coordinates
    #[napi]
    pub async fn get_pixel_color(&self, x: u32, y: u32) -> Result<NapiColorInfo> {
        let color = self.tool.get_pixel_color(x, y).await.map_err(|e| Error::from_reason(e))?;
        Ok(color.into())
    }

    /// Start interactive color picker
    #[napi]
    pub async fn pick_color(
        &self,
        options: Option<NapiColorPickerOptions>,
    ) -> Result<NapiColorInfo> {
        let opts = options.map(|o| o.into());
        let color = self.tool.pick_color(opts).await.map_err(|e| Error::from_reason(e))?;
        Ok(color.into())
    }

    /// Close and cleanup resources
    #[napi]
    pub async fn close(&self) -> Result<()> {
        self.tool.close().await;
        Ok(())
    }
}

/// Quick screenshot - capture entire screen
#[napi]
pub async fn quick_screenshot() -> Result<NapiScreenshotResult> {
    let result = capture_screen_region(None).await.map_err(|e| Error::from_reason(e))?;
    Ok(result.into())
}

/// Quick screenshot - capture specific region
#[napi]
pub async fn quick_screenshot_region(region: NapiScreenRegion) -> Result<NapiScreenshotResult> {
    let result =
        capture_screen_region(Some(region.into())).await.map_err(|e| Error::from_reason(e))?;
    Ok(result.into())
}

/// Start interactive capture
#[napi]
pub async fn start_interactive_capture(
    options: Option<NapiInteractiveCaptureOptions>,
) -> Result<NapiScreenshotResult> {
    let tool = ScreenshotTool::new(None);
    let opts = options.map(|o| o.into());
    let result = tool.capture_interactive(opts).await.map_err(|e| Error::from_reason(e))?;
    Ok(result.into())
}

/// Save screenshot result to file
#[napi]
pub async fn save_screenshot_to_file(
    result: NapiScreenshotResult,
    path: String,
    options: Option<NapiSaveImageOptions>,
) -> Result<()> {
    let screenshot_result = ScreenshotResult {
        image: result.image.to_vec(),
        region: result.region.into(),
        timestamp: result.timestamp,
    };

    let save_options = options.map(|opts| {
        let format =
            opts.format.and_then(|f| ImageFormat::from_extension(&f)).unwrap_or(ImageFormat::Png);

        SaveImageOptions { format, quality: opts.quality.unwrap_or(90) }
    });

    save_to_file(&screenshot_result, &path, save_options).map_err(|e| Error::from_reason(e))?;

    Ok(())
}

/// Copy screenshot to clipboard
#[napi]
pub async fn copy_screenshot_to_clipboard(result: NapiScreenshotResult) -> Result<()> {
    let screenshot_result = ScreenshotResult {
        image: result.image.to_vec(),
        region: result.region.into(),
        timestamp: result.timestamp,
    };

    copy_to_clipboard(&screenshot_result).map_err(|e| Error::from_reason(e))?;

    Ok(())
}
