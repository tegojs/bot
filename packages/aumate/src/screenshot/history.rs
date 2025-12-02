//! History management for undo/redo operations
//!
//! This module provides a snapshot-based undo/redo stack for screenshot annotations.

use super::stroke::AnnotationsSnapshot;

/// Maximum number of history entries to keep
const MAX_HISTORY_SIZE: usize = 50;

/// History manager for undo/redo
///
/// Uses a simple snapshot-based approach where each entry is a complete
/// copy of the annotations state. This is simpler and more reliable than
/// tracking individual operations.
pub struct History {
    /// Stack of snapshots that can be undone
    undo_stack: Vec<AnnotationsSnapshot>,
    /// Stack of snapshots that can be redone
    redo_stack: Vec<AnnotationsSnapshot>,
}

impl History {
    /// Create a new history manager
    pub fn new() -> Self {
        Self {
            undo_stack: Vec::with_capacity(MAX_HISTORY_SIZE),
            redo_stack: Vec::with_capacity(MAX_HISTORY_SIZE),
        }
    }

    /// Record a snapshot before a change
    ///
    /// Call this before making any change to annotations. The snapshot
    /// represents the state that will be restored if the user undoes
    /// the next action.
    ///
    /// This clears the redo stack since we're starting a new branch.
    pub fn record(&mut self, snapshot: AnnotationsSnapshot) {
        // Clear redo stack when new action is performed
        self.redo_stack.clear();

        // Add to undo stack
        self.undo_stack.push(snapshot);

        // Trim if over capacity
        if self.undo_stack.len() > MAX_HISTORY_SIZE {
            self.undo_stack.remove(0);
        }
    }

    /// Check if undo is available
    pub fn can_undo(&self) -> bool {
        !self.undo_stack.is_empty()
    }

    /// Check if redo is available
    pub fn can_redo(&self) -> bool {
        !self.redo_stack.is_empty()
    }

    /// Perform undo operation
    ///
    /// Takes the current annotations state and returns the previous state.
    /// The current state is saved for potential redo.
    pub fn undo(&mut self, current: AnnotationsSnapshot) -> Option<AnnotationsSnapshot> {
        let prev = self.undo_stack.pop()?;
        self.redo_stack.push(current);
        Some(prev)
    }

    /// Perform redo operation
    ///
    /// Takes the current annotations state and returns the next state.
    /// The current state is saved for potential undo.
    pub fn redo(&mut self, current: AnnotationsSnapshot) -> Option<AnnotationsSnapshot> {
        let next = self.redo_stack.pop()?;
        self.undo_stack.push(current);
        Some(next)
    }

    /// Clear all history
    pub fn clear(&mut self) {
        self.undo_stack.clear();
        self.redo_stack.clear();
    }

    /// Get the number of undo operations available
    pub fn undo_count(&self) -> usize {
        self.undo_stack.len()
    }

    /// Get the number of redo operations available
    pub fn redo_count(&self) -> usize {
        self.redo_stack.len()
    }
}

impl Default for History {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn empty_snapshot() -> AnnotationsSnapshot {
        AnnotationsSnapshot::default()
    }

    fn snapshot_with_marker(next_seq: u32) -> AnnotationsSnapshot {
        AnnotationsSnapshot {
            next_sequence: next_seq,
            ..Default::default()
        }
    }

    #[test]
    fn test_undo_redo() {
        let mut history = History::new();

        // Initially empty
        assert!(!history.can_undo());
        assert!(!history.can_redo());

        // Record initial state before first change
        history.record(empty_snapshot());
        // Record state before second change
        history.record(snapshot_with_marker(1));

        assert!(history.can_undo());
        assert!(!history.can_redo());
        assert_eq!(history.undo_count(), 2);

        // Undo one - pass current state (marker 2)
        let current = snapshot_with_marker(2);
        let prev = history.undo(current);
        assert!(prev.is_some());
        assert_eq!(prev.unwrap().next_sequence, 1);
        assert!(history.can_undo());
        assert!(history.can_redo());
        assert_eq!(history.undo_count(), 1);
        assert_eq!(history.redo_count(), 1);

        // Redo - pass current state (marker 1)
        let current = snapshot_with_marker(1);
        let next = history.redo(current);
        assert!(next.is_some());
        assert_eq!(next.unwrap().next_sequence, 2);
        assert_eq!(history.undo_count(), 2);
        assert_eq!(history.redo_count(), 0);

        // New action clears redo stack
        let current = snapshot_with_marker(2);
        history.undo(current);
        assert!(history.can_redo());
        history.record(snapshot_with_marker(1));
        assert!(!history.can_redo());
    }

    #[test]
    fn test_max_history_size() {
        let mut history = History::new();

        // Add more than MAX_HISTORY_SIZE entries
        for i in 0..MAX_HISTORY_SIZE + 10 {
            history.record(snapshot_with_marker(i as u32));
        }

        // Should be capped at MAX_HISTORY_SIZE
        assert_eq!(history.undo_count(), MAX_HISTORY_SIZE);
    }
}
