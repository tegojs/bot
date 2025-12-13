// 屏幕捕获适配器 (macOS)
use async_trait::async_trait;
use aumate_core_shared::{InfrastructureError, MonitorId, Rectangle, WindowId};
use aumate_core_traits::ScreenCapturePort;
use aumate_core_traits::screenshot::{
    CaptureMetadata, CaptureOptions, CaptureTarget, ColorFormat, Image, Monitor, Screenshot,
};
use std::sync::Arc;

#[cfg(target_os = "macos")]
use xcap::Monitor as XCapMonitor;

/// macOS 屏幕捕获适配器
pub struct ScreenCaptureAdapter {
    // 可以存储一些配置或缓存
}

impl ScreenCaptureAdapter {
    pub fn new() -> Self {
        log::info!("Creating ScreenCaptureAdapter (macOS)");
        Self {}
    }
}

impl Default for ScreenCaptureAdapter {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl ScreenCapturePort for ScreenCaptureAdapter {
    async fn capture(
        &self,
        target: CaptureTarget,
        _options: CaptureOptions,
    ) -> Result<Screenshot, InfrastructureError> {
        log::info!("ScreenCaptureAdapter: capturing with target={:?}", target);

        #[cfg(target_os = "macos")]
        {
            self.capture_macos(target).await
        }

        #[cfg(not(target_os = "macos"))]
        {
            Err(InfrastructureError::PlatformNotSupported)
        }
    }

    async fn get_monitors(&self) -> Result<Vec<Monitor>, InfrastructureError> {
        log::info!("ScreenCaptureAdapter: getting monitors");

        #[cfg(target_os = "macos")]
        {
            self.get_monitors_macos().await
        }

        #[cfg(not(target_os = "macos"))]
        {
            Err(InfrastructureError::PlatformNotSupported)
        }
    }

    async fn get_current_monitor(&self) -> Result<Monitor, InfrastructureError> {
        log::info!("ScreenCaptureAdapter: getting current monitor");

        let monitors = self.get_monitors().await?;
        monitors.into_iter().find(|m| m.is_primary).ok_or_else(|| {
            InfrastructureError::CaptureFailed("No primary monitor found".to_string())
        })
    }

    async fn get_focused_window(&self) -> Result<WindowId, InfrastructureError> {
        log::info!("ScreenCaptureAdapter: getting focused window");

        // TODO: 实现获取焦点窗口
        Err(InfrastructureError::PlatformOperationFailed(
            "get_focused_window not yet implemented".to_string(),
        ))
    }
}

#[cfg(target_os = "macos")]
impl ScreenCaptureAdapter {
    async fn capture_macos(
        &self,
        target: CaptureTarget,
    ) -> Result<Screenshot, InfrastructureError> {
        // 保存 target 的克隆用于后续使用
        let target_clone = target.clone();

        // 1. 确定要捕获的监视器
        let xcap_monitor = match target {
            CaptureTarget::CurrentMonitor => {
                let monitors = XCapMonitor::all()
                    .map_err(|e| InfrastructureError::CaptureFailed(e.to_string()))?;
                monitors.into_iter().find(|m| m.is_primary().unwrap_or(false)).ok_or_else(|| {
                    InfrastructureError::CaptureFailed("No primary monitor found".to_string())
                })?
            }
            CaptureTarget::Monitor(monitor_id) => {
                let monitors = XCapMonitor::all()
                    .map_err(|e| InfrastructureError::CaptureFailed(e.to_string()))?;
                monitors.into_iter().find(|m| m.id().ok() == Some(monitor_id.value())).ok_or_else(
                    || {
                        InfrastructureError::CaptureFailed(format!(
                            "Monitor {} not found",
                            monitor_id.value()
                        ))
                    },
                )?
            }
            CaptureTarget::Region(region) => {
                // 对于区域捕获，使用包含该区域的监视器
                let monitors = XCapMonitor::all()
                    .map_err(|e| InfrastructureError::CaptureFailed(e.to_string()))?;
                let monitor = self.find_monitor_for_region(&monitors, &region)?;
                monitor
            }
            CaptureTarget::AllMonitors => {
                // 捕获所有监视器 - 暂时返回主监视器
                let monitors = XCapMonitor::all()
                    .map_err(|e| InfrastructureError::CaptureFailed(e.to_string()))?;
                monitors.into_iter().find(|m| m.is_primary().unwrap_or(false)).ok_or_else(|| {
                    InfrastructureError::CaptureFailed("No primary monitor found".to_string())
                })?
            }
            CaptureTarget::FocusedWindow => {
                // 捕获焦点窗口 - TODO: 实现
                return Err(InfrastructureError::PlatformOperationFailed(
                    "FocusedWindow capture not yet implemented".to_string(),
                ));
            }
        };

        // 2. 捕获屏幕
        let xcap_image = xcap_monitor
            .capture_image()
            .map_err(|e| InfrastructureError::CaptureFailed(e.to_string()))?;

        // 3. 转换为我们的 Image 格式
        let width = xcap_image.width();
        let height = xcap_image.height();
        let raw_data = xcap_image.into_raw();

        // xcap 返回的是 RGBA 格式
        use aumate_core_domain::image::{
            ColorFormat as DomainColorFormat, ImageMetadata, ImageSource,
        };

        let image = Image::with_metadata(
            raw_data,
            width,
            height,
            DomainColorFormat::RGBA,
            ImageMetadata::new(ImageSource::Screenshot),
        )
        .map_err(|e| InfrastructureError::ImageProcessingFailed(e))?;

        // 4. 构建元数据
        let monitor_id =
            xcap_monitor.id().map_err(|e| InfrastructureError::CaptureFailed(e.to_string()))?;

        let capture_region = match &target_clone {
            CaptureTarget::Region(r) => Some(r.clone()),
            _ => None,
        };

        use aumate_core_domain::screenshot::CaptureTarget as DomainCaptureTarget;

        let domain_target = match &target_clone {
            CaptureTarget::CurrentMonitor => DomainCaptureTarget::CurrentMonitor,
            CaptureTarget::Monitor(m) => DomainCaptureTarget::Monitor { id: m.value().to_string() },
            CaptureTarget::Region(r) => DomainCaptureTarget::Region {
                x: r.min_x(),
                y: r.min_y(),
                width: r.width(),
                height: r.height(),
            },
            _ => DomainCaptureTarget::CurrentMonitor,
        };

        let metadata = CaptureMetadata::new(domain_target)
            .with_monitor_id(monitor_id.to_string())
            .with_cursor_visible(true);

        // 5. 如果是区域捕获，裁剪图像
        let final_image = if let CaptureTarget::Region(region) = target_clone {
            self.crop_image(image, region)?
        } else {
            image
        };

        Ok(Screenshot::new(final_image, metadata))
    }

    async fn get_monitors_macos(&self) -> Result<Vec<Monitor>, InfrastructureError> {
        let xcap_monitors =
            XCapMonitor::all().map_err(|e| InfrastructureError::CaptureFailed(e.to_string()))?;

        let monitors: Result<Vec<Monitor>, InfrastructureError> = xcap_monitors
            .into_iter()
            .map(|m| {
                let x = m.x().unwrap_or(0);
                let y = m.y().unwrap_or(0);
                let w = m.width().unwrap_or(800);
                let h = m.height().unwrap_or(600);

                let bounds = Rectangle::new(x, y, x + w as i32, y + h as i32)
                    .unwrap_or_else(|_| Rectangle::new(0, 0, 800, 600).unwrap());

                Ok(Monitor {
                    id: MonitorId::new(m.id().unwrap_or(0)),
                    name: m.name().unwrap_or_else(|_| "Unknown".to_string()),
                    rect: bounds,
                    is_primary: m.is_primary().unwrap_or(false),
                    scale_factor: 1.0,
                })
            })
            .collect();

        monitors
    }

    fn find_monitor_for_region(
        &self,
        monitors: &[XCapMonitor],
        region: &Rectangle,
    ) -> Result<XCapMonitor, InfrastructureError> {
        // 简单实现：找到包含区域左上角的监视器
        let region_x = region.min_x();
        let region_y = region.min_y();

        for monitor in monitors {
            let m_x = monitor.x().unwrap_or(0);
            let m_y = monitor.y().unwrap_or(0);
            let m_width = monitor.width().unwrap_or(800) as i32;
            let m_height = monitor.height().unwrap_or(600) as i32;

            if region_x >= m_x
                && region_x < m_x + m_width
                && region_y >= m_y
                && region_y < m_y + m_height
            {
                return Ok(monitor.clone());
            }
        }

        // 如果找不到，返回主监视器
        monitors.iter().find(|m| m.is_primary().unwrap_or(false)).cloned().ok_or_else(|| {
            InfrastructureError::CaptureFailed("No primary monitor found".to_string())
        })
    }

    fn crop_image(&self, image: Image, region: Rectangle) -> Result<Image, InfrastructureError> {
        use image::{DynamicImage, ImageBuffer, Rgba};

        // 验证区域在图像范围内
        let img_width = image.width as i32;
        let img_height = image.height as i32;

        if region.min_x() < 0
            || region.min_y() < 0
            || region.max_x() > img_width
            || region.max_y() > img_height
        {
            return Err(InfrastructureError::CaptureFailed(
                "Crop region out of bounds".to_string(),
            ));
        }

        // 创建 ImageBuffer
        let img_buffer: ImageBuffer<Rgba<u8>, Vec<u8>> =
            ImageBuffer::from_raw(image.width, image.height, image.data.clone()).ok_or_else(
                || InfrastructureError::ImageProcessingFailed("Invalid image data".to_string()),
            )?;

        let dynamic_img = DynamicImage::ImageRgba8(img_buffer);

        // 裁剪
        let cropped = dynamic_img.crop_imm(
            region.min_x() as u32,
            region.min_y() as u32,
            region.width() as u32,
            region.height() as u32,
        );

        let cropped_data = cropped.to_rgba8().into_raw();

        use aumate_core_domain::image::{
            ColorFormat as DomainColorFormat, ImageMetadata, ImageSource,
        };

        Image::with_metadata(
            cropped_data,
            region.width(),
            region.height(),
            DomainColorFormat::RGBA,
            ImageMetadata::new(ImageSource::Screenshot),
        )
        .map_err(|e| InfrastructureError::ImageProcessingFailed(e))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    #[cfg(target_os = "macos")]
    async fn test_get_monitors() {
        let adapter = ScreenCaptureAdapter::new();
        let result = adapter.get_monitors().await;
        assert!(result.is_ok());
        let monitors = result.unwrap();
        assert!(!monitors.is_empty());
    }

    #[test]
    fn test_adapter_creation() {
        let adapter = ScreenCaptureAdapter::new();
        // 只验证能够创建
        drop(adapter);
    }
}
