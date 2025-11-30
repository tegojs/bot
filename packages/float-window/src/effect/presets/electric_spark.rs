//! Electric Spark effect - gentle flickering particles around circle
//! Particles spawn OUTSIDE the circle and drift slowly outward

use super::{circle_edge_outside, outward_direction, random_color};
use crate::effect::particle::Particle;
use crate::effect::PresetEffectOptions;
use rand::Rng;

/// Spawn a particle for the electric spark effect
pub fn spawn(pos: f32, options: &PresetEffectOptions, width: f32, height: f32) -> Particle {
    // Spawn OUTSIDE the circle with a small gap
    let gap = 6.0; // Fixed gap outside circle

    let (x, y) = circle_edge_outside(pos, width, height, gap);

    // Get outward direction and add some angular variation
    let (dir_x, dir_y) = outward_direction(pos);
    let angle_variation: f32 = rand::rng().random_range(-0.2..0.2);
    let cos_v = angle_variation.cos();
    let sin_v = angle_variation.sin();
    let varied_dir_x = dir_x * cos_v - dir_y * sin_v;
    let varied_dir_y = dir_x * sin_v + dir_y * cos_v;

    // Very slow speed - barely drifting outward
    let speed = rand::rng().random_range(1.0..3.0) * options.speed;
    let vx = speed * varied_dir_x;
    let vy = speed * varied_dir_y;

    // Always use provided colors - no white fallback
    let color = random_color(options);

    // Faster lifetime - particles last 1-2 seconds (3x faster)
    let lifetime = rand::rng().random_range(1.0..2.0);

    // Particle size - 40-80% of configured size, with minimum of 3.0
    let (size_min, size_max) = options.particle_size;
    let size = rand::rng().random_range(size_min * 0.4..size_max * 0.8).max(3.0);

    Particle::new(x, y)
        .with_size(size)
        .with_color(color)
        .with_velocity(vx, vy)
        .with_lifetime(lifetime)
}

/// Update a particle for the electric spark effect
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

    // Get direction from center to particle (outward)
    let dx = particle.position.0 - cx;
    let dy = particle.position.1 - cy;
    let dist = (dx * dx + dy * dy).sqrt().max(0.001);
    let outward_x = dx / dist;
    let outward_y = dy / dist;

    // Very gentle jitter
    let jitter_strength: f32 = 0.3 * options.speed;
    let jitter_x: f32 = rand::rng().random_range(-jitter_strength..jitter_strength);
    let jitter_y: f32 = rand::rng().random_range(-jitter_strength..jitter_strength);

    // Very gentle outward drift
    let outward_force = 0.5 * options.speed;
    particle.velocity.0 += jitter_x + outward_x * outward_force * dt;
    particle.velocity.1 += jitter_y + outward_y * outward_force * dt;

    // Heavy damping to keep motion very slow
    particle.velocity.0 *= 0.90;
    particle.velocity.1 *= 0.90;

    // Standard physics update
    particle.update(dt);

    // Keep particle outside the circle
    let px = particle.position.0 - cx;
    let py = particle.position.1 - cy;
    let pdist = (px * px + py * py).sqrt();
    let min_dist = circle_radius + particle.size * 0.5 + 4.0;

    if pdist < min_dist && pdist > 0.001 {
        // Push particle outside
        let push_x = px / pdist;
        let push_y = py / pdist;
        particle.position.0 = cx + push_x * min_dist;
        particle.position.1 = cy + push_y * min_dist;
    }

    // Keep alpha high - fade only near end of life
    let life_ratio = (particle.lifetime / 0.5).min(1.0); // Fade in last 0.5 seconds
    particle.alpha = life_ratio.max(0.3); // Minimum 0.3 alpha, no invisible particles
}
