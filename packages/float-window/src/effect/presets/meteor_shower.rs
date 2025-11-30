//! MeteorShower effect - diagonal streaking lines with trails
//! Meteors pass by OUTSIDE the circle

use super::random_color;
use crate::effect::particle::Particle;
use crate::effect::PresetEffectOptions;
use rand::Rng;
use std::f32::consts::PI;

/// Spawn a particle for the meteor shower effect
pub fn spawn(_pos: f32, options: &PresetEffectOptions, width: f32, height: f32) -> Particle {

    // Spawn from top-right area, moving toward bottom-left
    let spawn_side = rand::rng().random_range(0.0..1.0);
    let (x, y) = if spawn_side < 0.5 {
        // From top
        (rand::rng().random_range(width * 0.3..width + 50.0), -rand::rng().random_range(10.0..50.0))
    } else {
        // From right
        (width + rand::rng().random_range(10.0..50.0), rand::rng().random_range(-50.0..height * 0.5))
    };

    // Diagonal velocity (top-right to bottom-left)
    let angle = rand::rng().random_range(PI * 0.6..PI * 0.8); // ~120-140 degrees
    let speed = rand::rng().random_range(150.0..300.0) * options.speed;
    let vx = angle.cos() * speed;
    let vy = -angle.sin() * speed; // Note: negative because y increases downward

    // Meteor colors - orange/yellow with some variation
    let color = if options.particle_colors.is_empty() {
        let r = rand::rng().random_range(0.9..1.0);
        let g = rand::rng().random_range(0.5..0.8);
        let b = rand::rng().random_range(0.1..0.3);
        [r, g, b, 1.0]
    } else {
        random_color(options)
    };

    // Medium sized
    let (size_min, size_max) = options.particle_size;
    let size = rand::rng().random_range(size_min * 0.4..size_max * 0.7).max(3.0);

    let lifetime = rand::rng().random_range(0.8..1.5) / options.speed;

    let mut particle = Particle::new(x, y)
        .with_size(size)
        .with_color(color)
        .with_velocity(vx, vy)
        .with_lifetime(lifetime);

    // Store trail length
    particle.custom = rand::rng().random_range(20.0..40.0);
    particle
}

/// Update a particle for the meteor shower effect
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

    // Check if meteor would hit circle - deflect around it
    let px = particle.position.0 - cx;
    let py = particle.position.1 - cy;
    let dist = (px * px + py * py).sqrt();

    if dist < circle_radius + 10.0 && dist > 0.001 {
        // Push meteor around the circle
        let push_x = px / dist;
        let push_y = py / dist;
        particle.position.0 = cx + push_x * (circle_radius + 12.0);
        particle.position.1 = cy + push_y * (circle_radius + 12.0);

        // Adjust velocity to curve around
        let tangent_x = -push_y;
        let tangent_y = push_x;
        let speed = (particle.velocity.0.powi(2) + particle.velocity.1.powi(2)).sqrt();
        particle.velocity.0 = tangent_x * speed * 0.8 + push_x * speed * 0.2;
        particle.velocity.1 = tangent_y * speed * 0.8 + push_y * speed * 0.2;
    }

    // Kill if off screen
    if particle.position.0 < -50.0 || particle.position.1 > height + 50.0 {
        particle.lifetime = 0.0;
    }

    // Bright alpha
    particle.alpha = 1.0;
}
