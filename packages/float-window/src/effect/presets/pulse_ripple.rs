//! Pulse Ripple effect - particles expand outward from edge in concentric waves

use super::{center, edge_position, random_color, random_size};
use crate::effect::particle::Particle;
use crate::effect::PresetEffectOptions;
use rand::Rng;
use std::f32::consts::PI;

/// Spawn a particle for the pulse ripple effect
pub fn spawn(pos: f32, options: &PresetEffectOptions, width: f32, height: f32) -> Particle {
    let (x, y) = edge_position(pos, width, height);
    let (cx, cy) = center(width, height);

    // Direction pointing outward from center
    let dx = x - cx;
    let dy = y - cy;
    let len = (dx * dx + dy * dy).sqrt().max(1.0);
    let nx = dx / len;
    let ny = dy / len;

    // Outward velocity
    let speed = options.speed * 50.0;
    let vx = nx * speed;
    let vy = ny * speed;

    let mut color = random_color(options);
    color[3] *= options.intensity;

    // Random phase for wave effect
    let phase = rand::rng().random_range(0.0..2.0 * PI);

    let mut particle = Particle::new(x, y)
        .with_size(random_size(options))
        .with_color(color)
        .with_velocity(vx, vy)
        .with_lifetime(1.5 / options.speed);

    particle.custom = phase; // Store phase for wave calculation
    particle
}

/// Update a particle for the pulse ripple effect
pub fn update(
    particle: &mut Particle,
    dt: f32,
    time: f32,
    options: &PresetEffectOptions,
    _width: f32,
    _height: f32,
) {
    // Standard physics update
    particle.update(dt);

    // Wave modulation of alpha
    let phase = particle.custom;
    let wave = ((time * 5.0 * options.speed + phase).sin() + 1.0) / 2.0;

    // Fade out based on age
    let age = particle.age();
    particle.alpha = particle.color[3] * (1.0 - age) * (0.5 + 0.5 * wave);

    // Size grows as particle ages
    let base_size = (particle.size + particle.size) / 2.0;
    particle.size = base_size * (1.0 + age * 0.5);
}
