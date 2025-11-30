//! HeartbeatPulse effect - rhythmic pulsing particles like a heartbeat
//! Pulses expand OUTWARD from the circle

use super::{circle_edge_outside, outward_direction, random_color};
use crate::gui::effect::PresetEffectOptions;
use crate::gui::effect::particle::Particle;
use rand::Rng;
use std::f32::consts::PI;

/// Spawn a particle for the heartbeat pulse effect
pub fn spawn(pos: f32, options: &PresetEffectOptions, width: f32, height: f32) -> Particle {
    // Spawn at circle edge
    let gap = 2.0;
    let (x, y) = circle_edge_outside(pos, width, height, gap);

    // Get outward direction
    let (dir_x, dir_y) = outward_direction(pos);

    // Burst outward speed
    let speed = rand::rng().random_range(40.0..80.0) * options.speed;
    let vx = dir_x * speed;
    let vy = dir_y * speed;

    // Heartbeat red/pink colors
    let color = if options.particle_colors.is_empty() {
        let r = rand::rng().random_range(0.8..1.0);
        let g = rand::rng().random_range(0.2..0.4);
        let b = rand::rng().random_range(0.3..0.5);
        [r, g, b, 1.0]
    } else {
        random_color(options)
    };

    // Medium particles
    let (size_min, size_max) = options.particle_size;
    let size = rand::rng().random_range(size_min * 0.4..size_max * 0.6).max(3.0);

    let lifetime = rand::rng().random_range(0.8..1.2) / options.speed;

    let mut particle = Particle::new(x, y)
        .with_size(size)
        .with_color(color)
        .with_velocity(vx, vy)
        .with_lifetime(lifetime);

    // Store spawn phase for sync
    particle.custom = pos;
    particle
}

/// Update a particle for the heartbeat pulse effect
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

    // Heartbeat rhythm: two quick beats then pause
    // Pattern: beat-beat-pause (like "lub-dub")
    let cycle = (time * 1.5 * options.speed) % 1.0;
    let beat = if cycle < 0.15 {
        // First beat
        (cycle / 0.15 * PI).sin()
    } else if cycle < 0.3 {
        // Second beat
        ((cycle - 0.15) / 0.15 * PI).sin() * 0.7
    } else {
        // Pause
        0.0
    };

    let age = particle.age();
    particle.alpha = ((1.0 - age) * (0.4 + beat * 0.6)).max(0.3);
}
