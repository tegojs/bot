//! Particle effect system

mod particle;
mod system;
pub mod presets;

pub use particle::Particle;
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
}

impl Default for PresetEffectOptions {
    fn default() -> Self {
        Self {
            particle_count: None,
            particle_size: (2.0, 4.0),
            particle_colors: vec![
                [1.0, 1.0, 1.0, 1.0], // White
                [0.8, 0.9, 1.0, 1.0], // Light blue
            ],
            speed: 1.0,
            intensity: 0.5,
            loop_effect: true,
            edge_width: 10.0,
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
}
