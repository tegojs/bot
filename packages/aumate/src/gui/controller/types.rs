//! Controller types for dynamic tab system

/// Unique identifier for a controller tab
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct TabId(pub String);

impl TabId {
    pub fn new(id: impl Into<String>) -> Self {
        Self(id.into())
    }
}

impl From<&str> for TabId {
    fn from(s: &str) -> Self {
        Self(s.to_string())
    }
}

impl std::fmt::Display for TabId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// Information about a controller tab for navigation
#[derive(Debug, Clone)]
pub struct TabInfo {
    /// Unique identifier
    pub id: TabId,
    /// Display name
    pub name: String,
    /// Optional icon identifier
    pub icon: Option<String>,
    /// Display order (lower = earlier in tab bar)
    pub order: i32,
}

impl TabInfo {
    pub fn new(id: impl Into<String>, name: impl Into<String>, order: i32) -> Self {
        Self { id: TabId::new(id), name: name.into(), icon: None, order }
    }

    pub fn with_icon(mut self, icon: impl Into<String>) -> Self {
        self.icon = Some(icon.into());
        self
    }
}
