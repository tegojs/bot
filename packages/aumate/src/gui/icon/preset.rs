//! Preset icon definitions

/// Preset icon names
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum IconName {
    // Window controls
    Close,
    Minimize,
    Maximize,

    // Navigation
    Home,
    Back,
    Forward,
    Refresh,

    // Actions
    Settings,
    Search,
    Download,
    Upload,
    Save,
    Edit,
    Delete,
    Add,
    Remove,

    // Status
    Check,
    Cancel,
    Info,
    Warning,
    Error,
    Success,

    // Media
    Play,
    Pause,
    Stop,
    Next,
    Previous,

    // Misc
    Menu,
    More,
    Pin,
    Unpin,
    Lock,
    Unlock,
}

impl IconName {
    /// Get the Unicode character for this icon (using common symbol characters)
    pub fn as_char(&self) -> char {
        match self {
            IconName::Close => '\u{2715}',    // ✕
            IconName::Minimize => '\u{2212}', // −
            IconName::Maximize => '\u{25A1}', // □
            IconName::Home => '\u{2302}',     // ⌂
            IconName::Back => '\u{2190}',     // ←
            IconName::Forward => '\u{2192}',  // →
            IconName::Refresh => '\u{21BB}',  // ↻
            IconName::Settings => '\u{2699}', // ⚙
            IconName::Search => '\u{1F50D}',  // 🔍
            IconName::Download => '\u{2193}', // ↓
            IconName::Upload => '\u{2191}',   // ↑
            IconName::Save => '\u{1F4BE}',    // 💾
            IconName::Edit => '\u{270E}',     // ✎
            IconName::Delete => '\u{1F5D1}',  // 🗑
            IconName::Add => '\u{002B}',      // +
            IconName::Remove => '\u{2212}',   // −
            IconName::Check => '\u{2713}',    // ✓
            IconName::Cancel => '\u{2717}',   // ✗
            IconName::Info => '\u{2139}',     // ℹ
            IconName::Warning => '\u{26A0}',  // ⚠
            IconName::Error => '\u{2718}',    // ✘
            IconName::Success => '\u{2714}',  // ✔
            IconName::Play => '\u{25B6}',     // ▶
            IconName::Pause => '\u{23F8}',    // ⏸
            IconName::Stop => '\u{25A0}',     // ■
            IconName::Next => '\u{23ED}',     // ⏭
            IconName::Previous => '\u{23EE}', // ⏮
            IconName::Menu => '\u{2630}',     // ☰
            IconName::More => '\u{22EE}',     // ⋮
            IconName::Pin => '\u{1F4CC}',     // 📌
            IconName::Unpin => '\u{1F4CC}',   // 📌
            IconName::Lock => '\u{1F512}',    // 🔒
            IconName::Unlock => '\u{1F513}',  // 🔓
        }
    }
}
