//! Stardust Scatter effect - random emission from edge with drift

use super::{edge_position, random_color, random_size};
use crate::effect::particle::Particle;
use crate::effect::PresetEffectOptions;
use rand::Rng;
use std::f32::consts::PI;

/// Spawn a particle for the stardust scatter effect
pub fn spawn(pos: f32, options: &PresetEffectOptions, width: f32, height: f32) -> Particle {
    let (x, y) = edge_position(pos, width, height);

    // Random direction with slight outward bias
    let angle = rand::rng().random_range(0.0..2.0 * PI);
    let speed = rand::rng().random_range(10.0..30.0) * options.speed;

    let vx = speed * angle.cos();
    let vy = speed * angle.sin();

    let mut color = random_color(options);
    color[3] *= options.intensity;

    // Random lifetime
    let lifetime = rand::rng().random_range(0.5..2.0) / options.speed;

    Particle::new(x, y)
        .with_size(random_size(options))
        .with_color(color)
        .with_velocity(vx, vy)
        .with_lifetime(lifetime)
}

/// Update a particle for the stardust scatter effect
pub fn update(
    particle: &mut Particle,
    dt: f32,
    _time: f32,
    options: &PresetEffectOptions,
    _width: f32,
    _height: f32,
) {
    // Apply slight deceleration (drag)
    let drag = 0.98;
    particle.velocity.0 *= drag;
    particle.velocity.1 *= drag;

    // Standard physics update
    particle.update(dt);

    // Fade out exponentially
    let age = particle.age();
    particle.alpha = particle.color[3] * (-age * 3.0).exp();

    // Twinkle effect
    let twinkle = (age * 20.0 * options.speed).sin().abs();
    particle.alpha *= 0.7 + 0.3 * twinkle;
}
