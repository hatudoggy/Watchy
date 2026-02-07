use rusqlite::Connection;
use watchy_lib::{
    db::{
        schema::{Channel, Database, Tag, Video, VideoStatus, VideoTag},
        schema_init::init_schema,
    },
    services::settings::{export_db_data, import_db_data},
};

fn setup_test_db() -> Connection {
    let conn = Connection::open_in_memory().expect("Failed to create in-memory database");
    init_schema(&conn).expect("Failed to initialize schema");
    conn
}

fn setup_db_with_data(conn: &Connection) {
    // Insert channels
    conn.execute(
        "INSERT INTO channels (id, link, name, avatar, verified, subscribers) 
         VALUES (1, 'https://youtube.com/@channel1', 'Channel 1', 'avatar1.jpg', 1, 10000)",
        [],
    )
    .unwrap();

    conn.execute(
        "INSERT INTO channels (id, link, name, avatar, verified, subscribers) 
         VALUES (2, 'https://youtube.com/@channel2', 'Channel 2', 'avatar2.jpg', 0, 5000)",
        [],
    )
    .unwrap();

    // Insert videos
    conn.execute(
        "INSERT INTO videos (id, channel_id, link, title, thumbnail, duration, views, likes, uploaded_at, status) 
         VALUES (1, 1, 'https://youtube.com/watch?v=1', 'Video 1', 'thumb1.jpg', 300, 1000, 50, '2026-01-01', 'SAVED')",
        [],
    )
    .unwrap();

    conn.execute(
        "INSERT INTO videos (id, channel_id, link, title, thumbnail, duration, views, likes, uploaded_at, status) 
         VALUES (2, 2, 'https://youtube.com/watch?v=2', 'Video 2', 'thumb2.jpg', 600, 2000, 100, '2026-01-02', 'WATCHED')",
        [],
    )
    .unwrap();

    // Insert tags
    conn.execute(
        "INSERT INTO tags (id, name, color) VALUES (1, 'Tutorial', '#FF0000')",
        [],
    )
    .unwrap();

    conn.execute(
        "INSERT INTO tags (id, name, color) VALUES (2, 'Review', '#00FF00')",
        [],
    )
    .unwrap();

    // Insert video_tags
    conn.execute(
        "INSERT INTO video_tags (video_id, tag_id) VALUES (1, 1)",
        [],
    )
    .unwrap();

    conn.execute(
        "INSERT INTO video_tags (video_id, tag_id) VALUES (1, 2)",
        [],
    )
    .unwrap();

    conn.execute(
        "INSERT INTO video_tags (video_id, tag_id) VALUES (2, 2)",
        [],
    )
    .unwrap();
}

// =====================
// EXPORT TESTS
// =====================

#[test]
fn test_export_db_data_empty_database() {
    let conn = setup_test_db();

    let result = export_db_data(&conn);
    assert!(result.is_ok());

    let database = result.unwrap();
    assert_eq!(database.videos.len(), 0);
    assert_eq!(database.channels.len(), 0);
    assert_eq!(database.tags.len(), 0);
    assert_eq!(database.video_tags.len(), 0);
}

#[test]
fn test_export_db_data_with_data() {
    let conn = setup_test_db();
    setup_db_with_data(&conn);

    let result = export_db_data(&conn);
    assert!(result.is_ok());

    let database = result.unwrap();
    assert_eq!(database.channels.len(), 2);
    assert_eq!(database.videos.len(), 2);
    assert_eq!(database.tags.len(), 2);
    assert_eq!(database.video_tags.len(), 3);

    // Verify channel data
    assert_eq!(database.channels[0].name, "Channel 1");
    assert_eq!(database.channels[1].name, "Channel 2");

    // Verify video data
    assert_eq!(database.videos[0].title, "Video 1");
    assert_eq!(database.videos[1].title, "Video 2");

    // Verify tag data
    assert_eq!(database.tags[0].name, "Tutorial");
    assert_eq!(database.tags[1].name, "Review");

    // Verify video_tags relationships
    assert_eq!(database.video_tags[0].video_id, 1);
    assert_eq!(database.video_tags[0].tag_id, 1);
}

#[test]
fn test_export_db_data_preserves_all_fields() {
    let conn = setup_test_db();
    setup_db_with_data(&conn);

    let database = export_db_data(&conn).unwrap();

    // Verify complete video data
    let video = &database.videos[0];
    assert_eq!(video.id, 1);
    assert_eq!(video.channel_id, 1);
    assert_eq!(video.link, "https://youtube.com/watch?v=1");
    assert_eq!(video.title, "Video 1");
    assert_eq!(video.thumbnail, "thumb1.jpg");
    assert_eq!(video.duration, Some(300));
    assert_eq!(video.views, Some(1000));
    assert_eq!(video.likes, Some(50));
    assert_eq!(video.uploaded_at, "2026-01-01");
    assert!(matches!(video.status, VideoStatus::SAVED));

    // Verify complete channel data
    let channel = &database.channels[0];
    assert_eq!(channel.id, 1);
    assert_eq!(channel.link, "https://youtube.com/@channel1");
    assert_eq!(channel.name, "Channel 1");
    assert_eq!(channel.avatar, "avatar1.jpg");
    assert_eq!(channel.verified, true);
    assert_eq!(channel.subscribers, Some(10000));

    // Verify complete tag data
    let tag = &database.tags[0];
    assert_eq!(tag.id, 1);
    assert_eq!(tag.name, "Tutorial");
    assert_eq!(tag.color, "#FF0000");
}

// =====================
// IMPORT TESTS
// =====================

#[test]
fn test_import_db_data_into_empty_database() {
    let mut conn = setup_test_db();

    let database = Database {
        channels: vec![Channel {
            id: 1,
            link: "https://youtube.com/@test".to_string(),
            name: "Test Channel".to_string(),
            avatar: "avatar.jpg".to_string(),
            verified: true,
            subscribers: Some(10000),
            created_at: "2026-01-01 00:00:00".to_string(),
            updated_at: "2026-01-01 00:00:00".to_string(),
        }],
        videos: vec![Video {
            id: 1,
            channel_id: 1,
            link: "https://youtube.com/watch?v=1".to_string(),
            title: "Test Video".to_string(),
            thumbnail: "thumb.jpg".to_string(),
            duration: Some(300),
            views: Some(1000),
            likes: Some(50),
            uploaded_at: "2026-01-01".to_string(),
            status: VideoStatus::SAVED,
            created_at: "2026-01-01 00:00:00".to_string(),
            updated_at: "2026-01-01 00:00:00".to_string(),
            watched_at: None,
        }],
        tags: vec![Tag {
            id: 1,
            name: "Tutorial".to_string(),
            color: "#FF0000".to_string(),
            created_at: "2026-01-01 00:00:00".to_string(),
            updated_at: "2026-01-01 00:00:00".to_string(),
        }],
        video_tags: vec![VideoTag {
            video_id: 1,
            tag_id: 1,
        }],
    };

    let result = import_db_data(&mut conn, database);
    assert!(result.is_ok());

    // Verify imported data
    let exported = export_db_data(&conn).unwrap();
    assert_eq!(exported.channels.len(), 1);
    assert_eq!(exported.videos.len(), 1);
    assert_eq!(exported.tags.len(), 1);
    assert_eq!(exported.video_tags.len(), 1);

    assert_eq!(exported.channels[0].name, "Test Channel");
    assert_eq!(exported.videos[0].title, "Test Video");
    assert_eq!(exported.tags[0].name, "Tutorial");
}

#[test]
fn test_import_db_data_replaces_existing_data() {
    let mut conn = setup_test_db();
    setup_db_with_data(&conn);

    // Verify initial data exists
    let initial = export_db_data(&conn).unwrap();
    assert_eq!(initial.channels.len(), 2);
    assert_eq!(initial.videos.len(), 2);

    // Import new data (should clear and replace)
    let database = Database {
        channels: vec![Channel {
            id: 99,
            link: "https://youtube.com/@newchannel".to_string(),
            name: "New Channel".to_string(),
            avatar: "new_avatar.jpg".to_string(),
            verified: false,
            subscribers: Some(500),
            created_at: "2026-02-01 00:00:00".to_string(),
            updated_at: "2026-02-01 00:00:00".to_string(),
        }],
        videos: vec![Video {
            id: 99,
            channel_id: 99,
            link: "https://youtube.com/watch?v=99".to_string(),
            title: "New Video".to_string(),
            thumbnail: "new_thumb.jpg".to_string(),
            duration: Some(120),
            views: Some(100),
            likes: Some(10),
            uploaded_at: "2026-02-01".to_string(),
            status: VideoStatus::TOWATCH,
            created_at: "2026-02-01 00:00:00".to_string(),
            updated_at: "2026-02-01 00:00:00".to_string(),
            watched_at: None,
        }],
        tags: vec![Tag {
            id: 99,
            name: "New Tag".to_string(),
            color: "#0000FF".to_string(),
            created_at: "2026-02-01 00:00:00".to_string(),
            updated_at: "2026-02-01 00:00:00".to_string(),
        }],
        video_tags: vec![VideoTag {
            video_id: 99,
            tag_id: 99,
        }],
    };

    let result = import_db_data(&mut conn, database);
    assert!(result.is_ok());

    // Verify old data is gone and new data exists
    let exported = export_db_data(&conn).unwrap();
    assert_eq!(exported.channels.len(), 1);
    assert_eq!(exported.videos.len(), 1);
    assert_eq!(exported.tags.len(), 1);
    assert_eq!(exported.video_tags.len(), 1);

    assert_eq!(exported.channels[0].name, "New Channel");
    assert_eq!(exported.videos[0].title, "New Video");
    assert_eq!(exported.tags[0].name, "New Tag");
}

#[test]
fn test_import_db_data_empty_import() {
    let mut conn = setup_test_db();
    setup_db_with_data(&conn);

    let database = Database {
        channels: vec![],
        videos: vec![],
        tags: vec![],
        video_tags: vec![],
    };

    let result = import_db_data(&mut conn, database);
    assert!(result.is_ok());

    // All tables should be empty after importing empty data
    let exported = export_db_data(&conn).unwrap();
    assert_eq!(exported.channels.len(), 0);
    assert_eq!(exported.videos.len(), 0);
    assert_eq!(exported.tags.len(), 0);
    assert_eq!(exported.video_tags.len(), 0);
}

// =====================
// ROUND-TRIP TESTS
// =====================

#[test]
fn test_export_import_round_trip() {
    let conn1 = setup_test_db();
    let mut conn2 = setup_test_db();

    setup_db_with_data(&conn1);

    // Export from first database
    let exported_data = export_db_data(&conn1).unwrap();

    // Import into second database
    let result = import_db_data(&mut conn2, exported_data);
    assert!(result.is_ok());

    // Export from second database
    let re_exported_data = export_db_data(&conn2).unwrap();

    // Verify data matches
    assert_eq!(re_exported_data.channels.len(), 2);
    assert_eq!(re_exported_data.videos.len(), 2);
    assert_eq!(re_exported_data.tags.len(), 2);
    assert_eq!(re_exported_data.video_tags.len(), 3);

    // Verify specific data integrity
    assert_eq!(re_exported_data.channels[0].name, "Channel 1");
    assert_eq!(re_exported_data.videos[0].title, "Video 1");
    assert_eq!(re_exported_data.tags[0].name, "Tutorial");
}

#[test]
fn test_export_import_maintains_relationships() {
    let conn1 = setup_test_db();
    let mut conn2 = setup_test_db();

    setup_db_with_data(&conn1);

    // Export and import
    let exported_data = export_db_data(&conn1).unwrap();
    import_db_data(&mut conn2, exported_data).unwrap();

    // Verify relationships are maintained
    let re_exported = export_db_data(&conn2).unwrap();

    // Check video-channel relationships
    assert_eq!(re_exported.videos[0].channel_id, 1);
    assert_eq!(re_exported.videos[1].channel_id, 2);

    // Check video-tag relationships
    let video1_tags: Vec<&VideoTag> = re_exported
        .video_tags
        .iter()
        .filter(|vt| vt.video_id == 1)
        .collect();
    assert_eq!(video1_tags.len(), 2);

    let video2_tags: Vec<&VideoTag> = re_exported
        .video_tags
        .iter()
        .filter(|vt| vt.video_id == 2)
        .collect();
    assert_eq!(video2_tags.len(), 1);
}

#[test]
fn test_import_db_data_multiple_times() {
    let mut conn = setup_test_db();

    let database1 = Database {
        channels: vec![Channel {
            id: 1,
            link: "https://youtube.com/@channel1".to_string(),
            name: "Channel 1".to_string(),
            avatar: "avatar1.jpg".to_string(),
            verified: true,
            subscribers: Some(1000),
            created_at: "2026-01-01 00:00:00".to_string(),
            updated_at: "2026-01-01 00:00:00".to_string(),
        }],
        videos: vec![],
        tags: vec![],
        video_tags: vec![],
    };

    // First import
    import_db_data(&mut conn, database1).unwrap();
    let exported1 = export_db_data(&conn).unwrap();
    assert_eq!(exported1.channels.len(), 1);
    assert_eq!(exported1.channels[0].name, "Channel 1");

    let database2 = Database {
        channels: vec![Channel {
            id: 2,
            link: "https://youtube.com/@channel2".to_string(),
            name: "Channel 2".to_string(),
            avatar: "avatar2.jpg".to_string(),
            verified: false,
            subscribers: Some(2000),
            created_at: "2026-01-02 00:00:00".to_string(),
            updated_at: "2026-01-02 00:00:00".to_string(),
        }],
        videos: vec![],
        tags: vec![],
        video_tags: vec![],
    };

    // Second import (should replace first)
    import_db_data(&mut conn, database2).unwrap();
    let exported2 = export_db_data(&conn).unwrap();
    assert_eq!(exported2.channels.len(), 1);
    assert_eq!(exported2.channels[0].name, "Channel 2");
}

#[test]
fn test_import_db_data_preserves_timestamps() {
    let mut conn = setup_test_db();

    let created_at = "2026-01-01 12:34:56";
    let updated_at = "2026-01-02 23:45:01";

    let database = Database {
        channels: vec![],
        videos: vec![],
        tags: vec![Tag {
            id: 1,
            name: "Test Tag".to_string(),
            color: "#FF0000".to_string(),
            created_at: created_at.to_string(),
            updated_at: updated_at.to_string(),
        }],
        video_tags: vec![],
    };

    import_db_data(&mut conn, database).unwrap();

    let exported = export_db_data(&conn).unwrap();
    assert_eq!(exported.tags[0].created_at, created_at);
    assert_eq!(exported.tags[0].updated_at, updated_at);
}
