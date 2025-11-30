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
    /// Delay before effect starts (in seconds)
    start_delay: f32,
    /// Time elapsed since system creation
    elapsed_since_creation: f32,
    /// Whether particles have been initialized after delay
    initialized: bool,
}

/// Default delay before effect starts (500ms)
const DEFAULT_START_DELAY: f32 = 0.5;

impl ParticleSystem {
    pub fn new(effect: PresetEffect, options: PresetEffectOptions, width: f32, height: f32) -> Self {
        Self::with_delay(effect, options, width, height, DEFAULT_START_DELAY)
    }

    /// Create a particle system with a custom start delay
    pub fn with_delay(effect: PresetEffect, options: PresetEffectOptions, width: f32, height: f32, delay: f32) -> Self {
        Self {
            particles: Vec::new(),
            effect,
            options,
            width,
            height,
            last_update: Instant::now(),
            time: 0.0,
            active: true,
            start_delay: delay,
            elapsed_since_creation: 0.0,
            initialized: false,
        }
    }

    /// Create a particle system with no delay (starts immediately)
    pub fn immediate(effect: PresetEffect, options: PresetEffectOptions, width: f32, height: f32) -> Self {
        let mut system = Self::with_delay(effect, options, width, height, 0.0);
        system.initialize();
        system.initialized = true;
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

    /// Calculate automatic particle count based on window size and effect type
    fn auto_particle_count(&self) -> usize {
        match self.effect {
            PresetEffect::RotatingHalo => {
                // More particles for a dense glowing halo
                let circumference = std::f32::consts::PI * self.width.min(self.height);
                let base_count = (circumference / 5.0) as usize; // One particle every 5 pixels
                ((base_count as f32 * self.options.intensity * 3.0) as usize).max(30)
            }
            PresetEffect::ElectricSpark => {
                // More particles - 1.5x more than before
                let circumference = std::f32::consts::PI * self.width.min(self.height);
                let base_count = (circumference / 10.0) as usize; // One particle every 10 pixels
                ((base_count as f32 * self.options.intensity * 1.5) as usize).max(15).min(60)
            }
            PresetEffect::SilkRibbon => {
                // 120 segments per ribbon × ribbon_count
                120 * self.options.ribbon_count.max(1)
            }
            _ => {
                let perimeter = 2.0 * (self.width + self.height);
                let base_count = (perimeter / 10.0) as usize;
                (base_count as f32 * self.options.intensity * 2.0) as usize
            }
        }
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
            PresetEffect::RainDrop => {
                presets::rain_drop::spawn(edge_position, &self.options, self.width, self.height)
            }
            PresetEffect::LaserBeam => {
                presets::laser_beam::spawn(edge_position, &self.options, self.width, self.height)
            }
            PresetEffect::LightningArc => {
                presets::lightning_arc::spawn(edge_position, &self.options, self.width, self.height)
            }
            PresetEffect::MeteorShower => {
                presets::meteor_shower::spawn(edge_position, &self.options, self.width, self.height)
            }
            PresetEffect::SonarPulse => {
                presets::sonar_pulse::spawn(edge_position, &self.options, self.width, self.height)
            }
            PresetEffect::MatrixRain => {
                presets::matrix_rain::spawn(edge_position, &self.options, self.width, self.height)
            }
            PresetEffect::AuroraWave => {
                presets::aurora_wave::spawn(edge_position, &self.options, self.width, self.height)
            }
            PresetEffect::OrbitRings => {
                presets::orbit_rings::spawn(edge_position, &self.options, self.width, self.height)
            }
            PresetEffect::HeartbeatPulse => {
                presets::heartbeat_pulse::spawn(edge_position, &self.options, self.width, self.height)
            }
            PresetEffect::CosmicStrings => {
                presets::cosmic_strings::spawn(edge_position, &self.options, self.width, self.height)
            }
            PresetEffect::SilkRibbon => {
                presets::silk_ribbon::spawn(edge_position, &self.options, self.width, self.height)
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

        // Track time since creation for delay
        self.elapsed_since_creation += dt;

        // Check if we should initialize particles after delay
        if !self.initialized && self.elapsed_since_creation >= self.start_delay {
            self.initialize();
            self.initialized = true;
        }

        // Don't update particles until initialized
        if !self.initialized {
            return;
        }

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
                PresetEffect::RainDrop => {
                    presets::rain_drop::update(particle, dt, self.time, &self.options, self.width, self.height);
                }
                PresetEffect::LaserBeam => {
                    presets::laser_beam::update(particle, dt, self.time, &self.options, self.width, self.height);
                }
                PresetEffect::LightningArc => {
                    presets::lightning_arc::update(particle, dt, self.time, &self.options, self.width, self.height);
                }
                PresetEffect::MeteorShower => {
                    presets::meteor_shower::update(particle, dt, self.time, &self.options, self.width, self.height);
                }
                PresetEffect::SonarPulse => {
                    presets::sonar_pulse::update(particle, dt, self.time, &self.options, self.width, self.height);
                }
                PresetEffect::MatrixRain => {
                    presets::matrix_rain::update(particle, dt, self.time, &self.options, self.width, self.height);
                }
                PresetEffect::AuroraWave => {
                    presets::aurora_wave::update(particle, dt, self.time, &self.options, self.width, self.height);
                }
                PresetEffect::OrbitRings => {
                    presets::orbit_rings::update(particle, dt, self.time, &self.options, self.width, self.height);
                }
                PresetEffect::HeartbeatPulse => {
                    presets::heartbeat_pulse::update(particle, dt, self.time, &self.options, self.width, self.height);
                }
                PresetEffect::CosmicStrings => {
                    presets::cosmic_strings::update(particle, dt, self.time, &self.options, self.width, self.height);
                }
                PresetEffect::SilkRibbon => {
                    presets::silk_ribbon::update(particle, dt, self.time, &self.options, self.width, self.height);
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
        self.elapsed_since_creation = 0.0;
        self.initialized = false;
        self.last_update = Instant::now();
        self.particles.clear();
    }

    /// Set the start delay
    pub fn set_delay(&mut self, delay: f32) {
        self.start_delay = delay;
    }

    /// Get the start delay
    pub fn delay(&self) -> f32 {
        self.start_delay
    }

    /// Check if the effect has started (after delay)
    pub fn has_started(&self) -> bool {
        self.initialized
    }
}
