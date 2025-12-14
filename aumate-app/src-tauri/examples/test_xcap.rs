// 测试 xcap 库获取窗口信息
// 运行: cargo run --example test_xcap

fn main() {
    println!("=== Testing xcap Window::all() ===\n");

    match xcap::Window::all() {
        Ok(windows) => {
            println!("Found {} windows:", windows.len());
            
            for (i, window) in windows.iter().enumerate() {
                let id = window.id().unwrap_or(0);
                let title = window.title().unwrap_or_default();
                let app_name = window.app_name().unwrap_or_default();
                let x = window.x().unwrap_or(0);
                let y = window.y().unwrap_or(0);
                let width = window.width().unwrap_or(0);
                let height = window.height().unwrap_or(0);
                let is_minimized = window.is_minimized().unwrap_or(true);
                let current_monitor = window.current_monitor();
                
                println!("\n[{}] Window ID: {}", i, id);
                println!("  Title: {}", title);
                println!("  App: {}", app_name);
                println!("  Position: ({}, {})", x, y);
                println!("  Size: {}x{}", width, height);
                println!("  Minimized: {}", is_minimized);
                println!("  Monitor: {:?}", current_monitor.map(|m| m.name().unwrap_or_default()));
                
                // 检查是否应该被过滤
                let mut should_filter = false;
                let mut filter_reason = String::new();
                
                if is_minimized {
                    should_filter = true;
                    filter_reason.push_str("minimized ");
                }
                
                #[cfg(target_os = "macos")]
                {
                    if title == "Notification Center" {
                        should_filter = true;
                        filter_reason.push_str("notification-center ");
                    }
                    if title == "Dock" {
                        should_filter = true;
                        filter_reason.push_str("dock ");
                    }
                    if title.starts_with("Item-") {
                        should_filter = true;
                        filter_reason.push_str("item- ");
                    }
                    if y == 0 {
                        should_filter = true;
                        filter_reason.push_str("y==0 ");
                    }
                    if title.is_empty() {
                        should_filter = true;
                        filter_reason.push_str("no-title ");
                    }
                }
                
                if should_filter {
                    println!("  ⚠️ Would be FILTERED: {}", filter_reason);
                } else {
                    println!("  ✅ Would be INCLUDED");
                }
            }
        }
        Err(e) => {
            eprintln!("Error getting windows: {}", e);
        }
    }
    
    println!("\n=== Testing xcap Monitor::all() ===\n");
    
    match xcap::Monitor::all() {
        Ok(monitors) => {
            println!("Found {} monitors:", monitors.len());
            
            for (i, monitor) in monitors.iter().enumerate() {
                let name = monitor.name().unwrap_or_default();
                let x = monitor.x().unwrap_or(0);
                let y = monitor.y().unwrap_or(0);
                let width = monitor.width().unwrap_or(0);
                let height = monitor.height().unwrap_or(0);
                let is_primary = monitor.is_primary().unwrap_or(false);
                
                println!("\n[{}] Monitor: {}", i, name);
                println!("  Position: ({}, {})", x, y);
                println!("  Size: {}x{}", width, height);
                println!("  Primary: {}", is_primary);
            }
        }
        Err(e) => {
            eprintln!("Error getting monitors: {}", e);
        }
    }
}

