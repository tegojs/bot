use aumate_core_shared::Rectangle;
/// Monitor information utilities
///
/// **迁移**: 从 app-utils/src/monitor_info.rs (仅提取 macOS 使用的部分)
use image::DynamicImage;
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};
use serde::Serialize;
use xcap::Monitor;

#[derive(Debug)]
pub struct MonitorInfo {
    pub monitor: Monitor,
    pub rect: Rectangle,
    pub scale_factor: f32,
    #[cfg(target_os = "macos")]
    pub monitor_scale_factor: f64,
}

#[derive(Debug, Clone, Copy)]
pub enum ColorFormat {
    Rgba8,
    Rgb8,
}

#[derive(Serialize, Clone)]
pub struct MonitorRect {
    pub rect: Rectangle,
    pub scale_factor: f32,
}

impl MonitorInfo {
    #[cfg(target_os = "macos")]
    pub fn new(monitor: &Monitor) -> Self {
        // xcap 0.7.1 API: x(), y(), width(), height() return Result
        let x = monitor.x().unwrap_or(0);
        let y = monitor.y().unwrap_or(0);
        let width = monitor.width().unwrap_or(0);
        let height = monitor.height().unwrap_or(0);
        let monitor_scale_factor = monitor.scale_factor().unwrap_or(1.0) as f64;

        let monitor_rect = Rectangle::from_bounds(
            (x as f64 * monitor_scale_factor) as i32,
            (y as f64 * monitor_scale_factor) as i32,
            ((x + width as i32) as f64 * monitor_scale_factor) as i32,
            ((y + height as i32) as f64 * monitor_scale_factor) as i32,
        );
        let scale_factor = 0.0;

        MonitorInfo {
            monitor: monitor.clone(),
            rect: monitor_rect,
            scale_factor,
            monitor_scale_factor,
        }
    }

    pub fn get_monitor_crop_region(&self, crop_region: Rectangle) -> Rectangle {
        let monitor_crop_region = self.rect.clip_rect(&crop_region);

        Rectangle::from_bounds(
            monitor_crop_region.min_x() - self.rect.min_x(),
            monitor_crop_region.min_y() - self.rect.min_y(),
            monitor_crop_region.max_x() - self.rect.min_x(),
            monitor_crop_region.max_y() - self.rect.min_y(),
        )
    }

    #[cfg(target_os = "macos")]
    pub fn capture(
        &self,
        crop_area: Option<Rectangle>,
        _exclude_window: Option<&tauri::Window>,
        color_format: ColorFormat,
    ) -> Option<DynamicImage> {
        capture_target_monitor(&self.monitor, crop_area, color_format)
    }
}

#[derive(Debug)]
pub struct MonitorList(Vec<MonitorInfo>);

impl MonitorList {
    fn get_monitors(region: Option<Rectangle>) -> MonitorList {
        let monitors = Monitor::all().unwrap_or_default();

        let region = region
            .unwrap_or_else(|| Rectangle::from_bounds(i32::MIN, i32::MIN, i32::MAX, i32::MAX));

        let monitor_info_list = monitors
            .iter()
            .map(|monitor| MonitorInfo::new(monitor))
            .filter(|monitor| monitor.rect.intersects(&region))
            .collect::<Vec<MonitorInfo>>();

        MonitorList(monitor_info_list)
    }

    pub fn all(_ignore_sdr_info: bool) -> MonitorList {
        Self::get_monitors(None)
    }

    pub fn get_by_region(region: Rectangle, _ignore_sdr_info: bool) -> MonitorList {
        Self::get_monitors(Some(region))
    }

    /// 获取所有显示器的最小矩形
    pub fn get_monitors_bounding_box(&self) -> Rectangle {
        let monitors = &self.0;

        if monitors.is_empty() {
            return Rectangle::from_bounds(0, 0, 0, 0);
        }

        let mut min_x = i32::MAX;
        let mut min_y = i32::MAX;
        let mut max_x = i32::MIN;
        let mut max_y = i32::MIN;

        for monitor in monitors {
            if monitor.rect.min_x() < min_x {
                min_x = monitor.rect.min_x();
            }
            if monitor.rect.min_y() < min_y {
                min_y = monitor.rect.min_y();
            }
            if monitor.rect.max_x() > max_x {
                max_x = monitor.rect.max_x();
            }
            if monitor.rect.max_y() > max_y {
                max_y = monitor.rect.max_y();
            }
        }

        Rectangle::from_bounds(min_x, min_y, max_x, max_y)
    }

    /// 捕获所有显示器，拼接为一个完整的图像
    pub async fn capture_async(
        &self,
        crop_region: Option<Rectangle>,
        exclude_window: Option<&tauri::Window>,
        color_format: ColorFormat,
    ) -> Result<DynamicImage, String> {
        let monitors = &self.0;

        // 特殊情况，只有一个显示器，直接返回
        if monitors.len() == 1 {
            let first_monitor = monitors.first().unwrap();
            let capture_image = first_monitor.capture(
                crop_region.map(|r| first_monitor.get_monitor_crop_region(r)),
                exclude_window,
                color_format,
            );

            return match capture_image {
                Some(capture_image) => Ok(capture_image),
                None => {
                    Err(format!("[MonitorList::capture_async] Failed to capture monitor image"))
                }
            };
        }

        // 将每个显示器截取的图像，绘制到该图像上
        let monitor_image_list = monitors
            .par_iter()
            .filter_map(|monitor| {
                let capture_image = monitor.capture(
                    crop_region.map(|r| monitor.get_monitor_crop_region(r)),
                    exclude_window,
                    color_format,
                );

                capture_image.map(|image| (monitor, image))
            })
            .collect::<Vec<(&MonitorInfo, DynamicImage)>>();

        if monitor_image_list.is_empty() {
            return Err("[MonitorList::capture_async] No monitor captured".to_string());
        }

        // 获取显示器的边界
        let monitors_bounding_box = self.get_monitors_bounding_box();
        let full_width = (monitors_bounding_box.max_x() - monitors_bounding_box.min_x()) as u32;
        let full_height = (monitors_bounding_box.max_y() - monitors_bounding_box.min_y()) as u32;

        let mut full_image = match color_format {
            ColorFormat::Rgb8 => DynamicImage::new_rgb8(full_width, full_height),
            ColorFormat::Rgba8 => DynamicImage::new_rgba8(full_width, full_height),
        };

        // 绘制每个显示器的图像到完整图像上
        for (monitor, capture_image) in monitor_image_list {
            let offset_x = (monitor.rect.min_x() - monitors_bounding_box.min_x()) as u32;
            let offset_y = (monitor.rect.min_y() - monitors_bounding_box.min_y()) as u32;

            image::imageops::overlay(
                &mut full_image,
                &capture_image,
                offset_x as i64,
                offset_y as i64,
            );
        }

        Ok(full_image)
    }

    pub fn capture(
        &self,
        crop_region: Option<Rectangle>,
        exclude_window: Option<&tauri::Window>,
        color_format: ColorFormat,
    ) -> Result<DynamicImage, String> {
        tokio::runtime::Handle::current().block_on(self.capture_async(
            crop_region,
            exclude_window,
            color_format,
        ))
    }
}

/// 捕获指定监视器的图像 (macOS)
#[cfg(target_os = "macos")]
fn capture_target_monitor(
    monitor: &Monitor,
    crop_area: Option<Rectangle>,
    color_format: ColorFormat,
) -> Option<DynamicImage> {
    let capture_buffer = match monitor.capture_image() {
        Ok(image) => image,
        Err(e) => {
            log::error!("[capture_target_monitor] Failed to capture monitor: {:?}", e);
            return None;
        }
    };

    // Convert to DynamicImage
    let mut capture_image = DynamicImage::ImageRgba8(capture_buffer);

    // 裁剪区域
    if let Some(crop_area) = crop_area {
        capture_image = capture_image.crop(
            crop_area.min_x() as u32,
            crop_area.min_y() as u32,
            crop_area.width(),
            crop_area.height(),
        );
    }

    // 转换颜色格式
    match color_format {
        ColorFormat::Rgb8 => Some(DynamicImage::ImageRgb8(capture_image.into_rgb8())),
        ColorFormat::Rgba8 => Some(capture_image),
    }
}
