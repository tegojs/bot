//! RainDrop effect - vertical rain lines falling from top
//! Rain falls OUTSIDE the circle with wind sway

use super::random_color;
use crate::gui::effect::PresetEffectOptions;
use crate::gui::effect::particle::Particle;
use rand::Rng;

/// Spawn a particle for the rain drop effect
pub fn spawn(_pos: f32, options: &PresetEffectOptions, width: f32, height: f32) -> Particle {
    let cx = width / 2.0;
    let circle_radius = width.min(height) / 2.0;

    // Spawn above the window, spread across width but outside circle area
    let x = rand::rng().random_range(0.0..width);
    let y = -rand::rng().random_range(10.0..50.0); // Above window

    // Check if spawn point is inside circle (projected down)
    let dx = x - cx;
    // Skip spawning directly above the circle center area
    let in_circle_x = dx.abs() < circle_radius * 0.8;

    // If inside circle x-range, push to edges
    let final_x = if in_circle_x {
        if dx < 0.0 {
            cx - circle_radius - rand::rng().random_range(10.0..40.0)
        } else {
            cx + circle_radius + rand::rng().random_range(10.0..40.0)
        }
    } else {
        x
    };

    // Rain falls down with slight wind
    let speed = rand::rng().random_range(100.0..200.0) * options.speed;
    let wind = rand::rng().random_range(-20.0..20.0) * options.speed;

    // Rain color - blue-ish or user colors
    let color = if options.particle_colors.is_empty() {
        let blue = rand::rng().random_range(0.5..0.8);
        [0.4, 0.6, blue, 1.0]
    } else {
        random_color(options)
    };

    // Rain drops are elongated (using custom field for length)
    let (size_min, size_max) = options.particle_size;
    let size = rand::rng().random_range(size_min * 0.3..size_max * 0.5).max(2.0);

    let lifetime = rand::rng().random_range(1.0..2.0) / options.speed;

    let mut particle = Particle::new(final_x, y)
        .with_size(size)
        .with_color(color)
        .with_velocity(wind, speed)
        .with_lifetime(lifetime);

    // Store rain drop length in custom
    particle.custom = rand::rng().random_range(8.0..20.0);
    particle
}

/// Update a particle for the rain drop effect
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

    // Check collision with circle
    let px = particle.position.0 - cx;
    let py = particle.position.1 - cy;
    let dist = (px * px + py * py).sqrt();

    // If rain hits circle, kill it (splash effect would be separate)
    if dist < circle_radius + 5.0 {
        particle.lifetime = 0.0;
    }

    // Kill if below window
    if particle.position.1 > height + 20.0 {
        particle.lifetime = 0.0;
    }

    // Keep alpha high
    particle.alpha = 0.8;
}
