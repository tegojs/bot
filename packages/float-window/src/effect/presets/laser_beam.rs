//! LaserBeam effect - rotating laser lines emanating from circle edge
//! Lasers radiate OUTWARD from the circle

use super::{circle_edge_outside, outward_direction, random_color};
use crate::effect::particle::Particle;
use crate::effect::PresetEffectOptions;
use rand::Rng;

/// Spawn a particle for the laser beam effect
pub fn spawn(pos: f32, options: &PresetEffectOptions, width: f32, height: f32) -> Particle {
    // Spawn at circle edge
    let gap = 2.0;
    let (x, y) = circle_edge_outside(pos, width, height, gap);

    // Get outward direction
    let (dir_x, dir_y) = outward_direction(pos);

    // Fast outward velocity
    let speed = rand::rng().random_range(80.0..150.0) * options.speed;
    let vx = dir_x * speed;
    let vy = dir_y * speed;

    // Laser colors - bright neon
    let color = if options.particle_colors.is_empty() {
        let hue = rand::rng().random_range(0.0..1.0);
        // Neon colors
        match (hue * 4.0) as i32 {
            0 => [1.0, 0.0, 0.3, 1.0], // Red/pink
            1 => [0.0, 1.0, 0.5, 1.0], // Green
            2 => [0.3, 0.5, 1.0, 1.0], // Blue
            _ => [1.0, 0.8, 0.0, 1.0], // Yellow
        }
    } else {
        random_color(options)
    };

    // Thin laser beams
    let (size_min, size_max) = options.particle_size;
    let size = rand::rng().random_range(size_min * 0.2..size_max * 0.4).max(2.0);

    let lifetime = rand::rng().random_range(0.3..0.6) / options.speed;

    let mut particle = Particle::new(x, y)
        .with_size(size)
        .with_color(color)
        .with_velocity(vx, vy)
        .with_lifetime(lifetime);

    // Store beam length
    particle.custom = rand::rng().random_range(15.0..30.0);
    particle
}

/// Update a particle for the laser beam effect
pub fn update(
    particle: &mut Particle,
    dt: f32,
    _time: f32,
    _options: &PresetEffectOptions,
    width: f32,
    height: f32,
) {
    let cx = width / 2.0;
    let cy = height / 2.0;
    let circle_radius = width.min(height) / 2.0;

    // Standard physics update
    particle.update(dt);

    // Keep particle outside the circle
    let px = particle.position.0 - cx;
    let py = particle.position.1 - cy;
    let pdist = (px * px + py * py).sqrt();
    let min_dist = circle_radius + 2.0;

    if pdist < min_dist && pdist > 0.001 {
        let push_x = px / pdist;
        let push_y = py / pdist;
        particle.position.0 = cx + push_x * min_dist;
        particle.position.1 = cy + push_y * min_dist;
    }

    // Bright alpha
    particle.alpha = 1.0;
}
