//! Async task helper for callback-based background operations

use std::sync::{Arc, Mutex};

/// Handle for tracking async task completion.
///
/// This provides a callback-based pattern for running operations in background
/// threads without blocking the UI. The UI can poll for results in its update loop.
///
/// # Example
/// ```ignore
/// let task = AsyncTask::new();
/// let callback = task.callback();
///
/// std::thread::spawn(move || {
///     let result = expensive_operation();
///     callback(result);
/// });
///
/// // Later, in UI update:
/// if let Some(result) = task.take() {
///     // Handle completed result
/// }
/// ```
pub struct AsyncTask<T> {
    result: Arc<Mutex<Option<T>>>,
    is_started: Arc<Mutex<bool>>,
}

impl<T> AsyncTask<T> {
    /// Create a new async task
    pub fn new() -> Self {
        Self { result: Arc::new(Mutex::new(None)), is_started: Arc::new(Mutex::new(false)) }
    }

    /// Get a callback that can be called from a background thread to deliver the result.
    /// The callback is Clone + Send, so it can be moved into threads.
    pub fn callback(&self) -> impl Fn(T) + Send + Clone + 'static
    where
        T: Send + 'static,
    {
        let result = self.result.clone();
        *self.is_started.lock().unwrap() = true;
        move |value| {
            *result.lock().unwrap() = Some(value);
        }
    }

    /// Check if the task has completed and take the result.
    /// Returns `Some(result)` if the task has completed, `None` otherwise.
    /// The result is consumed, so subsequent calls will return `None` until
    /// a new result is delivered.
    pub fn take(&self) -> Option<T> {
        self.result.lock().unwrap().take()
    }

    /// Check if the task has been started (callback was called)
    pub fn is_started(&self) -> bool {
        *self.is_started.lock().unwrap()
    }

    /// Check if a result is available without consuming it
    pub fn is_ready(&self) -> bool {
        self.result.lock().unwrap().is_some()
    }

    /// Check if the task is pending (started but no result yet)
    pub fn is_pending(&self) -> bool {
        self.is_started() && !self.is_ready()
    }
}

impl<T> Default for AsyncTask<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T> Clone for AsyncTask<T> {
    fn clone(&self) -> Self {
        Self { result: self.result.clone(), is_started: self.is_started.clone() }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_async_task_basic() {
        let task: AsyncTask<i32> = AsyncTask::new();
        assert!(!task.is_started());
        assert!(!task.is_ready());
        assert!(task.take().is_none());

        let callback = task.callback();
        assert!(task.is_started());
        assert!(!task.is_ready());
        assert!(task.is_pending());

        callback(42);
        assert!(task.is_ready());
        assert!(!task.is_pending());

        assert_eq!(task.take(), Some(42));
        assert!(task.take().is_none()); // Consumed
    }

    #[test]
    fn test_async_task_thread() {
        let task: AsyncTask<String> = AsyncTask::new();
        let callback = task.callback();

        std::thread::spawn(move || {
            std::thread::sleep(std::time::Duration::from_millis(10));
            callback("done".to_string());
        });

        // Wait for completion
        while !task.is_ready() {
            std::thread::sleep(std::time::Duration::from_millis(1));
        }

        assert_eq!(task.take(), Some("done".to_string()));
    }
}
