//! OrbitRings effect - particles orbiting in rings around the circle
//! Orbits stay OUTSIDE the circle

use super::{circle_edge_outside, random_color};
use crate::gui::effect::PresetEffectOptions;
use crate::gui::effect::particle::Particle;
use rand::Rng;
use std::f32::consts::PI;

/// Spawn a particle for the orbit rings effect
pub fn spawn(pos: f32, options: &PresetEffectOptions, width: f32, height: f32) -> Particle {
    // Spawn at different orbit distances
    let orbit_layer = rand::rng().random_range(0..3);
    let gap = 10.0 + orbit_layer as f32 * 15.0;
    let (x, y) = circle_edge_outside(pos, width, height, gap);

    // Orbital velocity - tangent to circle
    let angle = pos * 2.0 * PI;
    let tangent_x = -angle.sin();
    let tangent_y = angle.cos();

    // Different speeds for different orbits (inner faster)
    let base_speed = 60.0 - orbit_layer as f32 * 15.0;
    let speed = rand::rng().random_range(base_speed * 0.8..base_speed * 1.2) * options.speed;

    // Alternate direction based on layer
    let direction = if orbit_layer % 2 == 0 { 1.0 } else { -1.0 };
    let vx = tangent_x * speed * direction;
    let vy = tangent_y * speed * direction;

    // Ring colors - different color per layer
    let color = if options.particle_colors.is_empty() {
        match orbit_layer {
            0 => [1.0, 0.8, 0.3, 1.0], // Gold (inner)
            1 => [0.6, 0.8, 1.0, 1.0], // Light blue (middle)
            _ => [1.0, 0.6, 0.8, 1.0], // Pink (outer)
        }
    } else {
        random_color(options)
    };

    // Small orbital particles
    let (size_min, size_max) = options.particle_size;
    let size = rand::rng().random_range(size_min * 0.3..size_max * 0.5).max(2.0);

    let lifetime = rand::rng().random_range(3.0..5.0) / options.speed;

    let mut particle = Particle::new(x, y)
        .with_size(size)
        .with_color(color)
        .with_velocity(vx, vy)
        .with_lifetime(lifetime);

    // Store orbit layer and initial angle
    particle.custom = orbit_layer as f32 + pos * 0.001;
    particle
}

/// Update a particle for the orbit rings effect
pub fn update(
    particle: &mut Particle,
    dt: f32,
    _time: f32,
    _options: &PresetEffectOptions,
    width: f32,
    height: f32,
) {
    let cx = width / 2.0;
    let cy = height / 2.0;
    let circle_radius = width.min(height) / 2.0;

    // Standard physics update
    particle.update(dt);

    // Extract orbit layer from custom
    let orbit_layer = particle.custom.floor() as i32;
    let target_gap = 10.0 + orbit_layer as f32 * 15.0;
    let target_dist = circle_radius + target_gap;

    // Keep particle in its orbit
    let px = particle.position.0 - cx;
    let py = particle.position.1 - cy;
    let pdist = (px * px + py * py).sqrt();

    if pdist > 0.001 {
        // Gently correct to orbital distance
        let correction = (target_dist - pdist) * 0.1;
        let push_x = px / pdist;
        let push_y = py / pdist;
        particle.position.0 += push_x * correction;
        particle.position.1 += push_y * correction;

        // Update velocity to maintain orbit
        let current_angle = py.atan2(px);
        let tangent_x = -current_angle.sin();
        let tangent_y = current_angle.cos();
        let speed = (particle.velocity.0.powi(2) + particle.velocity.1.powi(2)).sqrt();
        let direction = if orbit_layer % 2 == 0 { 1.0 } else { -1.0 };
        particle.velocity.0 = tangent_x * speed * direction;
        particle.velocity.1 = tangent_y * speed * direction;
    }

    // Steady alpha
    particle.alpha = 0.9;
}
