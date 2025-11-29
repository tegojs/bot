//! Flowing Light effect - particles flow along edge with brightness modulation

use super::{edge_position, random_color, random_size};
use crate::effect::particle::Particle;
use crate::effect::PresetEffectOptions;
use std::f32::consts::PI;

/// Spawn a particle for the flowing light effect
pub fn spawn(pos: f32, options: &PresetEffectOptions, width: f32, height: f32) -> Particle {
    let (x, y) = edge_position(pos, width, height);

    let mut color = random_color(options);
    color[3] *= options.intensity;

    let mut particle = Particle::new(x, y)
        .with_size(random_size(options))
        .with_color(color)
        .with_lifetime(f32::INFINITY); // Particles don't die

    // Store initial edge position for flow calculation
    particle.custom = pos;
    particle
}

/// Update a particle for the flowing light effect
pub fn update(
    particle: &mut Particle,
    dt: f32,
    time: f32,
    options: &PresetEffectOptions,
    width: f32,
    height: f32,
) {
    // Flow speed
    let flow_speed = options.speed * 0.1;

    // Update position along edge
    let mut edge_pos = particle.custom + flow_speed * dt;
    if edge_pos > 1.0 {
        edge_pos -= 1.0;
    }
    particle.custom = edge_pos;

    // Get new position
    let (x, y) = edge_position(edge_pos, width, height);
    particle.position = (x, y);

    // Brightness modulation (flowing wave)
    let wavelength = 0.2; // Portion of perimeter for one wave
    let wave_pos = edge_pos / wavelength;
    let brightness = ((wave_pos * 2.0 * PI + time * options.speed * 5.0).sin() + 1.0) / 2.0;

    // Apply brightness to alpha
    particle.alpha = particle.color[3] * (0.3 + 0.7 * brightness);

    // Slight size variation
    particle.size = random_size(options) * (0.8 + 0.4 * brightness);
}
