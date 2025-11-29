//! Rotating Halo effect - particles orbit window edge

use super::{center, random_color, random_size};
use crate::effect::particle::Particle;
use crate::effect::PresetEffectOptions;
use std::f32::consts::PI;

/// Spawn a particle for the rotating halo effect
pub fn spawn(edge_position: f32, options: &PresetEffectOptions, width: f32, height: f32) -> Particle {
    let (cx, cy) = center(width, height);
    let radius = width.min(height) / 2.0 + options.edge_width / 2.0;

    // Initial angle based on edge position
    let angle = edge_position * 2.0 * PI;

    let x = cx + radius * angle.cos();
    let y = cy + radius * angle.sin();

    let mut color = random_color(options);
    color[3] *= options.intensity;

    Particle::new(x, y)
        .with_size(random_size(options))
        .with_color(color)
        .with_lifetime(f32::INFINITY) // Particles don't die, they orbit
}

/// Update a particle for the rotating halo effect
pub fn update(
    particle: &mut Particle,
    _dt: f32,
    time: f32,
    options: &PresetEffectOptions,
    width: f32,
    height: f32,
) {
    let (cx, cy) = center(width, height);
    let radius = width.min(height) / 2.0 + options.edge_width / 2.0;

    // Angular velocity based on speed option
    let angular_velocity = options.speed * 0.5; // radians per second

    // Calculate current angle from initial position + rotation
    let initial_angle = particle.custom; // Store initial angle in custom field
    let angle = initial_angle + time * angular_velocity;

    // Update position on orbit
    particle.position.0 = cx + radius * angle.cos();
    particle.position.1 = cy + radius * angle.sin();

    // Pulse size slightly
    let pulse = 1.0 + 0.2 * (time * 3.0 + initial_angle).sin();
    particle.size = random_size(options) * pulse;

    // Color variation based on angle
    let color_phase = (angle * 2.0).sin() * 0.5 + 0.5;
    if !options.particle_colors.is_empty() {
        let base_color = options.particle_colors[0];
        particle.color[0] = base_color[0] * (0.8 + 0.2 * color_phase);
        particle.color[1] = base_color[1] * (0.8 + 0.2 * color_phase);
        particle.color[2] = base_color[2] * (0.8 + 0.2 * color_phase);
    }
}
