//! Pulse Ripple effect - particles expand outward from circle edge in waves
//! Particles spawn OUTSIDE the circle and drift outward

use super::{circle_edge_outside, outward_direction, random_color};
use crate::gui::effect::PresetEffectOptions;
use crate::gui::effect::particle::Particle;
use rand::Rng;
use std::f32::consts::PI;

/// Spawn a particle for the pulse ripple effect
pub fn spawn(pos: f32, options: &PresetEffectOptions, width: f32, height: f32) -> Particle {
    // Spawn OUTSIDE the circle with a small gap
    let gap = 8.0;
    let (x, y) = circle_edge_outside(pos, width, height, gap);

    // Get outward direction
    let (dir_x, dir_y) = outward_direction(pos);

    // Slow outward velocity
    let speed = rand::rng().random_range(5.0..15.0) * options.speed;
    let vx = dir_x * speed;
    let vy = dir_y * speed;

    let color = random_color(options);

    // Random phase for wave effect
    let phase = rand::rng().random_range(0.0..2.0 * PI);

    // Smaller particle size
    let (size_min, size_max) = options.particle_size;
    let size = rand::rng().random_range(size_min * 0.4..size_max * 0.7).max(3.0);

    let mut particle = Particle::new(x, y)
        .with_size(size)
        .with_color(color)
        .with_velocity(vx, vy)
        .with_lifetime(rand::rng().random_range(1.5..2.5));

    particle.custom = phase; // Store phase for wave calculation
    particle
}

/// Update a particle for the pulse ripple effect
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

    // Keep alpha high - fade only near end of life (like electric_spark)
    let life_ratio = (particle.lifetime / 0.5).min(1.0); // Fade in last 0.5 seconds
    particle.alpha = life_ratio.max(0.3); // Minimum 0.3 alpha

    // Keep particle outside the circle
    let px = particle.position.0 - cx;
    let py = particle.position.1 - cy;
    let pdist = (px * px + py * py).sqrt();
    let min_dist = circle_radius + particle.size * 0.5 + 4.0;

    if pdist < min_dist && pdist > 0.001 {
        // Push particle outside
        let push_x = px / pdist;
        let push_y = py / pdist;
        particle.position.0 = cx + push_x * min_dist;
        particle.position.1 = cy + push_y * min_dist;
    }
}
