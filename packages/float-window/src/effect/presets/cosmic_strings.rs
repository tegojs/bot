//! CosmicStrings effect - ethereal string-like trails weaving around the circle
//! Strings stay OUTSIDE the circle

use super::{circle_edge_outside, random_color};
use crate::effect::particle::Particle;
use crate::effect::PresetEffectOptions;
use rand::Rng;
use std::f32::consts::PI;

/// Spawn a particle for the cosmic strings effect
pub fn spawn(pos: f32, options: &PresetEffectOptions, width: f32, height: f32) -> Particle {
    // Spawn at circle edge
    let gap = rand::rng().random_range(5.0..15.0);
    let (x, y) = circle_edge_outside(pos, width, height, gap);

    // Complex motion - mix of tangential and radial
    let angle = pos * 2.0 * PI;
    let tangent_x = -angle.sin();
    let tangent_y = angle.cos();
    let radial_x = angle.cos();
    let radial_y = angle.sin();

    // Weaving motion
    let tangent_speed = rand::rng().random_range(20.0..40.0) * options.speed;
    let radial_speed = rand::rng().random_range(-10.0..10.0) * options.speed;

    let vx = tangent_x * tangent_speed + radial_x * radial_speed;
    let vy = tangent_y * tangent_speed + radial_y * radial_speed;

    // Cosmic colors - deep purples, blues, with occasional bright stars
    let color = if options.particle_colors.is_empty() {
        let hue = rand::rng().random_range(0.0..1.0);
        if hue < 0.1 {
            // Occasional bright white
            [1.0, 1.0, 1.0, 1.0]
        } else {
            match ((hue - 0.1) * 4.0 / 0.9) as i32 {
                0 => [0.4, 0.2, 0.8, 1.0],  // Deep purple
                1 => [0.2, 0.3, 0.9, 1.0],  // Deep blue
                2 => [0.3, 0.6, 0.9, 1.0],  // Light blue
                _ => [0.6, 0.3, 0.7, 1.0],  // Violet
            }
        }
    } else {
        random_color(options)
    };

    // Thin string-like particles
    let (size_min, size_max) = options.particle_size;
    let size = rand::rng().random_range(size_min * 0.2..size_max * 0.4).max(2.0);

    let lifetime = rand::rng().random_range(2.0..4.0) / options.speed;

    let mut particle = Particle::new(x, y)
        .with_size(size)
        .with_color(color)
        .with_velocity(vx, vy)
        .with_lifetime(lifetime);

    // Store wave parameters
    particle.custom = rand::rng().random_range(0.0..PI * 2.0);
    particle
}

/// Update a particle for the cosmic strings effect
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

    // Add sinusoidal weaving to velocity
    let weave = (time * 3.0 * options.speed + particle.custom).sin() * 20.0;
    let px = particle.position.0 - cx;
    let py = particle.position.1 - cy;
    let pdist = (px * px + py * py).sqrt();

    if pdist > 0.001 {
        // Add perpendicular weaving force
        let tangent_x = -py / pdist;
        let tangent_y = px / pdist;
        particle.velocity.0 += tangent_x * weave * dt;
        particle.velocity.1 += tangent_y * weave * dt;
    }

    // Standard physics update
    particle.update(dt);

    // Keep particle outside the circle but not too far
    let px = particle.position.0 - cx;
    let py = particle.position.1 - cy;
    let pdist = (px * px + py * py).sqrt();
    let min_dist = circle_radius + 5.0;
    let max_dist = circle_radius + 50.0;

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

    // Twinkling alpha
    let twinkle = (time * 5.0 + particle.custom * 2.0).sin() * 0.2 + 0.8;
    let age = particle.age();
    particle.alpha = (twinkle * (1.0 - age * 0.3)).max(0.4);
}
