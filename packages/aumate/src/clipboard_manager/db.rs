//! SQLite database operations for clipboard history

use std::path::PathBuf;

use chrono::Utc;
use rusqlite::{Connection, params};
use uuid::Uuid;

use super::entry::{
    CategoryFilter, ClipboardContent, ClipboardEntry, ContentType, SensitiveDataType, Tag,
};

/// Maximum number of entries to keep in the database
const MAX_ENTRIES: usize = 500;

/// Clipboard database manager
pub struct ClipboardDb {
    conn: Connection,
}

impl ClipboardDb {
    /// Open or create the clipboard database
    ///
    /// The database is stored at `~/.aumate/clipboard.db`
    pub fn open() -> Result<Self, rusqlite::Error> {
        let db_path = Self::db_path();

        // Ensure directory exists
        if let Some(parent) = db_path.parent() {
            std::fs::create_dir_all(parent).ok();
        }

        let conn = Connection::open(&db_path)?;
        let db = Self { conn };
        db.init_schema()?;
        Ok(db)
    }

    /// Open an in-memory database (for testing)
    #[cfg(test)]
    pub fn open_in_memory() -> Result<Self, rusqlite::Error> {
        let conn = Connection::open_in_memory()?;
        let db = Self { conn };
        db.init_schema()?;
        Ok(db)
    }

    /// Get the database file path
    pub fn db_path() -> PathBuf {
        let home = dirs_next::home_dir().unwrap_or_else(|| PathBuf::from("."));
        home.join(".aumate").join("clipboard.db")
    }

    /// Initialize the database schema
    fn init_schema(&self) -> Result<(), rusqlite::Error> {
        self.conn.execute_batch(
            r#"
            CREATE TABLE IF NOT EXISTS clipboard_entries (
                id TEXT PRIMARY KEY,
                content_type TEXT NOT NULL,
                content_text TEXT,
                content_image BLOB,
                content_files TEXT,
                preview_text TEXT,
                hash TEXT NOT NULL,
                is_sensitive INTEGER DEFAULT 0,
                sensitive_type TEXT,
                is_favorite INTEGER DEFAULT 0,
                is_pinned INTEGER DEFAULT 0,
                created_at TEXT NOT NULL,
                accessed_at TEXT NOT NULL,
                access_count INTEGER DEFAULT 1,
                source_app TEXT
            );

            CREATE TABLE IF NOT EXISTS tags (
                id TEXT PRIMARY KEY,
                name TEXT UNIQUE NOT NULL,
                color TEXT DEFAULT '#808080'
            );

            CREATE TABLE IF NOT EXISTS entry_tags (
                entry_id TEXT NOT NULL,
                tag_id TEXT NOT NULL,
                PRIMARY KEY (entry_id, tag_id),
                FOREIGN KEY (entry_id) REFERENCES clipboard_entries(id) ON DELETE CASCADE,
                FOREIGN KEY (tag_id) REFERENCES tags(id) ON DELETE CASCADE
            );

            CREATE INDEX IF NOT EXISTS idx_entries_created_at ON clipboard_entries(created_at DESC);
            CREATE INDEX IF NOT EXISTS idx_entries_hash ON clipboard_entries(hash);
            CREATE INDEX IF NOT EXISTS idx_entries_content_type ON clipboard_entries(content_type);
            "#,
        )?;
        Ok(())
    }

    /// Insert a new clipboard entry
    ///
    /// Returns the entry ID if successful.
    /// If an entry with the same hash already exists, updates its access time instead.
    pub fn insert_entry(&self, entry: &ClipboardEntry) -> Result<String, rusqlite::Error> {
        // Check for duplicate by hash
        if let Some(existing_id) = self.find_by_hash(&entry.hash)? {
            // Update access time and count
            self.conn.execute(
                "UPDATE clipboard_entries SET accessed_at = ?, access_count = access_count + 1 WHERE id = ?",
                params![Utc::now().to_rfc3339(), existing_id],
            )?;
            return Ok(existing_id);
        }

        // Extract content fields
        let (content_text, content_image, content_files) = match &entry.content {
            ClipboardContent::Text(text) => (Some(text.clone()), None, None),
            ClipboardContent::Image { data, .. } => (None, Some(data.clone()), None),
            ClipboardContent::Files(files) => {
                (None, None, Some(serde_json::to_string(files).unwrap_or_default()))
            }
        };

        let sensitive_type = entry.sensitive_type.map(|t| t.as_str().to_string());

        self.conn.execute(
            r#"
            INSERT INTO clipboard_entries (
                id, content_type, content_text, content_image, content_files,
                preview_text, hash, is_sensitive, sensitive_type, is_favorite,
                is_pinned, created_at, accessed_at, access_count, source_app
            ) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
            "#,
            params![
                entry.id,
                entry.content_type.as_str(),
                content_text,
                content_image,
                content_files,
                entry.preview_text,
                entry.hash,
                entry.is_sensitive,
                sensitive_type,
                entry.is_favorite,
                entry.is_pinned,
                entry.created_at,
                entry.accessed_at,
                entry.access_count,
                entry.source_app,
            ],
        )?;

        // Enforce max entries limit
        self.purge_old_entries()?;

        Ok(entry.id.clone())
    }

    /// Find an entry by its content hash
    fn find_by_hash(&self, hash: &str) -> Result<Option<String>, rusqlite::Error> {
        let mut stmt = self.conn.prepare("SELECT id FROM clipboard_entries WHERE hash = ?")?;
        let mut rows = stmt.query(params![hash])?;
        if let Some(row) = rows.next()? { Ok(Some(row.get(0)?)) } else { Ok(None) }
    }

    /// Get all entries with optional filtering
    pub fn get_entries(
        &self,
        filter: CategoryFilter,
        search: Option<&str>,
        limit: usize,
        offset: usize,
    ) -> Result<Vec<ClipboardEntry>, rusqlite::Error> {
        let mut sql = String::from(
            r#"
            SELECT id, content_type, content_text, content_image, content_files,
                   preview_text, hash, is_sensitive, sensitive_type, is_favorite,
                   is_pinned, created_at, accessed_at, access_count, source_app
            FROM clipboard_entries
            WHERE 1=1
            "#,
        );

        // Apply category filter
        match filter {
            CategoryFilter::All => {}
            CategoryFilter::Text => sql.push_str(" AND content_type = 'text'"),
            CategoryFilter::Images => sql.push_str(" AND content_type = 'image'"),
            CategoryFilter::Files => sql.push_str(" AND content_type = 'files'"),
            CategoryFilter::Favorites => sql.push_str(" AND is_favorite = 1"),
            CategoryFilter::Sensitive => sql.push_str(" AND is_sensitive = 1"),
        }

        // Apply search filter
        if search.is_some() {
            sql.push_str(" AND (preview_text LIKE ? OR content_text LIKE ?)");
        }

        // Order by: pinned first, then by created_at desc
        sql.push_str(" ORDER BY is_pinned DESC, created_at DESC");
        sql.push_str(&format!(" LIMIT {} OFFSET {}", limit, offset));

        let mut stmt = self.conn.prepare(&sql)?;

        let mut entries = Vec::new();

        if let Some(search_term) = search {
            let pattern = format!("%{}%", search_term);
            let mut rows = stmt.query(params![pattern, pattern])?;
            while let Some(row) = rows.next()? {
                if let Ok(entry) = self.row_to_entry(row) {
                    entries.push(entry);
                }
            }
        } else {
            let mut rows = stmt.query([])?;
            while let Some(row) = rows.next()? {
                if let Ok(entry) = self.row_to_entry(row) {
                    entries.push(entry);
                }
            }
        };

        // Load tags for each entry
        for entry in &mut entries {
            entry.tags = self.get_entry_tags(&entry.id)?;
        }

        Ok(entries)
    }

    /// Convert a database row to a ClipboardEntry
    fn row_to_entry(&self, row: &rusqlite::Row) -> Result<ClipboardEntry, rusqlite::Error> {
        let id: String = row.get(0)?;
        let content_type_str: String = row.get(1)?;
        let content_text: Option<String> = row.get(2)?;
        let content_image: Option<Vec<u8>> = row.get(3)?;
        let content_files: Option<String> = row.get(4)?;
        let preview_text: String = row.get(5)?;
        let hash: String = row.get(6)?;
        let is_sensitive: bool = row.get(7)?;
        let sensitive_type_str: Option<String> = row.get(8)?;
        let is_favorite: bool = row.get(9)?;
        let is_pinned: bool = row.get(10)?;
        let created_at: String = row.get(11)?;
        let accessed_at: String = row.get(12)?;
        let access_count: u32 = row.get(13)?;
        let source_app: Option<String> = row.get(14)?;

        let content_type = ContentType::parse(&content_type_str).unwrap_or(ContentType::Text);

        let content = match content_type {
            ContentType::Text => ClipboardContent::Text(content_text.unwrap_or_default()),
            ContentType::Image => {
                // For image, we need to get dimensions from preview_text
                let (width, height) = parse_image_dimensions(&preview_text);
                ClipboardContent::Image { data: content_image.unwrap_or_default(), width, height }
            }
            ContentType::Files => {
                let files: Vec<String> =
                    content_files.and_then(|s| serde_json::from_str(&s).ok()).unwrap_or_default();
                ClipboardContent::Files(files)
            }
        };

        let sensitive_type = sensitive_type_str.and_then(|s| SensitiveDataType::parse(&s));

        Ok(ClipboardEntry {
            id,
            content_type,
            content,
            preview_text,
            hash,
            is_sensitive,
            sensitive_type,
            is_favorite,
            is_pinned,
            created_at,
            accessed_at,
            access_count,
            source_app,
            tags: Vec::new(), // Tags are loaded separately
        })
    }

    /// Get entry count
    pub fn count_entries(&self, filter: CategoryFilter) -> Result<usize, rusqlite::Error> {
        let sql = match filter {
            CategoryFilter::All => "SELECT COUNT(*) FROM clipboard_entries",
            CategoryFilter::Text => {
                "SELECT COUNT(*) FROM clipboard_entries WHERE content_type = 'text'"
            }
            CategoryFilter::Images => {
                "SELECT COUNT(*) FROM clipboard_entries WHERE content_type = 'image'"
            }
            CategoryFilter::Files => {
                "SELECT COUNT(*) FROM clipboard_entries WHERE content_type = 'files'"
            }
            CategoryFilter::Favorites => {
                "SELECT COUNT(*) FROM clipboard_entries WHERE is_favorite = 1"
            }
            CategoryFilter::Sensitive => {
                "SELECT COUNT(*) FROM clipboard_entries WHERE is_sensitive = 1"
            }
        };

        let count: i64 = self.conn.query_row(sql, [], |row| row.get(0))?;
        Ok(count as usize)
    }

    /// Delete an entry by ID
    pub fn delete_entry(&self, id: &str) -> Result<bool, rusqlite::Error> {
        let rows = self.conn.execute("DELETE FROM clipboard_entries WHERE id = ?", params![id])?;
        Ok(rows > 0)
    }

    /// Delete all entries
    pub fn clear_all(&self) -> Result<usize, rusqlite::Error> {
        let rows = self.conn.execute("DELETE FROM clipboard_entries", [])?;
        Ok(rows)
    }

    /// Toggle favorite status
    pub fn toggle_favorite(&self, id: &str) -> Result<bool, rusqlite::Error> {
        self.conn.execute(
            "UPDATE clipboard_entries SET is_favorite = NOT is_favorite WHERE id = ?",
            params![id],
        )?;

        let is_favorite: bool = self.conn.query_row(
            "SELECT is_favorite FROM clipboard_entries WHERE id = ?",
            params![id],
            |row| row.get(0),
        )?;

        Ok(is_favorite)
    }

    /// Toggle pinned status
    pub fn toggle_pinned(&self, id: &str) -> Result<bool, rusqlite::Error> {
        self.conn.execute(
            "UPDATE clipboard_entries SET is_pinned = NOT is_pinned WHERE id = ?",
            params![id],
        )?;

        let is_pinned: bool = self.conn.query_row(
            "SELECT is_pinned FROM clipboard_entries WHERE id = ?",
            params![id],
            |row| row.get(0),
        )?;

        Ok(is_pinned)
    }

    /// Remove old entries when exceeding the limit
    fn purge_old_entries(&self) -> Result<(), rusqlite::Error> {
        let count = self.count_entries(CategoryFilter::All)?;
        if count > MAX_ENTRIES {
            // Delete oldest non-pinned, non-favorite entries
            self.conn.execute(
                r#"
                DELETE FROM clipboard_entries WHERE id IN (
                    SELECT id FROM clipboard_entries
                    WHERE is_pinned = 0 AND is_favorite = 0
                    ORDER BY accessed_at ASC
                    LIMIT ?
                )
                "#,
                params![count - MAX_ENTRIES],
            )?;
        }
        Ok(())
    }

    // ==================== Tag Operations ====================

    /// Create a new tag
    pub fn create_tag(&self, name: &str, color: &str) -> Result<Tag, rusqlite::Error> {
        let id = Uuid::new_v4().to_string();
        self.conn.execute(
            "INSERT INTO tags (id, name, color) VALUES (?, ?, ?)",
            params![id, name, color],
        )?;
        Ok(Tag::new(id, name, color))
    }

    /// Get all tags
    pub fn get_all_tags(&self) -> Result<Vec<Tag>, rusqlite::Error> {
        let mut stmt = self.conn.prepare("SELECT id, name, color FROM tags ORDER BY name")?;
        let rows = stmt.query_map([], |row| {
            Ok(Tag { id: row.get(0)?, name: row.get(1)?, color: row.get(2)? })
        })?;

        let mut tags = Vec::new();
        for tag in rows.flatten() {
            tags.push(tag);
        }
        Ok(tags)
    }

    /// Get tags for an entry
    fn get_entry_tags(&self, entry_id: &str) -> Result<Vec<Tag>, rusqlite::Error> {
        let mut stmt = self.conn.prepare(
            r#"
            SELECT t.id, t.name, t.color
            FROM tags t
            JOIN entry_tags et ON t.id = et.tag_id
            WHERE et.entry_id = ?
            ORDER BY t.name
            "#,
        )?;

        let rows = stmt.query_map(params![entry_id], |row| {
            Ok(Tag { id: row.get(0)?, name: row.get(1)?, color: row.get(2)? })
        })?;

        let mut tags = Vec::new();
        for tag in rows.flatten() {
            tags.push(tag);
        }
        Ok(tags)
    }

    /// Add a tag to an entry
    pub fn add_tag_to_entry(&self, entry_id: &str, tag_id: &str) -> Result<(), rusqlite::Error> {
        self.conn.execute(
            "INSERT OR IGNORE INTO entry_tags (entry_id, tag_id) VALUES (?, ?)",
            params![entry_id, tag_id],
        )?;
        Ok(())
    }

    /// Remove a tag from an entry
    pub fn remove_tag_from_entry(
        &self,
        entry_id: &str,
        tag_id: &str,
    ) -> Result<(), rusqlite::Error> {
        self.conn.execute(
            "DELETE FROM entry_tags WHERE entry_id = ? AND tag_id = ?",
            params![entry_id, tag_id],
        )?;
        Ok(())
    }

    /// Delete a tag
    pub fn delete_tag(&self, tag_id: &str) -> Result<bool, rusqlite::Error> {
        let rows = self.conn.execute("DELETE FROM tags WHERE id = ?", params![tag_id])?;
        Ok(rows > 0)
    }

    /// Get an entry by ID
    pub fn get_entry(&self, id: &str) -> Result<Option<ClipboardEntry>, rusqlite::Error> {
        let mut stmt = self.conn.prepare(
            r#"
            SELECT id, content_type, content_text, content_image, content_files,
                   preview_text, hash, is_sensitive, sensitive_type, is_favorite,
                   is_pinned, created_at, accessed_at, access_count, source_app
            FROM clipboard_entries
            WHERE id = ?
            "#,
        )?;

        let mut rows = stmt.query(params![id])?;
        if let Some(row) = rows.next()? {
            let mut entry = self.row_to_entry(row)?;
            entry.tags = self.get_entry_tags(&entry.id)?;
            Ok(Some(entry))
        } else {
            Ok(None)
        }
    }
}

/// Parse image dimensions from preview text like "Image 1920x1080"
fn parse_image_dimensions(preview: &str) -> (u32, u32) {
    if let Some(rest) = preview.strip_prefix("Image ") {
        if let Some((w, h)) = rest.split_once('x') {
            if let (Ok(width), Ok(height)) = (w.parse(), h.parse()) {
                return (width, height);
            }
        }
    }
    (0, 0)
}

// Add dirs-next for home directory
mod dirs_next {
    use std::path::PathBuf;

    pub fn home_dir() -> Option<PathBuf> {
        #[cfg(target_os = "macos")]
        {
            std::env::var_os("HOME").map(PathBuf::from)
        }
        #[cfg(target_os = "windows")]
        {
            std::env::var_os("USERPROFILE").map(PathBuf::from)
        }
        #[cfg(target_os = "linux")]
        {
            std::env::var_os("HOME").map(PathBuf::from)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_db_open_and_insert() {
        let db = ClipboardDb::open_in_memory().unwrap();

        let entry = ClipboardEntry::new(
            Uuid::new_v4().to_string(),
            ClipboardContent::Text("Hello, World!".to_string()),
            "abc123",
            Utc::now().to_rfc3339(),
        );

        let id = db.insert_entry(&entry).unwrap();
        assert!(!id.is_empty());

        let count = db.count_entries(CategoryFilter::All).unwrap();
        assert_eq!(count, 1);
    }

    #[test]
    fn test_duplicate_detection() {
        let db = ClipboardDb::open_in_memory().unwrap();

        let entry1 = ClipboardEntry::new(
            Uuid::new_v4().to_string(),
            ClipboardContent::Text("Test".to_string()),
            "same_hash",
            Utc::now().to_rfc3339(),
        );

        let entry2 = ClipboardEntry::new(
            Uuid::new_v4().to_string(),
            ClipboardContent::Text("Test".to_string()),
            "same_hash",
            Utc::now().to_rfc3339(),
        );

        let id1 = db.insert_entry(&entry1).unwrap();
        let id2 = db.insert_entry(&entry2).unwrap();

        // Should return same ID (duplicate detected)
        assert_eq!(id1, id2);

        // Should still have only 1 entry
        let count = db.count_entries(CategoryFilter::All).unwrap();
        assert_eq!(count, 1);
    }

    #[test]
    fn test_tags() {
        let db = ClipboardDb::open_in_memory().unwrap();

        let tag = db.create_tag("work", "#FF5733").unwrap();
        assert_eq!(tag.name, "work");

        let tags = db.get_all_tags().unwrap();
        assert_eq!(tags.len(), 1);

        db.delete_tag(&tag.id).unwrap();
        let tags = db.get_all_tags().unwrap();
        assert_eq!(tags.len(), 0);
    }

    #[test]
    fn test_parse_image_dimensions() {
        assert_eq!(parse_image_dimensions("Image 1920x1080"), (1920, 1080));
        assert_eq!(parse_image_dimensions("Image 800x600"), (800, 600));
        assert_eq!(parse_image_dimensions("Invalid"), (0, 0));
    }
}
