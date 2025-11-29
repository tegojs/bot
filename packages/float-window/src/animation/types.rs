//! Animation types

use super::Easing;
use std::time::Duration;

/// Animation type
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum AnimationType {
    /// Fade in/out
    #[default]
    Fade,
    /// Scale up/down
    Scale,
    /// Slide in/out
    Slide,
    /// Bounce effect
    Bounce,
    /// Rotate
    Rotate,
    /// Blink on/off
    Blink,
    /// No animation
    None,
}

/// Animation direction (for slide animations)
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum AnimationDirection {
    #[default]
    Up,
    Down,
    Left,
    Right,
}

/// Animation state
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AnimationState {
    Running,
    Completed,
}

/// Window animation configuration
#[derive(Debug, Clone)]
pub struct WindowAnimation {
    /// Type of animation
    pub animation_type: AnimationType,
    /// Duration of the animation
    pub duration: Duration,
    /// Easing function
    pub easing: Easing,
    /// Direction (for slide animations)
    pub direction: AnimationDirection,
}

impl Default for WindowAnimation {
    fn default() -> Self {
        Self {
            animation_type: AnimationType::Fade,
            duration: Duration::from_millis(200),
            easing: Easing::EaseOut,
            direction: AnimationDirection::Up,
        }
    }
}

impl WindowAnimation {
    /// Create a fade animation
    pub fn fade(duration: Duration) -> Self {
        Self {
            animation_type: AnimationType::Fade,
            duration,
            ..Default::default()
        }
    }

    /// Create a scale animation
    pub fn scale(duration: Duration) -> Self {
        Self {
            animation_type: AnimationType::Scale,
            duration,
            ..Default::default()
        }
    }

    /// Create a slide animation
    pub fn slide(direction: AnimationDirection, duration: Duration) -> Self {
        Self {
            animation_type: AnimationType::Slide,
            duration,
            direction,
            ..Default::default()
        }
    }

    /// Create a bounce animation
    pub fn bounce(duration: Duration) -> Self {
        Self {
            animation_type: AnimationType::Bounce,
            duration,
            easing: Easing::Bounce,
            ..Default::default()
        }
    }

    /// Set the easing function
    pub fn with_easing(mut self, easing: Easing) -> Self {
        self.easing = easing;
        self
    }
}
