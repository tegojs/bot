//! Smoke Wisp effect - slow rising particles with horizontal sway

use super::{edge_position, random_color, random_size};
use crate::effect::particle::Particle;
use crate::effect::PresetEffectOptions;
use rand::Rng;
use std::f32::consts::PI;

/// Spawn a particle for the smoke wisp effect
pub fn spawn(pos: f32, options: &PresetEffectOptions, width: f32, height: f32) -> Particle {
    // Spawn from bottom edge primarily
    let spawn_pos = if rand::rng().random::<f32>() > 0.3 {
        // Bottom edge (0.5 - 0.75 of perimeter)
        rand::rng().random_range(0.5..0.75)
    } else {
        pos
    };

    let (x, y) = edge_position(spawn_pos, width, height);

    // Upward velocity with slight random horizontal
    let vy = -rand::rng().random_range(15.0..30.0) * options.speed;
    let vx = rand::rng().random_range(-5.0..5.0) * options.speed;

    // Smoke gray colors
    let color = if options.particle_colors.is_empty() {
        let gray = rand::rng().random_range(0.4..0.7);
        [gray, gray, gray, options.intensity * 0.6]
    } else {
        let mut c = random_color(options);
        c[3] *= options.intensity * 0.6;
        c
    };

    // Longer lifetime for slow drift
    let lifetime = rand::rng().random_range(2.0..4.0) / options.speed;

    // Random phase for horizontal sway
    let phase = rand::rng().random_range(0.0..2.0 * PI);

    let mut particle = Particle::new(x, y)
        .with_size(random_size(options))
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
    _width: f32,
    _height: f32,
) {
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

    // Fade out non-linearly
    let age = particle.age();
    particle.alpha = particle.color[3] * (1.0 - age * age);

    // Size grows as smoke disperses
    let base_size = (particle.size + particle.size) / 2.0;
    particle.size = base_size * (1.0 + age * 2.0);
}
