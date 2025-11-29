//! Particle struct and behavior

/// A single particle
#[derive(Debug, Clone)]
pub struct Particle {
    /// Position (x, y)
    pub position: (f32, f32),
    /// Velocity (vx, vy)
    pub velocity: (f32, f32),
    /// Acceleration (ax, ay)
    pub acceleration: (f32, f32),
    /// Color (RGBA, 0.0-1.0)
    pub color: [f32; 4],
    /// Size
    pub size: f32,
    /// Current alpha (for fade effects)
    pub alpha: f32,
    /// Lifetime remaining (seconds)
    pub lifetime: f32,
    /// Maximum lifetime (seconds)
    pub max_lifetime: f32,
    /// Angle (for rotating particles)
    pub angle: f32,
    /// Angular velocity
    pub angular_velocity: f32,
    /// Custom data for effect-specific use
    pub custom: f32,
}

impl Default for Particle {
    fn default() -> Self {
        Self {
            position: (0.0, 0.0),
            velocity: (0.0, 0.0),
            acceleration: (0.0, 0.0),
            color: [1.0, 1.0, 1.0, 1.0],
            size: 2.0,
            alpha: 1.0,
            lifetime: 1.0,
            max_lifetime: 1.0,
            angle: 0.0,
            angular_velocity: 0.0,
            custom: 0.0,
        }
    }
}

impl Particle {
    pub fn new(x: f32, y: f32) -> Self {
        Self {
            position: (x, y),
            ..Default::default()
        }
    }

    /// Update particle physics
    pub fn update(&mut self, dt: f32) {
        // Update velocity from acceleration
        self.velocity.0 += self.acceleration.0 * dt;
        self.velocity.1 += self.acceleration.1 * dt;

        // Update position from velocity
        self.position.0 += self.velocity.0 * dt;
        self.position.1 += self.velocity.1 * dt;

        // Update angle
        self.angle += self.angular_velocity * dt;

        // Update lifetime
        self.lifetime -= dt;
    }

    /// Check if particle is still alive
    pub fn is_alive(&self) -> bool {
        self.lifetime > 0.0
    }

    /// Get normalized age (0.0 = just born, 1.0 = about to die)
    pub fn age(&self) -> f32 {
        1.0 - (self.lifetime / self.max_lifetime).clamp(0.0, 1.0)
    }

    /// Set lifetime
    pub fn with_lifetime(mut self, lifetime: f32) -> Self {
        self.lifetime = lifetime;
        self.max_lifetime = lifetime;
        self
    }

    /// Set velocity
    pub fn with_velocity(mut self, vx: f32, vy: f32) -> Self {
        self.velocity = (vx, vy);
        self
    }

    /// Set color
    pub fn with_color(mut self, color: [f32; 4]) -> Self {
        self.color = color;
        self.alpha = color[3];
        self
    }

    /// Set size
    pub fn with_size(mut self, size: f32) -> Self {
        self.size = size;
        self
    }
}
