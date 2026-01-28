use rusqlite::{Connection, Error};
use std::path::PathBuf;

use crate::db::schema_init::init_schema;

pub fn open_db(path: PathBuf) -> rusqlite::Result<Connection> {
    std::fs::create_dir_all(&path).map_err(|_e| Error::ExecuteReturnedResults)?;

    let db_path = path.join("watchy.db");

    let conn = Connection::open(db_path)?;
    conn.execute("PRAGMA foreign_keys = ON", [])?;

    let _ = init_schema(&conn);

    Ok(conn)
}
