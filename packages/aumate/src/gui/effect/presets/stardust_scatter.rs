//! Stardust Scatter effect - random emission from circle edge with outward drift
//! Particles spawn OUTSIDE the circle and scatter outward

use super::{circle_edge_outside, outward_direction, random_color};
use crate::gui::effect::PresetEffectOptions;
use crate::gui::effect::particle::Particle;
use rand::Rng;
/// Spawn a particle for the stardust scatter effect
pub fn spawn(pos: f32, options: &PresetEffectOptions, width: f32, height: f32) -> Particle {
    // Spawn OUTSIDE the circle
    let gap = 6.0;
    let (x, y) = circle_edge_outside(pos, width, height, gap);

    // Get outward direction and add some random spread
    let (out_x, out_y) = outward_direction(pos);

    // Random angle variation for scatter effect
    let angle_variation: f32 = rand::rng().random_range(-0.5..0.5);
    let cos_v = angle_variation.cos();
    let sin_v = angle_variation.sin();
    let dir_x = out_x * cos_v - out_y * sin_v;
    let dir_y = out_x * sin_v + out_y * cos_v;

    let speed = rand::rng().random_range(10.0..30.0) * options.speed;
    let vx = speed * dir_x;
    let vy = speed * dir_y;

    let color = random_color(options);

    // Random lifetime
    let lifetime = rand::rng().random_range(0.5..2.0) / options.speed;

    // Particle size
    let (size_min, size_max) = options.particle_size;
    let size = rand::rng().random_range(size_min * 0.4..size_max * 0.7).max(3.0);

    Particle::new(x, y)
        .with_size(size)
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
    width: f32,
    height: f32,
) {
    let cx = width / 2.0;
    let cy = height / 2.0;
    let circle_radius = width.min(height) / 2.0;

    // Apply slight deceleration (drag)
    let drag = 0.98;
    particle.velocity.0 *= drag;
    particle.velocity.1 *= drag;

    // Standard physics update
    particle.update(dt);

    // Keep particle outside the circle
    let px = particle.position.0 - cx;
    let py = particle.position.1 - cy;
    let pdist = (px * px + py * py).sqrt();
    let min_dist = circle_radius + particle.size * 0.5 + 4.0;

    if pdist < min_dist && pdist > 0.001 {
        let push_x = px / pdist;
        let push_y = py / pdist;
        particle.position.0 = cx + push_x * min_dist;
        particle.position.1 = cy + push_y * min_dist;
    }

    // Keep alpha high - fade only near end of life
    let life_ratio = (particle.lifetime / 0.3).min(1.0);

    // Twinkle effect
    let age = particle.age();
    let twinkle = (age * 20.0 * options.speed).sin().abs();
    particle.alpha = (life_ratio * (0.7 + 0.3 * twinkle)).max(0.3);
}
