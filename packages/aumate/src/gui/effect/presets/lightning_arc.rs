//! LightningArc effect - electric arcs jumping around the circle perimeter
//! Lightning stays OUTSIDE the circle

use super::{circle_edge_outside, random_color};
use crate::gui::effect::PresetEffectOptions;
use crate::gui::effect::particle::Particle;
use rand::Rng;
use std::f32::consts::PI;

/// Spawn a particle for the lightning arc effect
pub fn spawn(pos: f32, options: &PresetEffectOptions, width: f32, height: f32) -> Particle {
    // Spawn at circle edge
    let gap = 4.0;
    let (x, y) = circle_edge_outside(pos, width, height, gap);

    // Lightning moves along the circle perimeter with random jumps
    let tangent_angle = pos * 2.0 * PI + PI / 2.0;
    let speed = rand::rng().random_range(50.0..100.0) * options.speed;
    let direction = if rand::rng().random::<bool>() { 1.0 } else { -1.0 };

    let vx = tangent_angle.cos() * speed * direction;
    let vy = tangent_angle.sin() * speed * direction;

    // Electric blue/white colors
    let color = if options.particle_colors.is_empty() {
        let intensity = rand::rng().random_range(0.8..1.0);
        [0.7 * intensity, 0.85 * intensity, 1.0, 1.0]
    } else {
        random_color(options)
    };

    // Small bright particles
    let (size_min, size_max) = options.particle_size;
    let size = rand::rng().random_range(size_min * 0.3..size_max * 0.6).max(2.0);

    let lifetime = rand::rng().random_range(0.1..0.3) / options.speed;

    let mut particle = Particle::new(x, y)
        .with_size(size)
        .with_color(color)
        .with_velocity(vx, vy)
        .with_lifetime(lifetime);

    // Store arc segment count
    particle.custom = pos;
    particle
}

/// Update a particle for the lightning arc effect
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

    // Random jitter for lightning effect
    let jitter_x: f32 = rand::rng().random_range(-30.0..30.0) * options.speed;
    let jitter_y: f32 = rand::rng().random_range(-30.0..30.0) * options.speed;

    particle.velocity.0 += jitter_x * dt;
    particle.velocity.1 += jitter_y * dt;

    // Standard physics update
    particle.update(dt);

    // Keep particle outside the circle but close to edge
    let px = particle.position.0 - cx;
    let py = particle.position.1 - cy;
    let pdist = (px * px + py * py).sqrt();
    let min_dist = circle_radius + 4.0;
    let max_dist = circle_radius + 25.0;

    if pdist < min_dist && pdist > 0.001 {
        let push_x = px / pdist;
        let push_y = py / pdist;
        particle.position.0 = cx + push_x * min_dist;
        particle.position.1 = cy + push_y * min_dist;
    } else if pdist > max_dist && pdist > 0.001 {
        let push_x = px / pdist;
        let push_y = py / pdist;
        particle.position.0 = cx + push_x * max_dist;
        particle.position.1 = cy + push_y * max_dist;
    }

    // Flickering alpha
    let flicker = (time * 50.0).sin().abs() * 0.5 + 0.5;
    particle.alpha = flicker.max(0.5);
}
