use rusqlite::Connection;

pub fn init_schema(conn: &Connection) -> rusqlite::Result<()> {
    conn.execute_batch(
        "
        -- =====================
        -- CHANNELS
        -- =====================
        CREATE TABLE IF NOT EXISTS channels (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            link TEXT NOT NULL UNIQUE,
            name TEXT NOT NULL,
            avatar TEXT NOT NULL,
            verified INTEGER NOT NULL CHECK (verified IN (0,1)),
            subscribers INTEGER,
            created_at TEXT NOT NULL DEFAULT (datetime('now')),
            updated_at TEXT NOT NULL DEFAULT (datetime('now'))
        );

        CREATE TRIGGER IF NOT EXISTS channels_updated_at
        AFTER UPDATE ON channels
        FOR EACH ROW
        BEGIN
            UPDATE channels
            SET updated_at = datetime('now')
            WHERE id = OLD.id;
        END;

        -- =====================
        -- VIDEOS
        -- =====================
        CREATE TABLE IF NOT EXISTS videos (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            channel_id INTEGER NOT NULL,
            link TEXT NOT NULL UNIQUE,
            title TEXT NOT NULL,
            thumbnail TEXT NOT NULL,
            duration INTEGER,
            views INTEGER,
            likes INTEGER,
            uploaded_at TEXT NOT NULL,
            status TEXT NOT NULL CHECK (
                status IN ('SAVED', 'TOWATCH', 'WATCHED')
            ),
            created_at TEXT NOT NULL DEFAULT (datetime('now')),
            updated_at TEXT NOT NULL DEFAULT (datetime('now')),
            watched_at TEXT,

            FOREIGN KEY (channel_id) REFERENCES channels(id)
                ON DELETE CASCADE
        );

        CREATE TRIGGER IF NOT EXISTS videos_updated_at
        AFTER UPDATE ON videos
        FOR EACH ROW
        BEGIN
            UPDATE videos
            SET updated_at = datetime('now')
            WHERE id = OLD.id;
        END;

        -- =====================
        -- TAGS
        -- =====================
        CREATE TABLE IF NOT EXISTS tags (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            name TEXT NOT NULL UNIQUE,
            color TEXT NOT NULL,
            created_at TEXT NOT NULL DEFAULT (datetime('now')),
            updated_at TEXT NOT NULL DEFAULT (datetime('now')),
            UNIQUE (name)
        );

        CREATE TRIGGER IF NOT EXISTS tags_updated_at
        AFTER UPDATE ON tags
        FOR EACH ROW
        BEGIN
            UPDATE tags
            SET updated_at = datetime('now')
            WHERE id = OLD.id;
        END;

        -- =====================
        -- VIDEO_TAGS (join table)
        -- =====================
        CREATE TABLE IF NOT EXISTS video_tags (
            video_id INTEGER NOT NULL,
            tag_id INTEGER NOT NULL,
            PRIMARY KEY (video_id, tag_id),
            FOREIGN KEY (video_id) REFERENCES videos(id) ON DELETE CASCADE,
            FOREIGN KEY (tag_id) REFERENCES tags(id) ON DELETE CASCADE
        );
        ",
    )
}
