//! Particle system management

use super::particle::Particle;
use super::presets;
use super::{PresetEffect, PresetEffectOptions};
use std::time::Instant;

/// Particle system that manages all particles for an effect
pub struct ParticleSystem {
    particles: Vec<Particle>,
    effect: PresetEffect,
    options: PresetEffectOptions,
    width: f32,
    height: f32,
    last_update: Instant,
    time: f32,
    active: bool,
}

impl ParticleSystem {
    pub fn new(effect: PresetEffect, options: PresetEffectOptions, width: f32, height: f32) -> Self {
        let mut system = Self {
            particles: Vec::new(),
            effect,
            options,
            width,
            height,
            last_update: Instant::now(),
            time: 0.0,
            active: true,
        };
        system.initialize();
        system
    }

    /// Initialize particles based on the effect type
    fn initialize(&mut self) {
        let count = self
            .options
            .particle_count
            .unwrap_or_else(|| self.auto_particle_count());

        self.particles.clear();
        self.particles.reserve(count);

        for i in 0..count {
            let particle = self.spawn_particle(i as f32 / count as f32);
            self.particles.push(particle);
        }
    }

    /// Calculate automatic particle count based on window size
    fn auto_particle_count(&self) -> usize {
        let perimeter = 2.0 * (self.width + self.height);
        let base_count = (perimeter / 10.0) as usize;
        (base_count as f32 * self.options.intensity * 2.0) as usize
    }

    /// Spawn a new particle at the given position along the edge (0.0 - 1.0)
    fn spawn_particle(&self, edge_position: f32) -> Particle {
        match self.effect {
            PresetEffect::RotatingHalo => {
                presets::rotating_halo::spawn(edge_position, &self.options, self.width, self.height)
            }
            PresetEffect::PulseRipple => {
                presets::pulse_ripple::spawn(edge_position, &self.options, self.width, self.height)
            }
            PresetEffect::FlowingLight => {
                presets::flowing_light::spawn(edge_position, &self.options, self.width, self.height)
            }
            PresetEffect::StardustScatter => {
                presets::stardust_scatter::spawn(edge_position, &self.options, self.width, self.height)
            }
            PresetEffect::ElectricSpark => {
                presets::electric_spark::spawn(edge_position, &self.options, self.width, self.height)
            }
            PresetEffect::SmokeWisp => {
                presets::smoke_wisp::spawn(edge_position, &self.options, self.width, self.height)
            }
        }
    }

    /// Update all particles
    pub fn update(&mut self) {
        if !self.active {
            return;
        }

        let now = Instant::now();
        let dt = now.duration_since(self.last_update).as_secs_f32();
        self.last_update = now;
        self.time += dt;

        // Update each particle based on effect type
        for particle in &mut self.particles {
            match self.effect {
                PresetEffect::RotatingHalo => {
                    presets::rotating_halo::update(particle, dt, self.time, &self.options, self.width, self.height);
                }
                PresetEffect::PulseRipple => {
                    presets::pulse_ripple::update(particle, dt, self.time, &self.options, self.width, self.height);
                }
                PresetEffect::FlowingLight => {
                    presets::flowing_light::update(particle, dt, self.time, &self.options, self.width, self.height);
                }
                PresetEffect::StardustScatter => {
                    presets::stardust_scatter::update(particle, dt, self.time, &self.options, self.width, self.height);
                }
                PresetEffect::ElectricSpark => {
                    presets::electric_spark::update(particle, dt, self.time, &self.options, self.width, self.height);
                }
                PresetEffect::SmokeWisp => {
                    presets::smoke_wisp::update(particle, dt, self.time, &self.options, self.width, self.height);
                }
            }
        }

        // Respawn dead particles if looping
        if self.options.loop_effect {
            for i in 0..self.particles.len() {
                if !self.particles[i].is_alive() {
                    let edge_pos = rand::random::<f32>();
                    self.particles[i] = self.spawn_particle(edge_pos);
                }
            }
        } else {
            // Remove dead particles
            self.particles.retain(|p| p.is_alive());
        }
    }

    /// Get all particles for rendering
    pub fn particles(&self) -> &[Particle] {
        &self.particles
    }

    /// Set window dimensions
    pub fn set_size(&mut self, width: f32, height: f32) {
        self.width = width;
        self.height = height;
    }

    /// Start the effect
    pub fn start(&mut self) {
        self.active = true;
        self.last_update = Instant::now();
    }

    /// Stop the effect
    pub fn stop(&mut self) {
        self.active = false;
    }

    /// Check if effect is active
    pub fn is_active(&self) -> bool {
        self.active
    }

    /// Reset the effect
    pub fn reset(&mut self) {
        self.time = 0.0;
        self.last_update = Instant::now();
        self.initialize();
    }
}
