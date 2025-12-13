use aumate_core_shared::{DomainError, Point, Rectangle};

use super::models::Monitor;

/// 截图领域服务
///
/// 协调多个聚合和值对象的复杂操作
pub struct ScreenshotService;

impl ScreenshotService {
    /// 计算所有监视器的边界框
    ///
    /// # 业务规则
    /// - 边界框应包含所有监视器的区域
    /// - 如果没有监视器，返回 None
    pub fn calculate_monitors_bounding_box(monitors: &[Monitor]) -> Option<Rectangle> {
        if monitors.is_empty() {
            return None;
        }
        
        let first = monitors[0].rect();
        let bounding = monitors
            .iter()
            .skip(1)
            .fold(*first, |acc, m| acc.union(m.rect()));
        
        Some(bounding)
    }
    
    /// 查找包含指定点的监视器
    pub fn find_monitor_at_point<'a>(monitors: &'a [Monitor], point: &Point) -> Option<&'a Monitor> {
        monitors.iter().find(|m| m.rect().contains_point(point))
    }
    
    /// 获取主监视器
    pub fn find_primary_monitor(monitors: &[Monitor]) -> Option<&Monitor> {
        monitors.iter().find(|m| m.is_primary())
    }
    
    /// 验证捕获区域是否有效
    ///
    /// # 业务规则
    /// - 捕获区域必须与至少一个监视器相交
    /// - 如果没有监视器，返回错误
    pub fn validate_capture_region(region: &Rectangle, monitors: &[Monitor]) -> Result<(), DomainError> {
        if monitors.is_empty() {
            return Err(DomainError::NoMonitorsAvailable);
        }
        
        let has_intersection = monitors.iter().any(|m| m.rect().intersects(region));
        
        if !has_intersection {
            return Err(DomainError::RegionOutOfBounds);
        }
        
        Ok(())
    }
    
    /// 计算区域在监视器空间中的相对位置
    pub fn calculate_relative_region(region: &Rectangle, monitor: &Monitor) -> Rectangle {
        let monitor_rect = monitor.rect();
        Rectangle::from_xywh(
            region.min_x() - monitor_rect.min_x(),
            region.min_y() - monitor_rect.min_y(),
            region.width(),
            region.height(),
        )
        .unwrap_or(*region) // Fallback to original if calculation fails
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use aumate_core_shared::MonitorId;
    
    fn create_test_monitor(id: u32, x: i32, y: i32, width: u32, height: u32, is_primary: bool) -> Monitor {
        let rect = Rectangle::from_xywh(x, y, width, height).unwrap();
        Monitor::new(
            MonitorId::new(id),
            format!("Monitor {}", id),
            rect,
            1.0,
            is_primary,
        )
    }
    
    #[test]
    fn test_calculate_monitors_bounding_box() {
        let monitors = vec![
            create_test_monitor(0, 0, 0, 1920, 1080, true),
            create_test_monitor(1, 1920, 0, 1920, 1080, false),
        ];
        
        let bounding = ScreenshotService::calculate_monitors_bounding_box(&monitors);
        assert!(bounding.is_some());
        
        let bounding = bounding.unwrap();
        assert_eq!(bounding.width(), 3840);
        assert_eq!(bounding.height(), 1080);
    }
    
    #[test]
    fn test_calculate_monitors_bounding_box_empty() {
        let monitors = vec![];
        let bounding = ScreenshotService::calculate_monitors_bounding_box(&monitors);
        assert!(bounding.is_none());
    }
    
    #[test]
    fn test_find_monitor_at_point() {
        let monitors = vec![
            create_test_monitor(0, 0, 0, 1920, 1080, true),
            create_test_monitor(1, 1920, 0, 1920, 1080, false),
        ];
        
        let point = Point::new(100, 100);
        let monitor = ScreenshotService::find_monitor_at_point(&monitors, &point);
        assert!(monitor.is_some());
        assert_eq!(monitor.unwrap().id().value(), 0);
        
        let point = Point::new(2000, 100);
        let monitor = ScreenshotService::find_monitor_at_point(&monitors, &point);
        assert!(monitor.is_some());
        assert_eq!(monitor.unwrap().id().value(), 1);
        
        let point = Point::new(-100, -100);
        let monitor = ScreenshotService::find_monitor_at_point(&monitors, &point);
        assert!(monitor.is_none());
    }
    
    #[test]
    fn test_find_primary_monitor() {
        let monitors = vec![
            create_test_monitor(0, 0, 0, 1920, 1080, false),
            create_test_monitor(1, 1920, 0, 1920, 1080, true),
        ];
        
        let primary = ScreenshotService::find_primary_monitor(&monitors);
        assert!(primary.is_some());
        assert_eq!(primary.unwrap().id().value(), 1);
    }
    
    #[test]
    fn test_validate_capture_region_valid() {
        let monitors = vec![
            create_test_monitor(0, 0, 0, 1920, 1080, true),
        ];
        
        let region = Rectangle::from_xywh(100, 100, 800, 600).unwrap();
        let result = ScreenshotService::validate_capture_region(&region, &monitors);
        assert!(result.is_ok());
    }
    
    #[test]
    fn test_validate_capture_region_out_of_bounds() {
        let monitors = vec![
            create_test_monitor(0, 0, 0, 1920, 1080, true),
        ];
        
        let region = Rectangle::from_xywh(3000, 3000, 800, 600).unwrap();
        let result = ScreenshotService::validate_capture_region(&region, &monitors);
        assert!(result.is_err());
    }
    
    #[test]
    fn test_validate_capture_region_no_monitors() {
        let monitors = vec![];
        let region = Rectangle::from_xywh(100, 100, 800, 600).unwrap();
        let result = ScreenshotService::validate_capture_region(&region, &monitors);
        assert!(result.is_err());
    }
}



