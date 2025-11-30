use anyhow::Context;
use arboard::ImageData;
use image::GenericImageView;

pub fn capture_and_save_to_clipboard(
    monitor: &xcap::Monitor,
    coords: ((u32, u32), (u32, u32)),
) -> anyhow::Result<()> {
    let ((min_x, min_y), (max_x, max_y)) = coords;
    let width = max_x.saturating_sub(min_x);
    let height = max_y.saturating_sub(min_y);

    if width == 0 || height == 0 {
        anyhow::bail!("Invalid selection size");
    }

    // Capture the full screen
    let img = monitor
        .capture_image()
        .context("Failed to capture screen")?;

    // Crop the selected region
    let cropped = img
        .view(min_x, min_y, width, height)
        .to_image();

    // Convert to RGBA bytes (already RGBA)
    let (img_width, img_height) = cropped.dimensions();
    let bytes = cropped.into_raw();

    // Save to clipboard
    let mut clipboard = arboard::Clipboard::new().context("Failed to access clipboard")?;
    let image_data = ImageData {
        width: img_width as usize,
        height: img_height as usize,
        bytes: std::borrow::Cow::Owned(bytes),
    };

    clipboard
        .set_image(image_data)
        .context("Failed to set clipboard image")?;

    Ok(())
}

