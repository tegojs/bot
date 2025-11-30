//! Particle effect system

pub mod particle;
mod system;
pub mod presets;

pub use particle::{Particle, ParticleStyle};
pub use system::ParticleSystem;

/// Preset particle effects
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum PresetEffect {
    /// Particles orbit window edge forming a rotating halo
    #[default]
    RotatingHalo,
    /// Particles expand outward from edge in concentric waves
    PulseRipple,
    /// Particles flow along edge with brightness modulation
    FlowingLight,
    /// Random emission from edge with drift
    StardustScatter,
    /// Fast flickering particles with branching
    ElectricSpark,
    /// Slow rising particles with horizontal sway
    SmokeWisp,
    /// Vertical rain falling outside the circle
    RainDrop,
    /// Rotating laser lines emanating from circle edge
    LaserBeam,
    /// Electric arcs jumping around the circle perimeter
    LightningArc,
    /// Diagonal streaking meteors passing by the circle
    MeteorShower,
    /// Expanding ring pulses radiating from circle edge
    SonarPulse,
    /// Falling digital particles like Matrix movie
    MatrixRain,
    /// Flowing waves of color around the circle
    AuroraWave,
    /// Particles orbiting in rings around the circle
    OrbitRings,
    /// Rhythmic pulsing particles like a heartbeat
    HeartbeatPulse,
    /// Ethereal string-like trails weaving around the circle
    CosmicStrings,
    /// Smooth flowing ribbon bands waving around the circle
    SilkRibbon,
}

/// Options for preset effects
#[derive(Debug, Clone)]
pub struct PresetEffectOptions {
    /// Number of particles (None = auto based on window size)
    pub particle_count: Option<usize>,
    /// Particle size range
    pub particle_size: (f32, f32),
    /// Particle colors (RGBA)
    pub particle_colors: Vec<[f32; 4]>,
    /// Base speed multiplier
    pub speed: f32,
    /// Effect intensity (0.0 - 1.0)
    pub intensity: f32,
    /// Whether the effect loops
    pub loop_effect: bool,
    /// Edge width for particle generation
    pub edge_width: f32,
    /// Number of ribbons for SilkRibbon effect (default: 2)
    pub ribbon_count: usize,
    /// Petal amplitude for SilkRibbon effect (default: 20.0)
    pub petal_amplitude: f32,
}

impl Default for PresetEffectOptions {
    fn default() -> Self {
        Self {
            particle_count: None,
            particle_size: (5.0, 10.0), // Bigger particles for more visibility
            particle_colors: vec![
                [0.2, 0.9, 1.0, 1.0],  // Bright Cyan
                [1.0, 0.3, 0.8, 1.0],  // Hot Pink
                [0.5, 1.0, 0.3, 1.0],  // Lime Green
                [1.0, 0.8, 0.1, 1.0],  // Golden Yellow
            ],
            speed: 1.0,
            intensity: 1.0,
            loop_effect: true,
            edge_width: 10.0,
            ribbon_count: 2,
            petal_amplitude: 20.0,
        }
    }
}

impl PresetEffectOptions {
    pub fn with_colors(mut self, colors: Vec<[f32; 4]>) -> Self {
        self.particle_colors = colors;
        self
    }

    pub fn with_speed(mut self, speed: f32) -> Self {
        self.speed = speed;
        self
    }

    pub fn with_intensity(mut self, intensity: f32) -> Self {
        self.intensity = intensity.clamp(0.0, 1.0);
        self
    }

    pub fn with_particle_count(mut self, count: usize) -> Self {
        self.particle_count = Some(count);
        self
    }

    pub fn with_particle_size(mut self, min: f32, max: f32) -> Self {
        self.particle_size = (min, max);
        self
    }

    pub fn with_ribbon_count(mut self, count: usize) -> Self {
        self.ribbon_count = count.max(1);
        self
    }

    pub fn with_petal_amplitude(mut self, amplitude: f32) -> Self {
        self.petal_amplitude = amplitude.max(0.0);
        self
    }
}
