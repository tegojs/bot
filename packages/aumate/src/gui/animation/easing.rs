//! Easing functions

use std::f32::consts::PI;

/// Easing function type
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum Easing {
    /// Linear interpolation
    Linear,
    /// Ease in (slow start)
    EaseIn,
    /// Ease out (slow end)
    #[default]
    EaseOut,
    /// Ease in and out
    EaseInOut,
    /// Bounce effect
    Bounce,
    /// Elastic effect
    Elastic,
}

impl Easing {
    /// Apply the easing function to a progress value (0.0 to 1.0)
    pub fn apply(&self, t: f32) -> f32 {
        match self {
            Easing::Linear => t,
            Easing::EaseIn => ease_in_quad(t),
            Easing::EaseOut => ease_out_quad(t),
            Easing::EaseInOut => ease_in_out_quad(t),
            Easing::Bounce => ease_out_bounce(t),
            Easing::Elastic => ease_out_elastic(t),
        }
    }
}

/// Quadratic ease in
fn ease_in_quad(t: f32) -> f32 {
    t * t
}

/// Quadratic ease out
fn ease_out_quad(t: f32) -> f32 {
    1.0 - (1.0 - t) * (1.0 - t)
}

/// Quadratic ease in/out
fn ease_in_out_quad(t: f32) -> f32 {
    if t < 0.5 { 2.0 * t * t } else { 1.0 - (-2.0 * t + 2.0).powi(2) / 2.0 }
}

/// Bounce ease out
fn ease_out_bounce(t: f32) -> f32 {
    const N1: f32 = 7.5625;
    const D1: f32 = 2.75;

    if t < 1.0 / D1 {
        N1 * t * t
    } else if t < 2.0 / D1 {
        let t = t - 1.5 / D1;
        N1 * t * t + 0.75
    } else if t < 2.5 / D1 {
        let t = t - 2.25 / D1;
        N1 * t * t + 0.9375
    } else {
        let t = t - 2.625 / D1;
        N1 * t * t + 0.984375
    }
}

/// Elastic ease out
fn ease_out_elastic(t: f32) -> f32 {
    const C4: f32 = (2.0 * PI) / 3.0;

    if t == 0.0 {
        0.0
    } else if t == 1.0 {
        1.0
    } else {
        2.0_f32.powf(-10.0 * t) * ((t * 10.0 - 0.75) * C4).sin() + 1.0
    }
}
