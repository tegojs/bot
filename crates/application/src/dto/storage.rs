// 存储相关 DTOs
use serde::{Deserialize, Serialize};

/// 文件操作请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileOperationRequest {
    /// 操作类型 ("read", "write", "delete")
    pub operation: String,
    /// 文件路径
    pub path: String,
    /// 数据（用于写入）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<Vec<u8>>,
}

/// 文件操作响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileOperationResponse {
    /// 是否成功
    pub success: bool,
    /// 消息
    pub message: Option<String>,
    /// 数据（用于读取）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<Vec<u8>>,
}
