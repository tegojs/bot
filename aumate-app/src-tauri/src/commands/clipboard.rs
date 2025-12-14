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

    // 直接调用 ClipboardAdapter 的 write_image_rgba 方法
    state.clipboard.write_image_rgba(data, width as usize, height as usize).await.map_err(|e| {
        format!("Failed to write image to clipboard: {}", e)
    })
}

/// 从 PNG base64 写入图像到剪贴板 (优化版)
#[tauri::command]
pub async fn write_clipboard_image_png(
    state: State<'_, AppState>,
    png_base64: String,
) -> Result<(), String> {
    log::info!("API: write_clipboard_image_png called, base64 length={}", png_base64.len());

    use base64::{Engine as _, engine::general_purpose::STANDARD};
    use image::GenericImageView;

    // Decode base64 to PNG bytes
    let png_data = STANDARD.decode(&png_base64).map_err(|e| {
        format!("Failed to decode base64: {}", e)
    })?;

    // Decode PNG to get RGBA pixels
    let img = image::load_from_memory(&png_data).map_err(|e| {
        format!("Failed to decode PNG: {}", e)
    })?;

    let (width, height) = img.dimensions();
    let rgba = img.to_rgba8();
    let data = rgba.into_raw();

    log::info!("API: Decoded PNG to {}x{} RGBA image", width, height);

    // 直接调用 ClipboardAdapter 的 write_image_rgba 方法（绕过不支持图像的通用 write 方法）
    state.clipboard.write_image_rgba(data, width as usize, height as usize).await.map_err(|e| {
        format!("Failed to write image to clipboard: {}", e)
    })
}
