use crate::connection::execute;
use rusqlite::Result;

pub fn initialize_tables() -> Result<()> {
    execute(|conn| {
        conn.execute(
            "CREATE TABLE IF NOT EXISTS posts (
            id          INTEGER PRIMARY KEY AUTOINCREMENT,
            path        TEXT NOT NULL UNIQUE,
            content     TEXT NOT NULL,
            title       TEXT NOT NULL,
            spoiler     TEXT,
            created_at  TEXT NOT NULL,
            updated_at  TEXT NOT NULL
        )",
            (),
        )?;

        Ok(())
    })
}
