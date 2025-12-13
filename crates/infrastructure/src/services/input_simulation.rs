/// 输入模拟服务
///
/// **迁移**: 从 app-shared/src/lib.rs (EnigoManager)
///
/// 提供鼠标和键盘输入模拟功能
use enigo::{Enigo, Settings};

pub struct EnigoManager {
    pub enigo: Option<Enigo>,
}

impl EnigoManager {
    pub fn new() -> Self {
        Self { enigo: None }
    }

    pub fn get_enigo(&mut self) -> Result<&mut Enigo, String> {
        if self.enigo.is_some() {
            return Ok(self.enigo.as_mut().unwrap());
        }

        let enigo = match Enigo::new(&Settings::default()) {
            Ok(enigo) => enigo,
            Err(e) => {
                return Err(format!("[EnigoManager] Could not create enigo: {}", e));
            }
        };

        self.enigo = Some(enigo);
        Ok(self.enigo.as_mut().unwrap())
    }
}

impl Default for EnigoManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_enigo_manager_creation() {
        let _manager = EnigoManager::new();
        // 创建成功
    }

    #[test]
    fn test_get_enigo() {
        let mut manager = EnigoManager::new();
        // 第一次调用应该创建 enigo 实例
        let result = manager.get_enigo();
        assert!(result.is_ok());

        // 第二次调用应该复用实例
        let result2 = manager.get_enigo();
        assert!(result2.is_ok());
    }
}
