//! Flowing Light effect - particles flow along circle edge with brightness modulation
//! Particles flow OUTSIDE the circle

use super::{circle_edge_outside, random_color};
use crate::gui::effect::PresetEffectOptions;
use crate::gui::effect::particle::Particle;
use rand::Rng;
use std::f32::consts::PI;

/// Spawn a particle for the flowing light effect
pub fn spawn(pos: f32, options: &PresetEffectOptions, width: f32, height: f32) -> Particle {
    // Spawn OUTSIDE the circle with a small gap
    let gap = 6.0;
    let (x, y) = circle_edge_outside(pos, width, height, gap);

    let color = random_color(options);

    // Particle size - smaller for flowing effect
    let (size_min, size_max) = options.particle_size;
    let size = rand::rng().random_range(size_min * 0.5..size_max * 0.8).max(3.0);

    let mut particle =
        Particle::new(x, y).with_size(size).with_color(color).with_lifetime(f32::INFINITY); // Particles don't die, they flow

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
    // Flow speed - particles orbit around the circle
    let flow_speed = options.speed * 0.1;

    // Update position along circle edge
    let mut edge_pos = particle.custom + flow_speed * dt;
    if edge_pos > 1.0 {
        edge_pos -= 1.0;
    }
    particle.custom = edge_pos;

    // Get new position OUTSIDE the circle
    let gap = 6.0;
    let (x, y) = circle_edge_outside(edge_pos, width, height, gap);
    particle.position = (x, y);

    // Brightness modulation (flowing wave)
    let wavelength = 0.2; // Portion of perimeter for one wave
    let wave_pos = edge_pos / wavelength;
    let brightness = ((wave_pos * 2.0 * PI + time * options.speed * 5.0).sin() + 1.0) / 2.0;

    // Keep alpha high to avoid compositor artifacts
    particle.alpha = (0.3 + 0.7 * brightness).max(0.3);

    // Slight size variation
    let (size_min, size_max) = options.particle_size;
    let base_size = rand::rng().random_range(size_min * 0.5..size_max * 0.8).max(3.0);
    particle.size = base_size * (0.8 + 0.4 * brightness);
}
