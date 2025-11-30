//! Smoke Wisp effect - slow rising particles with horizontal sway
//! Particles spawn OUTSIDE the circle and drift outward/upward

use super::{circle_edge_outside, outward_direction, random_color};
use crate::effect::particle::Particle;
use crate::effect::PresetEffectOptions;
use rand::Rng;
use std::f32::consts::PI;

/// Spawn a particle for the smoke wisp effect
pub fn spawn(pos: f32, options: &PresetEffectOptions, width: f32, height: f32) -> Particle {
    // Spawn from bottom half of circle primarily (pos 0.25-0.75 is bottom half)
    let spawn_pos = if rand::rng().random::<f32>() > 0.3 {
        rand::rng().random_range(0.25..0.75)
    } else {
        pos
    };

    // Spawn OUTSIDE the circle
    let gap = 6.0;
    let (x, y) = circle_edge_outside(spawn_pos, width, height, gap);

    // Get outward direction and add upward bias
    let (out_x, out_y) = outward_direction(spawn_pos);

    // Upward velocity with slight outward drift
    let speed = rand::rng().random_range(15.0..30.0) * options.speed;
    let vx = out_x * speed * 0.3 + rand::rng().random_range(-5.0..5.0) * options.speed;
    let vy = -speed.abs() + out_y * speed * 0.3; // Upward bias

    // Smoke gray colors or user colors
    let color = if options.particle_colors.is_empty() {
        let gray = rand::rng().random_range(0.4..0.7);
        [gray, gray, gray, 1.0]
    } else {
        random_color(options)
    };

    // Longer lifetime for slow drift
    let lifetime = rand::rng().random_range(2.0..4.0) / options.speed;

    // Random phase for horizontal sway
    let phase = rand::rng().random_range(0.0..2.0 * PI);

    // Particle size
    let (size_min, size_max) = options.particle_size;
    let size = rand::rng().random_range(size_min * 0.5..size_max * 0.8).max(3.0);

    let mut particle = Particle::new(x, y)
        .with_size(size)
        .with_color(color)
        .with_velocity(vx, vy)
        .with_lifetime(lifetime);

    particle.custom = phase;
    particle
}

/// Update a particle for the smoke wisp effect
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

    let phase = particle.custom;

    // Horizontal sway using sine wave
    let sway_amplitude = 30.0 * options.speed;
    let sway_frequency = 2.0 * options.speed;
    let sway = sway_amplitude * (time * sway_frequency + phase).sin();

    // Update velocity with sway
    particle.velocity.0 = sway;

    // Slow down upward velocity
    particle.velocity.1 *= 0.99;

    // Standard physics update
    particle.update(dt);

    // Keep particle outside the circle
    let px = particle.position.0 - cx;
    let py = particle.position.1 - cy;
    let pdist = (px * px + py * py).sqrt();
    let min_dist = circle_radius + particle.size * 0.5 + 4.0;

    if pdist < min_dist && pdist > 0.001 {
        let push_x = px / pdist;
        let push_y = py / pdist;
        particle.position.0 = cx + push_x * min_dist;
        particle.position.1 = cy + push_y * min_dist;
    }

    // Keep alpha high - fade only near end of life
    let life_ratio = (particle.lifetime / 0.5).min(1.0);
    particle.alpha = life_ratio.max(0.3);

    // Size grows as smoke disperses
    let age = particle.age();
    let (size_min, size_max) = options.particle_size;
    let base_size = rand::rng().random_range(size_min * 0.5..size_max * 0.8).max(3.0);
    particle.size = base_size * (1.0 + age * 1.5);
}
