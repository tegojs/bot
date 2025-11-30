//! AuroraWave effect - flowing waves of color around the circle
//! Aurora stays OUTSIDE the circle

use super::{circle_edge_outside, random_color};
use crate::effect::particle::Particle;
use crate::effect::PresetEffectOptions;
use rand::Rng;
use std::f32::consts::PI;

/// Spawn a particle for the aurora wave effect
pub fn spawn(pos: f32, options: &PresetEffectOptions, width: f32, height: f32) -> Particle {
    // Spawn at circle edge with varying distance
    let gap = rand::rng().random_range(5.0..30.0);
    let (x, y) = circle_edge_outside(pos, width, height, gap);

    // Slow tangential movement for wave effect
    let angle = pos * 2.0 * PI;
    let tangent_x = -angle.sin();
    let tangent_y = angle.cos();

    let speed = rand::rng().random_range(10.0..30.0) * options.speed;
    let vx = tangent_x * speed;
    let vy = tangent_y * speed;

    // Aurora colors - greens, blues, purples, pinks
    let color = if options.particle_colors.is_empty() {
        let hue = rand::rng().random_range(0.0..1.0);
        match (hue * 4.0) as i32 {
            0 => [0.3, 1.0, 0.5, 1.0],  // Green
            1 => [0.4, 0.8, 1.0, 1.0],  // Cyan
            2 => [0.6, 0.4, 1.0, 1.0],  // Purple
            _ => [1.0, 0.5, 0.8, 1.0],  // Pink
        }
    } else {
        random_color(options)
    };

    // Larger, softer particles
    let (size_min, size_max) = options.particle_size;
    let size = rand::rng().random_range(size_min * 0.5..size_max * 0.8).max(4.0);

    let lifetime = rand::rng().random_range(2.0..4.0) / options.speed;

    let mut particle = Particle::new(x, y)
        .with_size(size)
        .with_color(color)
        .with_velocity(vx, vy)
        .with_lifetime(lifetime);

    // Store wave phase
    particle.custom = pos;
    particle
}

/// Update a particle for the aurora wave effect
pub fn update(
    particle: &mut Particle,
    dt: f32,
    time: f32,
    options: &PresetEffectOptions,
    width: f32,
    height: f32,
) {
    let cx = width / 2.0;
    let cy = height / 2.0;
    let circle_radius = width.min(height) / 2.0;

    // Standard physics update
    particle.update(dt);

    // Add wave motion - particles oscillate in/out
    let wave_offset = (time * 2.0 * options.speed + particle.custom * PI * 2.0).sin() * 10.0;

    // Keep particle outside the circle with wave motion
    let px = particle.position.0 - cx;
    let py = particle.position.1 - cy;
    let pdist = (px * px + py * py).sqrt();
    let min_dist = circle_radius + 8.0 + wave_offset.max(0.0);
    let max_dist = circle_radius + 40.0 + wave_offset;

    if pdist < min_dist && pdist > 0.001 {
        let push_x = px / pdist;
        let push_y = py / pdist;
        particle.position.0 = cx + push_x * min_dist;
        particle.position.1 = cy + push_y * min_dist;
    } else if pdist > max_dist && pdist > 0.001 {
        let push_x = px / pdist;
        let push_y = py / pdist;
        particle.position.0 = cx + push_x * max_dist;
        particle.position.1 = cy + push_y * max_dist;
    }

    // Soft pulsing alpha
    let pulse = (time * 1.5 + particle.custom * 3.0).sin() * 0.3 + 0.7;
    let age = particle.age();
    particle.alpha = (pulse * (1.0 - age * 0.5)).max(0.3);
}
