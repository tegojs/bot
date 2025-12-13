// 剪贴板相关 Tauri Commands
use crate::state::AppState;
use aumate_application::dto::clipboard::{
    ClipboardContentDTO, ClipboardImageResponse, ReadClipboardResponse,
};
use aumate_core_shared::ApiError;
use tauri::State;

/// 读取剪贴板内容
#[tauri::command]
pub async fn read_clipboard(state: State<'_, AppState>) -> Result<ReadClipboardResponse, String> {
    log::info!("API: read_clipboard called");

    state.read_clipboard.execute().await.map_err(|e| {
        let api_error: ApiError = e.into();
        api_error.to_string()
    })
}

/// 写入剪贴板内容
#[tauri::command]
pub async fn write_clipboard(
    state: State<'_, AppState>,
    content: ClipboardContentDTO,
) -> Result<(), String> {
    log::info!("API: write_clipboard called");

    state.write_clipboard.execute(content).await.map_err(|e| {
        let api_error: ApiError = e.into();
        api_error.to_string()
    })
}

/// 清空剪贴板
#[tauri::command]
pub async fn clear_clipboard(state: State<'_, AppState>) -> Result<(), String> {
    log::info!("API: clear_clipboard called");

    use aumate_core_traits::clipboard::ClipboardPort;

    state.clipboard.clear().await.map_err(|e| {
        let api_error: ApiError = e.into();
        api_error.to_string()
    })
}

/// 获取剪贴板可用类型
#[tauri::command]
pub async fn get_clipboard_types(state: State<'_, AppState>) -> Result<Vec<String>, String> {
    log::info!("API: get_clipboard_types called");

    use aumate_core_traits::clipboard::ClipboardPort;

    let types = state.clipboard.get_available_types().await.map_err(|e| {
        let api_error: ApiError = e.into();
        api_error.to_string()
    })?;

    let type_strings = types.into_iter().map(|t| format!("{:?}", t)).collect();

    Ok(type_strings)
}

/// 从剪贴板读取图像 (便捷API)
#[tauri::command]
pub async fn read_clipboard_image(
    state: State<'_, AppState>,
) -> Result<ClipboardImageResponse, String> {
    log::info!("API: read_clipboard_image called");

    state.read_clipboard_image.execute().await.map_err(|e| {
        let api_error: ApiError = e.into();
        api_error.to_string()
    })
}

/// 写入图像到剪贴板 (便捷API)
#[tauri::command]
pub async fn write_clipboard_image(
    state: State<'_, AppState>,
    data: Vec<u8>,
    width: u32,
    height: u32,
) -> Result<(), String> {
    log::info!("API: write_clipboard_image called, size={}x{}", width, height);

    state.write_clipboard_image.execute(data, width, height).await.map_err(|e| {
        let api_error: ApiError = e.into();
        api_error.to_string()
    })
}
