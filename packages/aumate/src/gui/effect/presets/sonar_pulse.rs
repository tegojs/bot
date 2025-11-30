//! SonarPulse effect - expanding ring lines radiating from circle edge
//! Rings expand OUTWARD from the circle

use super::{circle_edge_outside, outward_direction, random_color};
use crate::gui::effect::PresetEffectOptions;
use crate::gui::effect::particle::Particle;
use rand::Rng;

/// Spawn a particle for the sonar pulse effect
pub fn spawn(pos: f32, options: &PresetEffectOptions, width: f32, height: f32) -> Particle {
    // Spawn at circle edge - particles form expanding rings
    let gap = 2.0;
    let (x, y) = circle_edge_outside(pos, width, height, gap);

    // Get outward direction
    let (dir_x, dir_y) = outward_direction(pos);

    // Slow outward expansion
    let speed = rand::rng().random_range(20.0..40.0) * options.speed;
    let vx = dir_x * speed;
    let vy = dir_y * speed;

    // Sonar green or user colors
    let color = if options.particle_colors.is_empty() {
        [0.2, 0.9, 0.4, 1.0]
    } else {
        random_color(options)
    };

    // Small particles that form the ring
    let (size_min, size_max) = options.particle_size;
    let size = rand::rng().random_range(size_min * 0.3..size_max * 0.5).max(2.0);

    let lifetime = rand::rng().random_range(1.5..2.5) / options.speed;

    let mut particle = Particle::new(x, y)
        .with_size(size)
        .with_color(color)
        .with_velocity(vx, vy)
        .with_lifetime(lifetime);

    // Store initial angle for ring coherence
    particle.custom = pos;
    particle
}

/// Update a particle for the sonar pulse effect
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

    // Standard physics update
    particle.update(dt);

    // Keep particle outside the circle
    let px = particle.position.0 - cx;
    let py = particle.position.1 - cy;
    let pdist = (px * px + py * py).sqrt();
    let min_dist = circle_radius + 2.0;

    if pdist < min_dist && pdist > 0.001 {
        let push_x = px / pdist;
        let push_y = py / pdist;
        particle.position.0 = cx + push_x * min_dist;
        particle.position.1 = cy + push_y * min_dist;
    }

    // Pulse wave pattern - fade as it expands
    let age = particle.age();
    let wave = (time * 5.0 * options.speed).sin() * 0.5 + 0.5;
    particle.alpha = ((1.0 - age) * (0.5 + 0.5 * wave)).max(0.3);
}
