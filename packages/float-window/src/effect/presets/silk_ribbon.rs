//! SilkRibbon effect - multiple continuous ribbons flowing around the circle
//! The ribbons wrap completely around the circle with gentle wind-blown wave motion
//! Drawing connected line segments with gradient colors, interleaved for visual effect

use crate::effect::particle::Particle;
use crate::effect::PresetEffectOptions;
use std::f32::consts::PI;

/// Number of segments per ribbon (more = smoother curve)
const SEGMENTS_PER_RIBBON: usize = 120;

/// Lerp between two colors
fn lerp_color(c1: [f32; 4], c2: [f32; 4], t: f32) -> [f32; 4] {
    [
        c1[0] + (c2[0] - c1[0]) * t,
        c1[1] + (c2[1] - c1[1]) * t,
        c1[2] + (c2[2] - c1[2]) * t,
        c1[3] + (c2[3] - c1[3]) * t,
    ]
}

/// Get gradient color based on position (0.0 to 1.0) around the circle
fn get_gradient_color(position: f32, colors: &[[f32; 4]]) -> [f32; 4] {
    if colors.is_empty() {
        return [0.4, 0.8, 1.0, 1.0]; // Default cyan
    }
    if colors.len() == 1 {
        return colors[0];
    }

    // Map position to color index
    let total_segments = colors.len();
    let scaled_pos = position * total_segments as f32;
    let idx = (scaled_pos.floor() as usize) % total_segments;
    let next_idx = (idx + 1) % total_segments;
    let t = scaled_pos.fract();

    lerp_color(colors[idx], colors[next_idx], t)
}

/// Spawn particles that represent ribbon segments
/// Each particle represents a point on one of the continuous ribbons
pub fn spawn(pos: f32, options: &PresetEffectOptions, width: f32, height: f32) -> Particle {
    let cx = width / 2.0;
    let cy = height / 2.0;
    let radius = width.min(height) / 2.0;

    let ribbon_count = options.ribbon_count.max(1);
    let total_segments = SEGMENTS_PER_RIBBON * ribbon_count;

    // Determine which ribbon and segment within that ribbon
    let total_index = (pos * total_segments as f32) as usize;
    let ribbon_id = total_index / SEGMENTS_PER_RIBBON;
    let segment_index = total_index % SEGMENTS_PER_RIBBON;

    // Angle for this segment within its ribbon
    let angle = (segment_index as f32 / SEGMENTS_PER_RIBBON as f32) * 2.0 * PI;

    // Start position with gap from circle edge
    let gap = 5.0;
    let x = cx + angle.cos() * (radius + gap);
    let y = cy + angle.sin() * (radius + gap);

    // Get gradient color based on position around circle
    let position = segment_index as f32 / SEGMENTS_PER_RIBBON as f32;
    let color = get_gradient_color(position, &options.particle_colors);

    // Thin line
    let size = 1.5;

    let mut particle = Particle::new(x, y)
        .with_size(size)
        .with_color(color)
        .with_velocity(0.0, 0.0)
        .with_lifetime(f32::MAX)
        .as_line();

    // Store: custom = angle, custom2 encodes both ribbon_id and segment_index
    // ribbon_id in high bits (multiply by 1000), segment_index in low bits
    particle.custom = angle;
    particle.custom2 = (ribbon_id * 1000 + segment_index) as f32;

    // Initialize prev_position
    particle.prev_position = (x, y);

    particle
}

/// Calculate ribbon position for a given angle at a given time
/// ribbon_id: index of the ribbon (0, 1, 2, ...)
/// ribbon_count: total number of ribbons
fn calc_ribbon_position(
    angle: f32,
    time: f32,
    ribbon_id: usize,
    ribbon_count: usize,
    options: &PresetEffectOptions,
    cx: f32,
    cy: f32,
    radius: f32,
) -> (f32, f32) {
    // Slow rotation speed
    let wave_speed = 0.5 * options.speed;
    let t = time * wave_speed;

    // Regular flower-like petals - exactly 5 petals
    let petal_count = 5.0;
    let petal_amp = options.petal_amplitude * options.intensity;

    // Phase offset: evenly distribute ribbons across ALL petals
    // For N ribbons with P petals, each ribbon is offset by P/N petals
    // This means ribbon spacing = (P/N) * (2*PI/P) = 2*PI/N
    // But we want to space by (petal_count / ribbon_count) petals
    let petals_per_ribbon = petal_count / ribbon_count as f32;
    let phase_offset = ribbon_id as f32 * petals_per_ribbon * (2.0 * PI / petal_count);

    // Pure sine wave for regular petal shape, rotating over time
    // (1 + sin) / 2 maps to 0-1, then multiply by amplitude
    let wave = (1.0 + (angle * petal_count + t + phase_offset).sin()) * 0.5 * petal_amp;

    // Gap from circle edge
    let gap = 5.0;

    // Distance from center
    let dist = radius + gap + wave;

    let x = cx + angle.cos() * dist;
    let y = cy + angle.sin() * dist;

    (x, y)
}

/// Update ribbon segments - apply wave motion and update gradient colors
pub fn update(
    particle: &mut Particle,
    _dt: f32,
    time: f32,
    options: &PresetEffectOptions,
    width: f32,
    height: f32,
) {
    let cx = width / 2.0;
    let cy = height / 2.0;
    let radius = width.min(height) / 2.0;

    let ribbon_count = options.ribbon_count.max(1);

    // Decode ribbon_id and segment_index from custom2
    let encoded = particle.custom2 as usize;
    let ribbon_id = encoded / 1000;
    let segment_index = encoded % 1000;
    let current_angle = particle.custom;

    // Calculate next segment's angle (wrapping around within this ribbon)
    let next_segment = (segment_index + 1) % SEGMENTS_PER_RIBBON;
    let next_angle = (next_segment as f32 / SEGMENTS_PER_RIBBON as f32) * 2.0 * PI;

    // Get positions for current and next segment (with ribbon-specific phase offset)
    let (curr_x, curr_y) = calc_ribbon_position(current_angle, time, ribbon_id, ribbon_count, options, cx, cy, radius);
    let (next_x, next_y) = calc_ribbon_position(next_angle, time, ribbon_id, ribbon_count, options, cx, cy, radius);

    // Current position is where we draw FROM
    particle.position = (curr_x, curr_y);
    // Prev position is where we draw TO (the next segment, to connect them)
    particle.prev_position = (next_x, next_y);

    // Update color based on current position for animated gradient
    let position = segment_index as f32 / SEGMENTS_PER_RIBBON as f32;
    let color = get_gradient_color(position, &options.particle_colors);
    particle.color = color;

    // Full alpha
    particle.alpha = 1.0;
}
