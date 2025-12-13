// 剪贴板适配器 (使用 arboard)
use arboard::{Clipboard, ImageData};
use async_trait::async_trait;
use aumate_core_shared::InfrastructureError;
use aumate_core_traits::clipboard::{ClipboardContent, ClipboardPort, ClipboardType};
use std::borrow::Cow;

/// 剪贴板适配器
pub struct ClipboardAdapter {
    // arboard 的 Clipboard 不是线程安全的，每次使用时创建新实例
}

impl ClipboardAdapter {
    pub fn new() -> Self {
        log::info!("Creating ClipboardAdapter with arboard");
        Self {}
    }
}

impl Default for ClipboardAdapter {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl ClipboardPort for ClipboardAdapter {
    async fn read(&self) -> Result<ClipboardContent, InfrastructureError> {
        log::info!("ClipboardAdapter: reading clipboard");

        tokio::task::spawn_blocking(|| {
            let mut clipboard = Clipboard::new()
                .map_err(|e| InfrastructureError::ClipboardFailed(e.to_string()))?;

            // 1. 尝试读取图像
            if let Ok(image) = clipboard.get_image() {
                let width = image.width;
                let height = image.height;
                let data = image.bytes.into_owned();

                log::info!("Read image from clipboard: {}x{}", width, height);
                return Ok(ClipboardContent::Image(data));
            }

            // 2. 尝试读取文本
            if let Ok(text) = clipboard.get_text() {
                log::info!("Read text from clipboard: {} chars", text.len());
                return Ok(ClipboardContent::Text(text));
            }

            Err(InfrastructureError::ClipboardFailed(
                "No supported content in clipboard".to_string(),
            ))
        })
        .await
        .map_err(|e| InfrastructureError::ClipboardFailed(e.to_string()))?
    }

    async fn write(&self, content: ClipboardContent) -> Result<(), InfrastructureError> {
        log::info!("ClipboardAdapter: writing to clipboard");

        tokio::task::spawn_blocking(move || {
            let mut clipboard = Clipboard::new()
                .map_err(|e| InfrastructureError::ClipboardFailed(e.to_string()))?;

            match content {
                ClipboardContent::Image(data) => {
                    // arboard 需要知道图像的宽高和格式
                    // 这里假设是 RGBA 格式，需要从外部传入宽高信息
                    // 暂时返回错误，需要改进接口
                    log::warn!("Image write not fully implemented - need width/height info");
                    Err(InfrastructureError::ClipboardFailed(
                        "Image write needs width/height information".to_string(),
                    ))
                }
                ClipboardContent::Text(text) => {
                    clipboard
                        .set_text(text)
                        .map_err(|e| InfrastructureError::ClipboardFailed(e.to_string()))?;
                    log::info!("Text written to clipboard");
                    Ok(())
                }
                ClipboardContent::Files(_paths) => {
                    log::warn!("File list write not yet implemented");
                    Err(InfrastructureError::ClipboardFailed(
                        "File write not yet implemented".to_string(),
                    ))
                }
            }
        })
        .await
        .map_err(|e| InfrastructureError::ClipboardFailed(e.to_string()))?
    }

    async fn clear(&self) -> Result<(), InfrastructureError> {
        log::info!("ClipboardAdapter: clearing clipboard");

        tokio::task::spawn_blocking(|| {
            let mut clipboard = Clipboard::new()
                .map_err(|e| InfrastructureError::ClipboardFailed(e.to_string()))?;

            clipboard.clear().map_err(|e| InfrastructureError::ClipboardFailed(e.to_string()))?;

            log::info!("Clipboard cleared");
            Ok(())
        })
        .await
        .map_err(|e| InfrastructureError::ClipboardFailed(e.to_string()))?
    }

    async fn get_available_types(&self) -> Result<Vec<ClipboardType>, InfrastructureError> {
        log::info!("ClipboardAdapter: getting available clipboard types");

        tokio::task::spawn_blocking(|| {
            let mut clipboard = Clipboard::new()
                .map_err(|e| InfrastructureError::ClipboardFailed(e.to_string()))?;

            let mut types = Vec::new();

            // 检查是否有文本
            if clipboard.get_text().is_ok() {
                types.push(ClipboardType::Text);
            }

            // 检查是否有图像
            if clipboard.get_image().is_ok() {
                types.push(ClipboardType::Image);
            }

            Ok(types)
        })
        .await
        .map_err(|e| InfrastructureError::ClipboardFailed(e.to_string()))?
    }
}

// 辅助方法：写入图像到剪贴板
impl ClipboardAdapter {
    /// 写入 RGBA 图像到剪贴板
    pub async fn write_image_rgba(
        &self,
        data: Vec<u8>,
        width: usize,
        height: usize,
    ) -> Result<(), InfrastructureError> {
        log::info!("ClipboardAdapter: writing RGBA image {}x{} to clipboard", width, height);

        tokio::task::spawn_blocking(move || {
            let mut clipboard = Clipboard::new()
                .map_err(|e| InfrastructureError::ClipboardFailed(e.to_string()))?;

            let image_data = ImageData { width, height, bytes: Cow::Owned(data) };

            clipboard
                .set_image(image_data)
                .map_err(|e| InfrastructureError::ClipboardFailed(e.to_string()))?;

            log::info!("RGBA image written to clipboard");
            Ok(())
        })
        .await
        .map_err(|e| InfrastructureError::ClipboardFailed(e.to_string()))?
    }

    /// 从剪贴板读取图像（返回 RGBA 数据和尺寸）
    pub async fn read_image_rgba(&self) -> Result<(Vec<u8>, usize, usize), InfrastructureError> {
        log::info!("ClipboardAdapter: reading RGBA image from clipboard");

        tokio::task::spawn_blocking(|| {
            let mut clipboard = Clipboard::new()
                .map_err(|e| InfrastructureError::ClipboardFailed(e.to_string()))?;

            let image = clipboard
                .get_image()
                .map_err(|e| InfrastructureError::ClipboardFailed(e.to_string()))?;

            let width = image.width;
            let height = image.height;
            let data = image.bytes.into_owned();

            log::info!("Read RGBA image {}x{} from clipboard", width, height);
            Ok((data, width, height))
        })
        .await
        .map_err(|e| InfrastructureError::ClipboardFailed(e.to_string()))?
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_clipboard_text() {
        let adapter = ClipboardAdapter::new();

        // 写入文本
        let result = adapter.write(ClipboardContent::Text("Hello, Clipboard!".to_string())).await;
        assert!(result.is_ok());

        // 读取文本
        let content = adapter.read().await;
        assert!(content.is_ok());

        if let Ok(ClipboardContent::Text(text)) = content {
            assert_eq!(text, "Hello, Clipboard!");
        } else {
            panic!("Expected text content");
        }
    }

    #[tokio::test]
    async fn test_clipboard_clear() {
        let adapter = ClipboardAdapter::new();

        // 写入文本
        let _ = adapter.write(ClipboardContent::Text("Test".to_string())).await;

        // 清空剪贴板
        let result = adapter.clear().await;
        assert!(result.is_ok());
    }
}
