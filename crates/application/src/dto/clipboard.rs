/// 剪贴板相关 DTO
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", content = "content")]
pub enum ClipboardContentDTO {
    #[serde(rename = "text")]
    Text(String),
    #[serde(rename = "image")]
    Image { data: Vec<u8>, width: u32, height: u32, format: String },
    #[serde(rename = "files")]
    Files(Vec<String>),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReadClipboardResponse {
    pub content: ClipboardContentDTO,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClipboardImageResponse {
    pub data: Vec<u8>,
    pub width: u32,
    pub height: u32,
    pub format: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClipboardTypesResponse {
    pub types: Vec<String>,
}
