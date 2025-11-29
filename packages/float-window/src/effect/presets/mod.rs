//! Preset particle effect implementations

pub mod electric_spark;
pub mod flowing_light;
pub mod pulse_ripple;
pub mod rotating_halo;
pub mod smoke_wisp;
pub mod stardust_scatter;

use super::PresetEffectOptions;
use rand::Rng;

/// Helper to get a random color from the options
pub fn random_color(options: &PresetEffectOptions) -> [f32; 4] {
    if options.particle_colors.is_empty() {
        [1.0, 1.0, 1.0, 1.0]
    } else {
        let idx = rand::rng().random_range(0..options.particle_colors.len());
        options.particle_colors[idx]
    }
}

/// Helper to get a random size from the options
pub fn random_size(options: &PresetEffectOptions) -> f32 {
    let (min, max) = options.particle_size;
    rand::rng().random_range(min..=max)
}

/// Get position on window edge given a normalized position (0.0 - 1.0)
pub fn edge_position(t: f32, width: f32, height: f32) -> (f32, f32) {
    let perimeter = 2.0 * (width + height);
    let pos = t * perimeter;

    if pos < width {
        // Top edge
        (pos, 0.0)
    } else if pos < width + height {
        // Right edge
        (width, pos - width)
    } else if pos < 2.0 * width + height {
        // Bottom edge
        (2.0 * width + height - pos, height)
    } else {
        // Left edge
        (0.0, perimeter - pos)
    }
}

/// Get the center of the window
pub fn center(width: f32, height: f32) -> (f32, f32) {
    (width / 2.0, height / 2.0)
}
