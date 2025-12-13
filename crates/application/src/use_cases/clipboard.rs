/// 剪贴板管理 Use Cases
use crate::dto::clipboard::{ClipboardContentDTO, ClipboardImageResponse, ReadClipboardResponse};
use aumate_core_domain::image::{ColorFormat, Image, ImageMetadata, ImageSource};
use aumate_core_shared::{DomainError, Result};
use aumate_core_traits::clipboard::ClipboardContent;
use aumate_core_traits::clipboard::{ClipboardPort, ClipboardType};
use std::sync::Arc;

/// 读取剪贴板 Use Case
pub struct ReadClipboardUseCase<P: ClipboardPort> {
    clipboard: Arc<P>,
}

impl<P: ClipboardPort> ReadClipboardUseCase<P> {
    pub fn new(clipboard: Arc<P>) -> Self {
        Self { clipboard }
    }

    pub async fn execute(&self) -> Result<ReadClipboardResponse> {
        log::info!("ReadClipboardUseCase: reading clipboard");

        let content = self
            .clipboard
            .read()
            .await
            .map_err(|e| DomainError::ValidationFailed(e.to_string()))?;

        let dto = match content {
            ClipboardContent::Text(text) => ClipboardContentDTO::Text(text),
            ClipboardContent::Image(data) => {
                // 假设图像是 RGBA 格式，需要根据实际情况解析
                // TODO: 从图像数据中提取宽高和格式
                ClipboardContentDTO::Image {
                    data,
                    width: 0, // Placeholder
                    height: 0,
                    format: "rgba".to_string(),
                }
            }
            ClipboardContent::Files(paths) => ClipboardContentDTO::Files(
                paths.into_iter().map(|p| p.to_string_lossy().to_string()).collect(),
            ),
        };

        Ok(ReadClipboardResponse { content: dto })
    }
}

/// 读取剪贴板图像 Use Case
pub struct ReadClipboardImageUseCase<P: ClipboardPort> {
    clipboard: Arc<P>,
}

impl<P: ClipboardPort> ReadClipboardImageUseCase<P> {
    pub fn new(clipboard: Arc<P>) -> Self {
        Self { clipboard }
    }

    pub async fn execute(&self) -> Result<ClipboardImageResponse> {
        log::info!("ReadClipboardImageUseCase: reading clipboard image");

        let content = self
            .clipboard
            .read()
            .await
            .map_err(|e| DomainError::ValidationFailed(e.to_string()))?;

        match content {
            ClipboardContent::Image(data) => {
                // TODO: 解析图像数据获取宽高
                Ok(ClipboardImageResponse { data, width: 0, height: 0, format: "rgba".to_string() })
            }
            _ => Err(DomainError::ValidationFailed("Clipboard does not contain image".to_string())),
        }
    }
}

/// 写入剪贴板 Use Case
pub struct WriteClipboardUseCase<P: ClipboardPort> {
    clipboard: Arc<P>,
}

impl<P: ClipboardPort> WriteClipboardUseCase<P> {
    pub fn new(clipboard: Arc<P>) -> Self {
        Self { clipboard }
    }

    pub async fn execute(&self, content: ClipboardContentDTO) -> Result<()> {
        log::info!("WriteClipboardUseCase: writing to clipboard");

        let domain_content = match content {
            ClipboardContentDTO::Text(text) => ClipboardContent::Text(text),
            ClipboardContentDTO::Image { data, width, height, format } => {
                // 将图像数据包装为 ClipboardContent::Image
                // 简化：直接传递原始数据
                ClipboardContent::Image(data)
            }
            ClipboardContentDTO::Files(paths) => {
                ClipboardContent::Files(paths.into_iter().map(std::path::PathBuf::from).collect())
            }
        };

        self.clipboard
            .write(domain_content)
            .await
            .map_err(|e| DomainError::ValidationFailed(e.to_string()))?;

        Ok(())
    }
}

/// 写入剪贴板图像 Use Case
pub struct WriteClipboardImageUseCase<P: ClipboardPort> {
    clipboard: Arc<P>,
}

impl<P: ClipboardPort> WriteClipboardImageUseCase<P> {
    pub fn new(clipboard: Arc<P>) -> Self {
        Self { clipboard }
    }

    pub async fn execute(&self, data: Vec<u8>, width: u32, height: u32) -> Result<()> {
        log::info!("WriteClipboardImageUseCase: writing image {}x{} to clipboard", width, height);

        // 创建 Image 对象
        let image = Image::with_metadata(
            data.clone(),
            width,
            height,
            ColorFormat::RGBA,
            ImageMetadata::new(ImageSource::Clipboard),
        )
        .map_err(|e| DomainError::InvalidImageData)?;

        // 写入剪贴板
        self.clipboard
            .write(ClipboardContent::Image(data))
            .await
            .map_err(|e| DomainError::ValidationFailed(e.to_string()))?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use async_trait::async_trait;
    use aumate_core_shared::InfrastructureError;

    struct MockClipboard {
        content: Option<ClipboardContent>,
    }

    #[async_trait]
    impl ClipboardPort for MockClipboard {
        async fn read(&self) -> std::result::Result<ClipboardContent, InfrastructureError> {
            self.content
                .clone()
                .ok_or_else(|| InfrastructureError::ClipboardFailed("Empty".to_string()))
        }

        async fn write(
            &self,
            _content: ClipboardContent,
        ) -> std::result::Result<(), InfrastructureError> {
            Ok(())
        }

        async fn clear(&self) -> std::result::Result<(), InfrastructureError> {
            Ok(())
        }

        async fn get_available_types(
            &self,
        ) -> std::result::Result<Vec<ClipboardType>, InfrastructureError> {
            Ok(vec![ClipboardType::Text])
        }
    }

    #[tokio::test]
    async fn test_read_text_from_clipboard() {
        let mock =
            Arc::new(MockClipboard { content: Some(ClipboardContent::Text("Hello".to_string())) });
        let use_case = ReadClipboardUseCase::new(mock);

        let result = use_case.execute().await.unwrap();
        match result.content {
            ClipboardContentDTO::Text(text) => assert_eq!(text, "Hello"),
            _ => panic!("Expected text content"),
        }
    }
}
