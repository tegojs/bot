// Window Layout Use Cases - Application Layer
// Business logic for window resizing and centering

use aumate_core_shared::WindowId;
use aumate_core_traits::{WindowLayout, WindowLayoutPort};
use std::sync::Arc;

/// Use Case: Resize and center window immediately (no animation)
pub struct ResizeAndCenterUseCase {
    layout_port: Arc<dyn WindowLayoutPort + Send + Sync>,
}

impl ResizeAndCenterUseCase {
    pub fn new(layout_port: Arc<dyn WindowLayoutPort + Send + Sync>) -> Self {
        Self { layout_port }
    }

    pub async fn execute(
        &self,
        window_id: WindowId,
        target_width: f64,
        target_height: f64,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        log::info!(
            "UseCase: ResizeAndCenter - window={:?}, size={}x{}",
            window_id,
            target_width,
            target_height
        );

        // Get monitor info
        let monitor = self.layout_port.get_monitor_info(&window_id).await?;

        // Convert monitor dimensions to logical pixels
        let scale_factor = monitor.scale_factor;
        let screen_width = monitor.width as f64 / scale_factor;
        let screen_height = monitor.height as f64 / scale_factor;
        let monitor_x = monitor.position_x as f64 / scale_factor;
        let monitor_y = monitor.position_y as f64 / scale_factor;

        // Calculate centered position
        let target_x = monitor_x + (screen_width - target_width) / 2.0;
        let target_y = monitor_y + (screen_height - target_height) / 2.0;

        log::debug!(
            "Centering: screen={}x{}, target_pos=({}, {})",
            screen_width,
            screen_height,
            target_x,
            target_y
        );

        // Set new layout
        let layout = WindowLayout {
            width: target_width,
            height: target_height,
            x: target_x,
            y: target_y,
        };

        self.layout_port.set_window_layout(&window_id, layout).await?;

        log::info!("Window centered successfully");
        Ok(())
    }
}

/// Use Case: Animate window resize and center
pub struct AnimateResizeAndCenterUseCase {
    layout_port: Arc<dyn WindowLayoutPort + Send + Sync>,
}

impl AnimateResizeAndCenterUseCase {
    pub fn new(layout_port: Arc<dyn WindowLayoutPort + Send + Sync>) -> Self {
        Self { layout_port }
    }

    pub async fn execute(
        &self,
        window_id: WindowId,
        target_width: f64,
        target_height: f64,
        duration_ms: u64,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        log::info!(
            "UseCase: AnimateResizeAndCenter - window={:?}, size={}x{}, duration={}ms",
            window_id,
            target_width,
            target_height,
            duration_ms
        );

        // Get monitor info
        let monitor = self.layout_port.get_monitor_info(&window_id).await?;

        // Convert monitor dimensions to logical pixels
        let scale_factor = monitor.scale_factor;
        let screen_width = monitor.width as f64 / scale_factor;
        let screen_height = monitor.height as f64 / scale_factor;
        let monitor_x = monitor.position_x as f64 / scale_factor;
        let monitor_y = monitor.position_y as f64 / scale_factor;

        // Calculate target centered position
        let target_x = monitor_x + (screen_width - target_width) / 2.0;
        let target_y = monitor_y + (screen_height - target_height) / 2.0;

        log::debug!(
            "Target: pos=({}, {}), size={}x{}",
            target_x,
            target_y,
            target_width,
            target_height
        );

        // Get current layout
        let current = self.layout_port.get_window_layout(&window_id).await?;

        log::debug!(
            "Current: pos=({}, {}), size={}x{}",
            current.x,
            current.y,
            current.width,
            current.height
        );

        // Check if already at target
        if (current.width - target_width).abs() < 1.0
            && (current.height - target_height).abs() < 1.0
            && (current.x - target_x).abs() < 1.0
            && (current.y - target_y).abs() < 1.0
        {
            log::info!("Window already at target position and size");
            return Ok(());
        }

        // Animation parameters
        let fps = 60;
        let frame_duration = std::time::Duration::from_millis(1000 / fps);
        let total_frames = (duration_ms as f64 / (1000.0 / fps as f64)).round() as u64;

        log::debug!("Animation: {} frames at {}fps", total_frames, fps);

        // Easing function (easeInOutCubic)
        let ease_in_out_cubic = |t: f64| -> f64 {
            if t < 0.5 {
                4.0 * t * t * t
            } else {
                1.0 - (-2.0 * t + 2.0).powi(3) / 2.0
            }
        };

        // Execute animation
        for frame in 0..=total_frames {
            let progress = frame as f64 / total_frames as f64;
            let eased_progress = ease_in_out_cubic(progress);

            // Calculate frame layout
            let frame_width = current.width + (target_width - current.width) * eased_progress;
            let frame_height = current.height + (target_height - current.height) * eased_progress;
            let frame_x = current.x + (target_x - current.x) * eased_progress;
            let frame_y = current.y + (target_y - current.y) * eased_progress;

            let frame_layout = WindowLayout {
                width: frame_width.round(),
                height: frame_height.round(),
                x: frame_x.round(),
                y: frame_y.round(),
            };

            self.layout_port
                .set_window_layout(&window_id, frame_layout)
                .await?;

            // Wait for next frame
            if frame < total_frames {
                tokio::time::sleep(frame_duration).await;
            }
        }

        // Ensure final state is exact
        let final_layout = WindowLayout {
            width: target_width,
            height: target_height,
            x: target_x,
            y: target_y,
        };

        self.layout_port
            .set_window_layout(&window_id, final_layout)
            .await?;

        log::info!("Animation completed successfully");
        Ok(())
    }
}
