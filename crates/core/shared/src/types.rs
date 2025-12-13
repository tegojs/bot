use serde::{Deserialize, Serialize};

/// 时间戳
pub type Timestamp = std::time::SystemTime;

/// 点坐标
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

impl Point {
    pub fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }

    pub fn distance_to(&self, other: &Point) -> f64 {
        let dx = (self.x - other.x) as f64;
        let dy = (self.y - other.y) as f64;
        (dx * dx + dy * dy).sqrt()
    }
}

/// 矩形区域
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Rectangle {
    min_x: i32,
    min_y: i32,
    max_x: i32,
    max_y: i32,
}

impl Rectangle {
    pub fn new(min_x: i32, min_y: i32, max_x: i32, max_y: i32) -> Result<Self, crate::DomainError> {
        if max_x <= min_x || max_y <= min_y {
            return Err(crate::DomainError::InvalidRectangle(format!(
                "width and height must be positive: ({}, {}, {}, {})",
                min_x, min_y, max_x, max_y
            )));
        }

        Ok(Self { min_x, min_y, max_x, max_y })
    }

    pub fn from_xywh(x: i32, y: i32, width: u32, height: u32) -> Result<Self, crate::DomainError> {
        Self::new(x, y, x + width as i32, y + height as i32)
    }

    pub fn min_x(&self) -> i32 {
        self.min_x
    }

    pub fn min_y(&self) -> i32 {
        self.min_y
    }

    pub fn max_x(&self) -> i32 {
        self.max_x
    }

    pub fn max_y(&self) -> i32 {
        self.max_y
    }

    pub fn width(&self) -> u32 {
        (self.max_x - self.min_x) as u32
    }

    pub fn height(&self) -> u32 {
        (self.max_y - self.min_y) as u32
    }

    pub fn contains_point(&self, point: &Point) -> bool {
        point.x >= self.min_x
            && point.x < self.max_x
            && point.y >= self.min_y
            && point.y < self.max_y
    }

    pub fn intersects(&self, other: &Rectangle) -> bool {
        self.min_x < other.max_x
            && self.max_x > other.min_x
            && self.min_y < other.max_y
            && self.max_y > other.min_y
    }

    pub fn union(&self, other: &Rectangle) -> Rectangle {
        Rectangle {
            min_x: self.min_x.min(other.min_x),
            min_y: self.min_y.min(other.min_y),
            max_x: self.max_x.max(other.max_x),
            max_y: self.max_y.max(other.max_y),
        }
    }

    /// 兼容 ElementRect - 从边界创建矩形（不做验证，用于已知有效的边界）
    pub fn from_bounds(min_x: i32, min_y: i32, max_x: i32, max_y: i32) -> Self {
        Self { min_x, min_y, max_x, max_y }
    }

    /// 兼容 ElementRect - 裁剪矩形到另一个矩形的边界内
    pub fn clip_rect(&self, other: &Rectangle) -> Rectangle {
        let new_min_x = self.min_x.max(other.min_x);
        let new_min_y = self.min_y.max(other.min_y);
        let new_max_x = self.max_x.min(other.max_x);
        let new_max_y = self.max_y.min(other.max_y);

        Rectangle::from_bounds(new_min_x, new_min_y, new_max_x, new_max_y)
    }

    /// 兼容 ElementRect - 检查是否等于给定边界
    pub fn equals(&self, min_x: i32, min_y: i32, max_x: i32, max_y: i32) -> bool {
        self.min_x == min_x && self.min_y == min_y && self.max_x == max_x && self.max_y == max_y
    }
}

/// 平台类型
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Platform {
    Windows,
    MacOS,
    Linux,
}

impl Platform {
    pub fn current() -> Self {
        #[cfg(target_os = "windows")]
        return Self::Windows;

        #[cfg(target_os = "macos")]
        return Self::MacOS;

        #[cfg(target_os = "linux")]
        return Self::Linux;
    }

    pub fn as_str(&self) -> &str {
        match self {
            Self::Windows => "windows",
            Self::MacOS => "macos",
            Self::Linux => "linux",
        }
    }
}

/// ID 类型

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct ScreenshotId(String);

impl ScreenshotId {
    pub fn generate() -> Self {
        Self(uuid::Uuid::new_v4().to_string())
    }

    pub fn from_string(s: String) -> Self {
        Self(s)
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct MonitorId(u32);

impl MonitorId {
    pub fn new(id: u32) -> Self {
        Self(id)
    }

    pub fn value(&self) -> u32 {
        self.0
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct WindowId(String);

impl WindowId {
    pub fn new(id: String) -> Self {
        Self(id)
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct PageId(String);

impl PageId {
    pub fn generate() -> Self {
        Self(uuid::Uuid::new_v4().to_string())
    }

    pub fn from_string(s: String) -> Self {
        Self(s)
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rectangle_valid() {
        let rect = Rectangle::new(0, 0, 100, 100);
        assert!(rect.is_ok());

        let rect = rect.unwrap();
        assert_eq!(rect.width(), 100);
        assert_eq!(rect.height(), 100);
    }

    #[test]
    fn test_rectangle_invalid() {
        let rect = Rectangle::new(0, 0, 0, 100);
        assert!(rect.is_err());

        let rect = Rectangle::new(0, 0, 100, 0);
        assert!(rect.is_err());
    }

    #[test]
    fn test_rectangle_contains_point() {
        let rect = Rectangle::new(10, 10, 100, 100).unwrap();

        assert!(rect.contains_point(&Point::new(50, 50)));
        assert!(!rect.contains_point(&Point::new(5, 5)));
        assert!(!rect.contains_point(&Point::new(105, 105)));
    }

    #[test]
    fn test_rectangle_intersects() {
        let rect1 = Rectangle::new(0, 0, 100, 100).unwrap();
        let rect2 = Rectangle::new(50, 50, 150, 150).unwrap();
        let rect3 = Rectangle::new(200, 200, 300, 300).unwrap();

        assert!(rect1.intersects(&rect2));
        assert!(!rect1.intersects(&rect3));
    }
}
