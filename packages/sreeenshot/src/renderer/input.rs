/// 构建 Egui 输入事件
pub fn build_egui_input(
    width: u32,
    height: u32,
    mouse_pos: Option<(f32, f32)>,
    mouse_pressed: bool,
    last_mouse_pressed: &mut bool,
) -> egui::RawInput {
    let mut events = Vec::new();
    
    // 鼠标移动事件
    if let Some((x, y)) = mouse_pos {
        events.push(egui::Event::PointerMoved(egui::Pos2::new(x, y)));
    }
    
    // 鼠标按钮事件（检测点击）
    if let Some((x, y)) = mouse_pos {
        let pos = egui::Pos2::new(x, y);
        
        if mouse_pressed && !*last_mouse_pressed {
            // 按钮按下
            events.push(egui::Event::PointerButton {
                pos,
                button: egui::PointerButton::Primary,
                pressed: true,
                modifiers: egui::Modifiers::default(),
            });
        } else if !mouse_pressed && *last_mouse_pressed {
            // 按钮释放（点击完成）
            events.push(egui::Event::PointerButton {
                pos,
                button: egui::PointerButton::Primary,
                pressed: false,
                modifiers: egui::Modifiers::default(),
            });
        }
    }
    
    *last_mouse_pressed = mouse_pressed;
    
    egui::RawInput {
        screen_rect: Some(egui::Rect::from_min_size(
            egui::Pos2::ZERO,
            egui::Vec2::new(width as f32, height as f32),
        )),
        events,
        ..Default::default()
    }
}

