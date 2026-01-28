use rusqlite::Connection;

use crate::db::{
    dto::{CreateTag, UpdateTag},
    queries,
    schema::Tag,
};

pub fn list_tags(conn: &Connection) -> Result<Vec<Tag>, String> {
    queries::list_tags(conn)
}

pub fn add_tag(conn: &Connection, tag: CreateTag) -> Result<Tag, String> {
    queries::create_tag(conn, tag)
}

pub fn edit_tag(conn: &Connection, id: i64, tag: UpdateTag) -> Result<Tag, String> {
    queries::update_tag(conn, id, tag)
}

pub fn delete_tag(conn: &Connection, id: i64) -> Result<(), String> {
    queries::delete_tag(conn, id)
}
