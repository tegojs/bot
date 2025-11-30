//! Preset particle effect implementations

pub mod electric_spark;
pub mod flowing_light;
pub mod pulse_ripple;
pub mod rotating_halo;
pub mod smoke_wisp;
pub mod stardust_scatter;

// New line-based/rain effects
pub mod aurora_wave;
pub mod cosmic_strings;
pub mod heartbeat_pulse;
pub mod laser_beam;
pub mod lightning_arc;
pub mod matrix_rain;
pub mod meteor_shower;
pub mod orbit_rings;
pub mod rain_drop;
pub mod silk_ribbon;
pub mod sonar_pulse;

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

/// Get position on circle edge OUTSIDE the shape, given a normalized angle (0.0 - 1.0)
/// Returns (x, y) position outside the circle by the specified gap
pub fn circle_edge_outside(t: f32, width: f32, height: f32, gap: f32) -> (f32, f32) {
    use std::f32::consts::PI;
    let cx = width / 2.0;
    let cy = height / 2.0;
    let radius = width.min(height) / 2.0 + gap;
    let angle = t * 2.0 * PI;
    (cx + radius * angle.cos(), cy + radius * angle.sin())
}

/// Get outward direction from circle center at given angle (0.0 - 1.0)
pub fn outward_direction(t: f32) -> (f32, f32) {
    use std::f32::consts::PI;
    let angle = t * 2.0 * PI;
    (angle.cos(), angle.sin())
}
