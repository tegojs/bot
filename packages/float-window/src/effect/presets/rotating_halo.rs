//! Rotating Halo effect - particles orbit OUTSIDE window edge like a glowing ring

use super::{random_color, random_size};
use crate::effect::particle::Particle;
use crate::effect::PresetEffectOptions;
use std::f32::consts::PI;

/// Spawn a particle for the rotating halo effect
pub fn spawn(edge_position: f32, options: &PresetEffectOptions, width: f32, height: f32) -> Particle {
    let cx = width / 2.0;
    let cy = height / 2.0;
    let circle_radius = width.min(height) / 2.0;

    // Particles spawn OUTSIDE the circle
    let (size_min, size_max) = options.particle_size;
    let particle_size = size_min + (size_max - size_min) * 0.5;
    let gap = 4.0;
    let orbit_radius = circle_radius + particle_size * 0.5 + gap + options.edge_width * 0.3;

    // Initial angle based on edge position (distribute particles evenly around circle)
    let initial_angle = edge_position * 2.0 * PI;

    let x = cx + orbit_radius * initial_angle.cos();
    let y = cy + orbit_radius * initial_angle.sin();

    // Use bright, vivid colors
    let mut color = random_color(options);
    color[3] = 1.0; // Full alpha for bright particles

    // Create particle with initial angle stored in custom field
    let mut particle = Particle::new(x, y)
        .with_size(random_size(options))
        .with_color(color)
        .with_lifetime(f32::INFINITY); // Particles don't die, they orbit

    // Store initial angle for rotation calculation
    particle.custom = initial_angle;
    particle.alpha = 1.0; // Start fully visible

    particle
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
    let cx = width / 2.0;
    let cy = height / 2.0;
    let circle_radius = width.min(height) / 2.0;

    // Calculate particle size for offset
    let (size_min, size_max) = options.particle_size;
    let base_particle_size = size_min + (size_max - size_min) * 0.5;

    // Angular velocity based on speed option
    let angular_velocity = options.speed * 1.5;

    // Get initial angle from custom field
    let initial_angle = particle.custom;

    // Calculate current angle with rotation over time
    let angle = initial_angle + time * angular_velocity;

    // Pulse size for sparkle effect
    let pulse = 1.0 + 0.3 * (time * 6.0 + initial_angle * 2.0).sin();
    let current_size = base_particle_size * pulse;
    particle.size = current_size;

    // Orbit radius stays OUTSIDE the circle - particle edge should not touch circle
    // Minimum orbit = circle edge + half particle size (since particle is drawn from center) + gap
    let gap = 4.0; // Minimum gap between particle edge and circle edge
    let min_orbit = circle_radius + current_size * 0.5 + gap;

    // Add small outward variation for organic look (only adds, never subtracts)
    let radius_variation = 0.05 * (time * 3.0 + initial_angle * 2.0).sin().abs();
    let orbit_radius = min_orbit + options.edge_width * 0.3 + radius_variation * 5.0;

    // Update position on orbit - always outside the circle
    particle.position.0 = cx + orbit_radius * angle.cos();
    particle.position.1 = cy + orbit_radius * angle.sin();

    // Color cycling through the provided colors
    if options.particle_colors.len() >= 2 {
        let color_time = time * 0.8 + initial_angle / (2.0 * PI);
        let color_idx = color_time % (options.particle_colors.len() as f32);
        let idx1 = color_idx.floor() as usize % options.particle_colors.len();
        let idx2 = (idx1 + 1) % options.particle_colors.len();
        let t = color_idx.fract();

        let c1 = &options.particle_colors[idx1];
        let c2 = &options.particle_colors[idx2];

        particle.color[0] = c1[0] * (1.0 - t) + c2[0] * t;
        particle.color[1] = c1[1] * (1.0 - t) + c2[1] * t;
        particle.color[2] = c1[2] * (1.0 - t) + c2[2] * t;
    }

    // Alpha pulsing - keep bright with gentle variation
    let alpha_pulse = 0.85 + 0.15 * (time * 4.0 + initial_angle).sin();
    particle.alpha = alpha_pulse; // Use full brightness, ignore intensity for vivid effect
}
