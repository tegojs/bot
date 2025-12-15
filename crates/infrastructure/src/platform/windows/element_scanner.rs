#![cfg(target_os = "windows")]

use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use aumate_core_shared::Rectangle;
use aumate_core_traits::{ElementType, ScannableElement};
use uiautomation::UIAutomation;
use uiautomation::types::{TreeScope, UIProperty};
use uiautomation::controls::ControlType;
use windows::Win32::Foundation::{HWND, LPARAM, RECT};
use windows::Win32::UI::WindowsAndMessaging::{
    FindWindowA, FindWindowExA, EnumChildWindows, GetWindowRect, 
    GetWindowTextW, IsWindowVisible, GetSystemMetrics, 
    SYSTEM_METRICS_INDEX,
};
use windows::core::s;
use once_cell::sync::Lazy;

/// 元素缓存，存储已扫描的元素供后续操作使用
static ELEMENT_CACHE: Lazy<Arc<Mutex<HashMap<String, CachedElement>>>> = 
    Lazy::new(|| Arc::new(Mutex::new(HashMap::new())));

/// 缓存的元素信息
#[derive(Clone)]
struct CachedElement {
    element_type: ElementType,
    hwnd: Option<isize>,
    bounds: Rectangle,
}

/// 获取屏幕尺寸
fn get_screen_bounds() -> Rectangle {
    unsafe {
        let width = GetSystemMetrics(SYSTEM_METRICS_INDEX(0)) as u32; // SM_CXSCREEN = 0
        let height = GetSystemMetrics(SYSTEM_METRICS_INDEX(1)) as u32; // SM_CYSCREEN = 1
        Rectangle::from_xywh(0, 0, width, height).unwrap()
    }
}

/// 检查元素是否在屏幕可见区域内
/// 要求元素至少有 50% 在屏幕内
fn is_element_visible_on_screen(element_bounds: &Rectangle, screen_bounds: &Rectangle) -> bool {
    let intersection_width = std::cmp::min(element_bounds.max_x(), screen_bounds.max_x())
        - std::cmp::max(element_bounds.min_x(), screen_bounds.min_x());
    let intersection_height = std::cmp::min(element_bounds.max_y(), screen_bounds.max_y())
        - std::cmp::max(element_bounds.min_y(), screen_bounds.min_y());

    if intersection_width <= 0 || intersection_height <= 0 {
        return false;
    }

    let intersection_area = (intersection_width * intersection_height) as f32;
    let element_area = (element_bounds.width() * element_bounds.height()) as f32;

    // 元素至少 50% 在屏幕内才认为可见
    intersection_area / element_area >= 0.5
}

/// 扫描屏幕上的可交互元素
pub async fn scan_elements() -> Result<Vec<ScannableElement>, String> {
    let mut elements = Vec::new();
    let mut cache_map = HashMap::new();

    // 1. 扫描输入框（同步执行，不跨越 await 边界）
    log::info!("[ElementScanner] Scanning input fields...");
    let input_result = {
        let automation = UIAutomation::new()
            .map_err(|e| format!("Failed to create UIAutomation: {}", e))?;
        scan_input_fields_sync(&automation)
    };
    
    match input_result {
        Ok(mut input_elements) => {
            log::info!("[ElementScanner] Found {} input fields", input_elements.len());
            for elem in &input_elements {
                cache_map.insert(
                    elem.id.clone(),
                    CachedElement {
                        element_type: ElementType::InputField,
                        hwnd: None,
                        bounds: elem.bounds.clone(),
                    },
                );
            }
            elements.append(&mut input_elements);
        }
        Err(e) => {
            log::warn!("[ElementScanner] Failed to scan input fields: {}", e);
        }
    }

    // 2. 扫描任务栏图标（同步执行）
    log::info!("[ElementScanner] Scanning taskbar icons...");
    let taskbar_result = {
        let automation = UIAutomation::new()
            .map_err(|e| format!("Failed to create UIAutomation: {}", e))?;
        scan_taskbar_icons_sync(&automation)
    };
    
    match taskbar_result {
        Ok(mut taskbar_elements) => {
            log::info!("[ElementScanner] Found {} taskbar icons", taskbar_elements.len());
            for elem in &taskbar_elements {
                cache_map.insert(
                    elem.id.clone(),
                    CachedElement {
                        element_type: ElementType::TaskbarIcon,
                        hwnd: None,
                        bounds: elem.bounds.clone(),
                    },
                );
            }
            elements.append(&mut taskbar_elements);
        }
        Err(e) => {
            log::warn!("[ElementScanner] Failed to scan taskbar icons: {}", e);
        }
    }

    // 3. 按照从上到下、从左到右排序
    elements.sort_by(|a, b| {
        let a_top = a.bounds.min_y();
        let b_top = b.bounds.min_y();
        let a_left = a.bounds.min_x();
        let b_left = b.bounds.min_x();

        // 先按 Y 坐标排序，如果 Y 坐标相近（差距小于 50 像素），则按 X 坐标排序
        if (a_top - b_top).abs() < 50 {
            a_left.cmp(&b_left)
        } else {
            a_top.cmp(&b_top)
        }
    });

    // 4. 限制为最多 26 个元素，并分配字母标签
    elements.truncate(26);
    for (i, elem) in elements.iter_mut().enumerate() {
        elem.label = (b'A' + i as u8) as char;
    }

    // 5. 保存到缓存
    *ELEMENT_CACHE.lock().unwrap() = cache_map;

    log::info!("[ElementScanner] Total elements scanned: {}", elements.len());
    Ok(elements)
}

/// 扫描输入框（同步函数）
fn scan_input_fields_sync(automation: &UIAutomation) -> Result<Vec<ScannableElement>, String> {
    let screen_bounds = get_screen_bounds();
    let mut elements = Vec::new();

    log::info!("[ElementScanner] Screen bounds: {:?}", screen_bounds);

    // 获取桌面根元素
    let root = automation
        .get_root_element()
        .map_err(|e| format!("Failed to get root element: {}", e))?;

    // 创建条件：查找所有 Edit 控件（输入框）
    // UIAutomation 使用 i32 作为 ControlType 的值
    let condition = automation
        .create_property_condition(UIProperty::ControlType, (ControlType::Edit as i32).into(), None)
        .map_err(|e| format!("Failed to create condition: {}", e))?;

    // 查找所有匹配的元素
    let found_elements = root
        .find_all(TreeScope::Descendants, &condition)
        .map_err(|e| format!("Failed to find elements: {}", e))?;

    log::info!("[ElementScanner] Found {} raw input elements", found_elements.len());

    for i in 0..found_elements.len() {
        if let Some(element) = found_elements.get(i) {
            // 获取边界
            if let Ok(rect) = element.get_bounding_rectangle() {
                let left = rect.get_left();
                let top = rect.get_top();
                let width = (rect.get_right() - left) as u32;
                let height = (rect.get_bottom() - top) as u32;

                // 过滤太小的元素（可能是隐藏的或无效的）
                if width < 10 || height < 10 {
                    log::debug!("[ElementScanner] Skipping too small element: {}x{}", width, height);
                    continue;
                }

                // 过滤异常大的元素（可能是整个屏幕或容器）
                if width > screen_bounds.width() || height > screen_bounds.height() {
                    log::debug!("[ElementScanner] Skipping too large element: {}x{}", width, height);
                    continue;
                }

                if let Ok(bounds) = Rectangle::from_xywh(left, top, width, height) {
                    // 检查元素是否在屏幕可见区域内
                    if !is_element_visible_on_screen(&bounds, &screen_bounds) {
                        log::debug!(
                            "[ElementScanner] Skipping element outside visible screen: {:?}",
                            bounds
                        );
                        continue;
                    }

                    // 尝试获取元素名称
                    let title = element.get_name().ok();

                    let id = format!("input_{}_{}", left, top);

                    log::debug!(
                        "[ElementScanner] Adding visible input element: {} at ({}, {}) {}x{}",
                        id,
                        left,
                        top,
                        width,
                        height
                    );

                    elements.push(ScannableElement {
                        id,
                        element_type: ElementType::InputField,
                        bounds,
                        title,
                        label: 'A', // 临时值，稍后会重新分配
                    });
                }
            }
        }
    }

    log::info!("[ElementScanner] Filtered to {} visible input elements", elements.len());
    Ok(elements)
}

/// 扫描任务栏图标（同步函数）
fn scan_taskbar_icons_sync(_automation: &UIAutomation) -> Result<Vec<ScannableElement>, String> {
    // TODO: 任务栏检测需要更复杂的实现，暂时返回空列表
    // Windows API 回调函数类型在 windows crate 中有兼容性问题
    // 未来可以使用其他方法实现任务栏图标检测
    log::warn!("[ElementScanner] Taskbar icon scanning temporarily disabled due to API compatibility issues");
    Ok(Vec::new())
}

/// 点击指定元素
pub async fn click_element(element_id: &str) -> Result<(), String> {
    let cache = ELEMENT_CACHE.lock().unwrap();
    
    let cached_elem = cache
        .get(element_id)
        .ok_or_else(|| format!("Element not found: {}", element_id))?;

    // 计算元素中心点
    let center_x = cached_elem.bounds.min_x() + (cached_elem.bounds.width() / 2) as i32;
    let center_y = cached_elem.bounds.min_y() + (cached_elem.bounds.height() / 2) as i32;

    log::info!(
        "[ElementScanner] Clicking element {} at ({}, {})",
        element_id,
        center_x,
        center_y
    );

    // 使用 Windows API 模拟鼠标点击
    unsafe {
        use windows::Win32::UI::Input::KeyboardAndMouse::{
            SendInput, INPUT, INPUT_0, INPUT_MOUSE, MOUSEEVENTF_ABSOLUTE,
            MOUSEEVENTF_LEFTDOWN, MOUSEEVENTF_LEFTUP, MOUSEEVENTF_MOVE, MOUSEINPUT,
        };
        use windows::Win32::UI::WindowsAndMessaging::{
            GetSystemMetrics, SM_CXSCREEN, SM_CYSCREEN,
        };

        let screen_width = GetSystemMetrics(SM_CXSCREEN);
        let screen_height = GetSystemMetrics(SM_CYSCREEN);

        // 将屏幕坐标转换为绝对坐标（0-65535）
        let abs_x = (center_x * 65535) / screen_width;
        let abs_y = (center_y * 65535) / screen_height;

        let mut inputs = [
            // 移动鼠标
            INPUT {
                r#type: INPUT_MOUSE,
                Anonymous: INPUT_0 {
                    mi: MOUSEINPUT {
                        dx: abs_x,
                        dy: abs_y,
                        mouseData: 0,
                        dwFlags: MOUSEEVENTF_ABSOLUTE | MOUSEEVENTF_MOVE,
                        time: 0,
                        dwExtraInfo: 0,
                    },
                },
            },
            // 按下左键
            INPUT {
                r#type: INPUT_MOUSE,
                Anonymous: INPUT_0 {
                    mi: MOUSEINPUT {
                        dx: abs_x,
                        dy: abs_y,
                        mouseData: 0,
                        dwFlags: MOUSEEVENTF_ABSOLUTE | MOUSEEVENTF_LEFTDOWN,
                        time: 0,
                        dwExtraInfo: 0,
                    },
                },
            },
            // 释放左键
            INPUT {
                r#type: INPUT_MOUSE,
                Anonymous: INPUT_0 {
                    mi: MOUSEINPUT {
                        dx: abs_x,
                        dy: abs_y,
                        mouseData: 0,
                        dwFlags: MOUSEEVENTF_ABSOLUTE | MOUSEEVENTF_LEFTUP,
                        time: 0,
                        dwExtraInfo: 0,
                    },
                },
            },
        ];

        let sent = SendInput(&mut inputs, std::mem::size_of::<INPUT>() as i32);
        if sent != 3 {
            return Err(format!("Failed to send input events, sent: {}", sent));
        }
    }

    Ok(())
}

/// 聚焦到指定元素
pub async fn focus_element(element_id: &str) -> Result<(), String> {
    // 从缓存中获取元素信息（立即释放锁）
    let (element_type, bounds) = {
        let cache = ELEMENT_CACHE.lock().unwrap();
        let cached_elem = cache
            .get(element_id)
            .ok_or_else(|| format!("Element not found: {}", element_id))?;
        (cached_elem.element_type.clone(), cached_elem.bounds.clone())
    }; // 锁在这里被释放

    log::info!("[ElementScanner] Focusing element {}", element_id);

    // 对于输入框，使用 UIAutomation 的 SetFocus 方法
    if element_type == ElementType::InputField {
        // 根据坐标计算中心点
        let center_x = bounds.min_x() + (bounds.width() / 2) as i32;
        let center_y = bounds.min_y() + (bounds.height() / 2) as i32;

        // 尝试使用 UIAutomation 聚焦
        let focus_result = {
            let automation = UIAutomation::new()
                .map_err(|e| format!("Failed to create UIAutomation: {}", e))?;

            if let Ok(element) = automation.element_from_point(uiautomation::types::Point::new(center_x, center_y)) {
                element.set_focus().map_err(|e| format!("Failed to set focus: {}", e))
            } else {
                Err("Element not found at position".to_string())
            }
        }; // UIAutomation 在这里被释放

        // 如果 UIAutomation 失败，回退到点击
        if focus_result.is_err() {
            return click_element(element_id).await;
        }
    } else {
        // 对于任务栏图标等其他类型，直接点击
        return click_element(element_id).await;
    }

    Ok(())
}

