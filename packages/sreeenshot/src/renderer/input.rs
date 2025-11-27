use winit::event::{ElementState, KeyEvent, Ime};
use winit::keyboard::{Key, NamedKey};

/// 将 winit Key 转换为 egui Key
fn winit_key_to_egui_key(key: &Key) -> Option<egui::Key> {
    match key {
        Key::Named(NamedKey::ArrowDown) => Some(egui::Key::ArrowDown),
        Key::Named(NamedKey::ArrowLeft) => Some(egui::Key::ArrowLeft),
        Key::Named(NamedKey::ArrowRight) => Some(egui::Key::ArrowRight),
        Key::Named(NamedKey::ArrowUp) => Some(egui::Key::ArrowUp),
        Key::Named(NamedKey::Escape) => Some(egui::Key::Escape),
        Key::Named(NamedKey::Tab) => Some(egui::Key::Tab),
        Key::Named(NamedKey::Backspace) => Some(egui::Key::Backspace),
        Key::Named(NamedKey::Enter) => Some(egui::Key::Enter),
        Key::Named(NamedKey::Space) => Some(egui::Key::Space),
        Key::Named(NamedKey::Insert) => Some(egui::Key::Insert),
        Key::Named(NamedKey::Delete) => Some(egui::Key::Delete),
        Key::Named(NamedKey::Home) => Some(egui::Key::Home),
        Key::Named(NamedKey::End) => Some(egui::Key::End),
        Key::Named(NamedKey::PageUp) => Some(egui::Key::PageUp),
        Key::Named(NamedKey::PageDown) => Some(egui::Key::PageDown),
        Key::Character(ch) => {
            // 处理单个字符
            if ch.len() == 1 {
                let c = ch.chars().next().unwrap();
                if c.is_ascii() {
                    egui::Key::from_name(ch)
                } else {
                    None
                }
            } else {
                None
            }
        }
        _ => None,
    }
}

/// 构建 Egui 输入事件
pub fn build_egui_input(
    width: u32,
    height: u32,
    mouse_pos: Option<(f32, f32)>,
    mouse_pressed: bool,
    last_mouse_pressed: &mut bool,
    keyboard_events: &[KeyEvent],
    ime_events: &[Ime],
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
    
    // 处理 IME 事件（输入法事件）
    // 注意：egui 的 Event::Ime 直接接受 ImeEvent，但我们需要从 winit 的 Ime 转换
    // 对于 Commit 事件，我们直接发送 Text 事件，因为 egui 的 TextEdit 会处理它
    for ime_event in ime_events {
        match ime_event {
            Ime::Commit(text, ..) => {
                // 输入法提交的文本 - 直接作为文本输入发送
                if !text.is_empty() {
                    events.push(egui::Event::Text(text.to_string()));
                }
            }
            Ime::Preedit(_, ..) => {
                // 输入法预编辑文本（如中文输入时的拼音）
                // egui 的 TextEdit 会自动处理，我们不需要特别处理
            }
            Ime::Enabled | Ime::Disabled => {
                // IME 启用/禁用 - egui 会自动处理，不需要特别处理
            }
        }
    }
    
    // 处理键盘事件
    for key_event in keyboard_events {
        let pressed = matches!(key_event.state, ElementState::Pressed);
        
        // 处理修饰键
        let modifiers = egui::Modifiers::default();
        // 注意：winit 的 KeyEvent 可能不包含修饰键信息，这里需要从其他地方获取
        // 暂时使用默认值
        
        if let Some(egui_key) = winit_key_to_egui_key(&key_event.logical_key) {
            // physical_key 字段用于标识物理按键位置，对于文本输入来说不是必需的，使用 None
            events.push(egui::Event::Key {
                key: egui_key,
                pressed,
                repeat: false,
                modifiers,
                physical_key: None,
            });
        } else if let Key::Character(ch) = &key_event.logical_key {
            // 处理文本输入 - 对于 Character 类型的键，直接发送文本事件
            // 注意：只有在按下时才发送，并且不发送特殊键（这些键应该被上面的 winit_key_to_egui_key 处理）
            if pressed {
                // 发送所有字符，不仅仅是 ASCII
                events.push(egui::Event::Text(ch.to_string()));
            }
        }
    }
    
    egui::RawInput {
        screen_rect: Some(egui::Rect::from_min_size(
            egui::Pos2::ZERO,
            egui::Vec2::new(width as f32, height as f32),
        )),
        events,
        ..Default::default()
    }
}

