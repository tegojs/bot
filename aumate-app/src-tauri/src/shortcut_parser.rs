// 快捷键解析模块
// 将字符串格式的快捷键（如 "Ctrl+4"）转换为 Tauri Shortcut 对象

use tauri_plugin_global_shortcut::{Code, Modifiers, Shortcut};

/// 解析快捷键字符串为 Shortcut 对象
/// 
/// 支持的格式：
/// - "F3" - 单个功能键
/// - "Ctrl+4" - Ctrl 修饰键 + 数字键
/// - "Ctrl+Shift+A" - 多个修饰键 + 字母键
/// - "Ctrl+," - Ctrl + 逗号
pub fn parse_shortcut(shortcut_str: &str) -> Result<Shortcut, String> {
    let parts: Vec<&str> = shortcut_str.split('+').collect();
    
    if parts.is_empty() {
        return Err("Empty shortcut string".to_string());
    }

    let mut modifiers_flags = Modifiers::empty();
    let mut key_code: Option<Code> = None;

    for (i, part) in parts.iter().enumerate() {
        let part_lower = part.trim().to_lowercase();
        
        // 判断是修饰键还是主键
        match part_lower.as_str() {
            "ctrl" | "control" => modifiers_flags |= Modifiers::CONTROL,
            "shift" => modifiers_flags |= Modifiers::SHIFT,
            "alt" => modifiers_flags |= Modifiers::ALT,
            "meta" | "cmd" | "win" => modifiers_flags |= Modifiers::META,
            // 最后一个部分应该是主键
            _ if i == parts.len() - 1 => {
                key_code = Some(parse_key_code(part.trim())?);
            }
            _ => return Err(format!("Unknown modifier or misplaced key: {}", part)),
        }
    }

    let code = key_code.ok_or_else(|| "No key code found".to_string())?;
    
    // 如果没有修饰键，传 None；否则传 Some(modifiers)
    let modifiers = if modifiers_flags.is_empty() {
        None
    } else {
        Some(modifiers_flags)
    };

    Ok(Shortcut::new(modifiers, code))
}

/// 解析单个按键字符串为 Code
fn parse_key_code(key: &str) -> Result<Code, String> {
    let key_lower = key.to_lowercase();
    
    match key_lower.as_str() {
        // 功能键
        "f1" => Ok(Code::F1),
        "f2" => Ok(Code::F2),
        "f3" => Ok(Code::F3),
        "f4" => Ok(Code::F4),
        "f5" => Ok(Code::F5),
        "f6" => Ok(Code::F6),
        "f7" => Ok(Code::F7),
        "f8" => Ok(Code::F8),
        "f9" => Ok(Code::F9),
        "f10" => Ok(Code::F10),
        "f11" => Ok(Code::F11),
        "f12" => Ok(Code::F12),
        
        // 数字键
        "0" => Ok(Code::Digit0),
        "1" => Ok(Code::Digit1),
        "2" => Ok(Code::Digit2),
        "3" => Ok(Code::Digit3),
        "4" => Ok(Code::Digit4),
        "5" => Ok(Code::Digit5),
        "6" => Ok(Code::Digit6),
        "7" => Ok(Code::Digit7),
        "8" => Ok(Code::Digit8),
        "9" => Ok(Code::Digit9),
        
        // 字母键
        "a" => Ok(Code::KeyA),
        "b" => Ok(Code::KeyB),
        "c" => Ok(Code::KeyC),
        "d" => Ok(Code::KeyD),
        "e" => Ok(Code::KeyE),
        "f" => Ok(Code::KeyF),
        "g" => Ok(Code::KeyG),
        "h" => Ok(Code::KeyH),
        "i" => Ok(Code::KeyI),
        "j" => Ok(Code::KeyJ),
        "k" => Ok(Code::KeyK),
        "l" => Ok(Code::KeyL),
        "m" => Ok(Code::KeyM),
        "n" => Ok(Code::KeyN),
        "o" => Ok(Code::KeyO),
        "p" => Ok(Code::KeyP),
        "q" => Ok(Code::KeyQ),
        "r" => Ok(Code::KeyR),
        "s" => Ok(Code::KeyS),
        "t" => Ok(Code::KeyT),
        "u" => Ok(Code::KeyU),
        "v" => Ok(Code::KeyV),
        "w" => Ok(Code::KeyW),
        "x" => Ok(Code::KeyX),
        "y" => Ok(Code::KeyY),
        "z" => Ok(Code::KeyZ),
        
        // 特殊键
        "space" => Ok(Code::Space),
        "enter" | "return" => Ok(Code::Enter),
        "tab" => Ok(Code::Tab),
        "escape" | "esc" => Ok(Code::Escape),
        "backspace" => Ok(Code::Backspace),
        "delete" | "del" => Ok(Code::Delete),
        "insert" | "ins" => Ok(Code::Insert),
        "home" => Ok(Code::Home),
        "end" => Ok(Code::End),
        "pageup" | "pgup" => Ok(Code::PageUp),
        "pagedown" | "pgdn" => Ok(Code::PageDown),
        
        // 箭头键
        "arrowup" | "up" => Ok(Code::ArrowUp),
        "arrowdown" | "down" => Ok(Code::ArrowDown),
        "arrowleft" | "left" => Ok(Code::ArrowLeft),
        "arrowright" | "right" => Ok(Code::ArrowRight),
        
        // 标点符号
        "," | "comma" => Ok(Code::Comma),
        "." | "period" => Ok(Code::Period),
        "/" | "slash" => Ok(Code::Slash),
        ";" | "semicolon" => Ok(Code::Semicolon),
        "'" | "quote" => Ok(Code::Quote),
        "[" | "bracketleft" => Ok(Code::BracketLeft),
        "]" | "bracketright" => Ok(Code::BracketRight),
        "\\" | "backslash" => Ok(Code::Backslash),
        "-" | "minus" => Ok(Code::Minus),
        "=" | "equal" => Ok(Code::Equal),
        "`" | "backquote" => Ok(Code::Backquote),
        
        _ => Err(format!("Unknown key code: {}", key)),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_function_key() {
        let shortcut = parse_shortcut("F3").unwrap();
        assert_eq!(shortcut, Shortcut::new(None, Code::F3));
    }

    #[test]
    fn test_parse_ctrl_digit() {
        let shortcut = parse_shortcut("Ctrl+4").unwrap();
        assert_eq!(shortcut, Shortcut::new(Some(Modifiers::CONTROL), Code::Digit4));
    }

    #[test]
    fn test_parse_ctrl_shift() {
        let shortcut = parse_shortcut("Ctrl+Shift+A").unwrap();
        assert_eq!(
            shortcut,
            Shortcut::new(Some(Modifiers::CONTROL | Modifiers::SHIFT), Code::KeyA)
        );
    }
}

