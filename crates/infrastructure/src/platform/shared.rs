/// 平台共享代码
///
/// **迁移**: 从 app-os/src/lib.rs
use std::{cmp::Ordering, hash::Hash};

/// 元素层级
#[derive(Debug, Clone, PartialEq, Eq, Hash, Copy, PartialOrd)]
pub struct ElementLevel {
    /// 遍历时，首先获得层级最高的元素
    /// 同级元素，index 越高，层级越低
    pub element_index: i32,
    /// 元素层级
    pub element_level: i32,
    /// 父元素索引
    pub parent_index: i32,
    /// 窗口索引
    pub window_index: i32,
}

impl ElementLevel {
    pub fn root() -> Self {
        Self { element_index: 0, element_level: 0, parent_index: i32::MAX, window_index: i32::MAX }
    }

    pub fn next_level(&mut self) {
        self.element_level += 1;
        let current_element_index = self.element_index;
        self.element_index = 0;
        self.parent_index = current_element_index;
    }

    pub fn next_element(&mut self) {
        self.element_index += 1;
    }
}

impl Ord for ElementLevel {
    fn cmp(&self, other: &Self) -> Ordering {
        // 先窗口索引排序，窗口索引小的优先级越高
        if self.window_index != other.window_index {
            return other.window_index.cmp(&self.window_index);
        }

        // 元素层级排序，层级高的优先级越高
        if self.element_level != other.element_level {
            return self.element_level.cmp(&other.element_level);
        }

        // 元素索引排序，索引小的优先级越高
        if self.element_index != other.element_index {
            return other.element_index.cmp(&self.element_index);
        }

        // 父元素索引排序，索引大的优先级越高
        other.parent_index.cmp(&self.parent_index)
    }
}

/// UI Automation 错误
#[derive(Debug)]
pub enum UIAutomationError {
    Capture(String),
}

impl std::fmt::Display for UIAutomationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Capture(e) => write!(f, "Capture error: {}", e),
        }
    }
}

impl std::error::Error for UIAutomationError {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_element_level_creation() {
        let level = ElementLevel::root();
        assert_eq!(level.element_level, 0);
        assert_eq!(level.parent_index, i32::MAX);
    }

    #[test]
    fn test_element_level_next() {
        let mut level = ElementLevel::root();
        level.next_element();
        assert_eq!(level.element_index, 1);

        level.next_level();
        assert_eq!(level.element_level, 1);
    }
}
