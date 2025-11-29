//! Electric Spark effect - fast flickering particles with branching

use super::{edge_position, random_color, random_size};
use crate::effect::particle::Particle;
use crate::effect::PresetEffectOptions;
use rand::Rng;
use std::f32::consts::PI;

/// Spawn a particle for the electric spark effect
pub fn spawn(pos: f32, options: &PresetEffectOptions, width: f32, height: f32) -> Particle {
    let (x, y) = edge_position(pos, width, height);

    // Random jump direction
    let angle = rand::rng().random_range(0.0..2.0 * PI);
    let speed = rand::rng().random_range(50.0..150.0) * options.speed;

    let vx = speed * angle.cos();
    let vy = speed * angle.sin();

    // Electric blue/white colors
    let color = if options.particle_colors.is_empty() {
        let brightness = rand::rng().random_range(0.8..1.0);
        [brightness, brightness, 1.0, options.intensity]
    } else {
        let mut c = random_color(options);
        c[3] *= options.intensity;
        c
    };

    // Very short lifetime for spark effect
    let lifetime = rand::rng().random_range(0.05..0.15) / options.speed;

    Particle::new(x, y)
        .with_size(random_size(options) * 0.5) // Smaller particles
        .with_color(color)
        .with_velocity(vx, vy)
        .with_lifetime(lifetime)
}

/// Update a particle for the electric spark effect
pub fn update(
    particle: &mut Particle,
    dt: f32,
    _time: f32,
    options: &PresetEffectOptions,
    _width: f32,
    _height: f32,
) {
    // Random jitter (electricity effect)
    let jitter_strength = 20.0 * options.speed;
    particle.velocity.0 += rand::rng().random_range(-jitter_strength..jitter_strength);
    particle.velocity.1 += rand::rng().random_range(-jitter_strength..jitter_strength);

    // Standard physics update
    particle.update(dt);

    // Flicker effect
    let flicker = if rand::rng().random::<f32>() > 0.3 { 1.0 } else { 0.2 };
    particle.alpha = particle.color[3] * flicker;

    // Random size changes
    if rand::rng().random::<f32>() > 0.8 {
        particle.size = random_size(options) * rand::rng().random_range(0.5..1.5);
    }
}
