//! MatrixRain effect - falling digital characters/particles like Matrix movie
//! Rain falls OUTSIDE the circle

use super::random_color;
use crate::effect::particle::Particle;
use crate::effect::PresetEffectOptions;
use rand::Rng;

/// Spawn a particle for the matrix rain effect
pub fn spawn(_pos: f32, options: &PresetEffectOptions, width: f32, height: f32) -> Particle {
    let cx = width / 2.0;
    let circle_radius = width.min(height) / 2.0;

    // Spawn in columns across the width, but avoid circle area
    let column_width = 15.0;
    let num_columns = (width / column_width) as i32;
    let column = rand::rng().random_range(0..num_columns);
    let x = column as f32 * column_width + column_width / 2.0;

    // Check if this column intersects with circle
    let dx = x - cx;
    let in_circle_x = dx.abs() < circle_radius + 10.0;

    // If in circle area, spawn above or we'll kill when approaching
    let y = if in_circle_x {
        -rand::rng().random_range(10.0..100.0)
    } else {
        -rand::rng().random_range(10.0..50.0)
    };

    // Fall straight down
    let speed = rand::rng().random_range(80.0..150.0) * options.speed;

    // Matrix green or user colors
    let color = if options.particle_colors.is_empty() {
        let intensity = rand::rng().random_range(0.5..1.0);
        [0.0, intensity, 0.0, 1.0]
    } else {
        random_color(options)
    };

    // Small square particles like characters
    let (size_min, size_max) = options.particle_size;
    let size = rand::rng().random_range(size_min * 0.3..size_max * 0.5).max(3.0);

    let lifetime = rand::rng().random_range(2.0..4.0) / options.speed;

    let mut particle = Particle::new(x, y)
        .with_size(size)
        .with_color(color)
        .with_velocity(0.0, speed)
        .with_lifetime(lifetime);

    // Store flicker timer
    particle.custom = rand::rng().random_range(0.0..1.0);
    particle
}

/// Update a particle for the matrix rain effect
pub fn update(
    particle: &mut Particle,
    dt: f32,
    time: f32,
    _options: &PresetEffectOptions,
    width: f32,
    height: f32,
) {
    let cx = width / 2.0;
    let cy = height / 2.0;
    let circle_radius = width.min(height) / 2.0;

    // Standard physics update
    particle.update(dt);

    // Check if particle would hit circle
    let px = particle.position.0 - cx;
    let py = particle.position.1 - cy;
    let dist = (px * px + py * py).sqrt();

    // Kill particles that hit the circle
    if dist < circle_radius + 5.0 {
        particle.lifetime = 0.0;
        return;
    }

    // Kill if below window
    if particle.position.1 > height + 20.0 {
        particle.lifetime = 0.0;
    }

    // Matrix flicker effect
    let flicker = (time * 10.0 + particle.custom * 20.0).sin() * 0.3 + 0.7;
    particle.alpha = flicker.max(0.4);
}
