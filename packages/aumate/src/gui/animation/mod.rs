//! Animation system

mod easing;
mod types;

pub use easing::Easing;
pub use types::{AnimationDirection, AnimationState, AnimationType, WindowAnimation};

use std::time::Instant;

/// Animation controller
pub struct AnimationController {
    animation: WindowAnimation,
    start_time: Instant,
    state: AnimationState,
}

impl AnimationController {
    pub fn new(animation: WindowAnimation) -> Self {
        Self { animation, start_time: Instant::now(), state: AnimationState::Running }
    }

    /// Get the current progress (0.0 to 1.0)
    pub fn progress(&self) -> f32 {
        if self.state == AnimationState::Completed {
            return 1.0;
        }

        let elapsed = self.start_time.elapsed();
        let raw_progress = elapsed.as_secs_f32() / self.animation.duration.as_secs_f32();
        let clamped = raw_progress.clamp(0.0, 1.0);

        self.animation.easing.apply(clamped)
    }

    /// Check if animation is complete
    pub fn is_complete(&self) -> bool {
        self.start_time.elapsed() >= self.animation.duration
    }

    /// Update state and return true if animation just completed
    pub fn update(&mut self) -> bool {
        if self.state == AnimationState::Running && self.is_complete() {
            self.state = AnimationState::Completed;
            true
        } else {
            false
        }
    }

    /// Get the animation type
    pub fn animation_type(&self) -> AnimationType {
        self.animation.animation_type
    }

    /// Get the animation direction (for slide animations)
    pub fn direction(&self) -> AnimationDirection {
        self.animation.direction
    }

    /// Get animation state
    pub fn state(&self) -> AnimationState {
        self.state
    }
}
