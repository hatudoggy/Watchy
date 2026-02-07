use rusqlite::Connection;
use watchy_lib::db::{
    data_transfer::{
        batch_insert_channels, batch_insert_tags, batch_insert_video_tags, batch_insert_videos,
        clear_all_tables, list_all_channels, list_all_tags, list_all_video_tags, list_all_videos,
    },
    schema::{Channel, Tag, Video, VideoStatus, VideoTag},
    schema_init::init_schema,
};

fn setup_test_db() -> Connection {
    let conn = Connection::open_in_memory().expect("Failed to create in-memory database");
    init_schema(&conn).expect("Failed to initialize schema");
    conn
}

// =====================
// LIST FUNCTIONS TESTS
// =====================

#[test]
fn test_list_all_videos_empty() {
    let conn = setup_test_db();
    let result = list_all_videos(&conn);
    assert!(result.is_ok());
    assert_eq!(result.unwrap().len(), 0);
}

#[test]
fn test_list_all_videos_with_data() {
    let conn = setup_test_db();

    // Insert test data manually
    conn.execute(
        "INSERT INTO channels (id, link, name, avatar, verified, subscribers) VALUES (1, 'https://youtube.com/@test', 'TestChannel', 'avatar.jpg', 1, 1000)",
        [],
    )
    .unwrap();

    conn.execute(
        "INSERT INTO videos (id, channel_id, link, title, thumbnail, duration, views, likes, uploaded_at, status) 
         VALUES (1, 1, 'https://youtube.com/watch?v=1', 'Test Video 1', 'thumb1.jpg', 300, 1000, 50, '2026-01-01', 'SAVED')",
        [],
    )
    .unwrap();

    conn.execute(
        "INSERT INTO videos (id, channel_id, link, title, thumbnail, duration, views, likes, uploaded_at, status) 
         VALUES (2, 1, 'https://youtube.com/watch?v=2', 'Test Video 2', 'thumb2.jpg', 600, 2000, 100, '2026-01-02', 'WATCHED')",
        [],
    )
    .unwrap();

    let result = list_all_videos(&conn);
    assert!(result.is_ok());

    let videos = result.unwrap();
    assert_eq!(videos.len(), 2);
    assert_eq!(videos[0].title, "Test Video 1");
    assert_eq!(videos[1].title, "Test Video 2");
}

#[test]
fn test_list_all_channels_empty() {
    let conn = setup_test_db();
    let result = list_all_channels(&conn);
    assert!(result.is_ok());
    assert_eq!(result.unwrap().len(), 0);
}

#[test]
fn test_list_all_channels_with_data() {
    let conn = setup_test_db();

    conn.execute(
        "INSERT INTO channels (id, link, name, avatar, verified, subscribers) VALUES (1, 'https://youtube.com/@channel1', 'Channel 1', 'avatar1.jpg', 1, 10000)",
        [],
    )
    .unwrap();

    conn.execute(
        "INSERT INTO channels (id, link, name, avatar, verified, subscribers) VALUES (2, 'https://youtube.com/@channel2', 'Channel 2', 'avatar2.jpg', 0, 5000)",
        [],
    )
    .unwrap();

    let result = list_all_channels(&conn);
    assert!(result.is_ok());

    let channels = result.unwrap();
    assert_eq!(channels.len(), 2);
    assert_eq!(channels[0].name, "Channel 1");
    assert_eq!(channels[1].name, "Channel 2");
}

#[test]
fn test_list_all_tags_empty() {
    let conn = setup_test_db();
    let result = list_all_tags(&conn);
    assert!(result.is_ok());
    assert_eq!(result.unwrap().len(), 0);
}

#[test]
fn test_list_all_tags_with_data() {
    let conn = setup_test_db();

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

    let result = list_all_tags(&conn);
    assert!(result.is_ok());

    let tags = result.unwrap();
    assert_eq!(tags.len(), 2);
    assert_eq!(tags[0].name, "Tutorial");
    assert_eq!(tags[1].name, "Review");
}

#[test]
fn test_list_all_video_tags_empty() {
    let conn = setup_test_db();
    let result = list_all_video_tags(&conn);
    assert!(result.is_ok());
    assert_eq!(result.unwrap().len(), 0);
}

#[test]
fn test_list_all_video_tags_with_data() {
    let conn = setup_test_db();

    // Setup prerequisite data
    conn.execute(
        "INSERT INTO channels (id, link, name, avatar, verified, subscribers) VALUES (1, 'https://youtube.com/@test', 'TestChannel', 'avatar.jpg', 1, 1000)",
        [],
    )
    .unwrap();

    conn.execute(
        "INSERT INTO videos (id, channel_id, link, title, thumbnail, uploaded_at, status) 
         VALUES (1, 1, 'https://youtube.com/watch?v=1', 'Test Video', 'thumb.jpg', '2026-01-01', 'SAVED')",
        [],
    )
    .unwrap();

    conn.execute(
        "INSERT INTO tags (id, name, color) VALUES (1, 'Tag1', '#FF0000')",
        [],
    )
    .unwrap();

    conn.execute(
        "INSERT INTO tags (id, name, color) VALUES (2, 'Tag2', '#00FF00')",
        [],
    )
    .unwrap();

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

    let result = list_all_video_tags(&conn);
    assert!(result.is_ok());

    let video_tags = result.unwrap();
    assert_eq!(video_tags.len(), 2);
    assert_eq!(video_tags[0].video_id, 1);
    assert_eq!(video_tags[0].tag_id, 1);
    assert_eq!(video_tags[1].video_id, 1);
    assert_eq!(video_tags[1].tag_id, 2);
}

// =====================
// BATCH INSERT TESTS
// =====================

#[test]
fn test_batch_insert_videos_empty() {
    let mut conn = setup_test_db();

    // Setup prerequisite channel
    conn.execute(
        "INSERT INTO channels (id, link, name, avatar, verified, subscribers) VALUES (1, 'https://youtube.com/@test', 'TestChannel', 'avatar.jpg', 1, 1000)",
        [],
    )
    .unwrap();

    let tx = conn.transaction().unwrap();
    let videos: Vec<Video> = vec![];

    let result = batch_insert_videos(&tx, &videos);
    assert!(result.is_ok());
    tx.commit().unwrap();

    let all_videos = list_all_videos(&conn).unwrap();
    assert_eq!(all_videos.len(), 0);
}

#[test]
fn test_batch_insert_videos_success() {
    let mut conn = setup_test_db();

    // Setup prerequisite channel
    conn.execute(
        "INSERT INTO channels (id, link, name, avatar, verified, subscribers) VALUES (1, 'https://youtube.com/@test', 'TestChannel', 'avatar.jpg', 1, 1000)",
        [],
    )
    .unwrap();

    let videos = vec![
        Video {
            id: 1,
            channel_id: 1,
            link: "https://youtube.com/watch?v=1".to_string(),
            title: "Video 1".to_string(),
            thumbnail: "thumb1.jpg".to_string(),
            duration: Some(300),
            views: Some(1000),
            likes: Some(50),
            uploaded_at: "2026-01-01".to_string(),
            status: VideoStatus::SAVED,
            created_at: "2026-01-01 00:00:00".to_string(),
            updated_at: "2026-01-01 00:00:00".to_string(),
            watched_at: None,
        },
        Video {
            id: 2,
            channel_id: 1,
            link: "https://youtube.com/watch?v=2".to_string(),
            title: "Video 2".to_string(),
            thumbnail: "thumb2.jpg".to_string(),
            duration: Some(600),
            views: Some(2000),
            likes: Some(100),
            uploaded_at: "2026-01-02".to_string(),
            status: VideoStatus::WATCHED,
            created_at: "2026-01-02 00:00:00".to_string(),
            updated_at: "2026-01-02 00:00:00".to_string(),
            watched_at: Some("2026-01-03 00:00:00".to_string()),
        },
    ];

    let tx = conn.transaction().unwrap();
    let result = batch_insert_videos(&tx, &videos);
    assert!(result.is_ok());
    tx.commit().unwrap();

    let all_videos = list_all_videos(&conn).unwrap();
    assert_eq!(all_videos.len(), 2);
    assert_eq!(all_videos[0].title, "Video 1");
    assert_eq!(all_videos[1].title, "Video 2");
}

#[test]
fn test_batch_insert_channels_success() {
    let mut conn = setup_test_db();

    let channels = vec![
        Channel {
            id: 1,
            link: "https://youtube.com/@channel1".to_string(),
            name: "Channel 1".to_string(),
            avatar: "avatar1.jpg".to_string(),
            verified: true,
            subscribers: Some(10000),
            created_at: "2026-01-01 00:00:00".to_string(),
            updated_at: "2026-01-01 00:00:00".to_string(),
        },
        Channel {
            id: 2,
            link: "https://youtube.com/@channel2".to_string(),
            name: "Channel 2".to_string(),
            avatar: "avatar2.jpg".to_string(),
            verified: false,
            subscribers: Some(5000),
            created_at: "2026-01-02 00:00:00".to_string(),
            updated_at: "2026-01-02 00:00:00".to_string(),
        },
    ];

    let tx = conn.transaction().unwrap();
    let result = batch_insert_channels(&tx, &channels);
    assert!(result.is_ok());
    tx.commit().unwrap();

    let all_channels = list_all_channels(&conn).unwrap();
    assert_eq!(all_channels.len(), 2);
    assert_eq!(all_channels[0].name, "Channel 1");
    assert_eq!(all_channels[1].name, "Channel 2");
}

#[test]
fn test_batch_insert_tags_success() {
    let mut conn = setup_test_db();

    let tags = vec![
        Tag {
            id: 1,
            name: "Tutorial".to_string(),
            color: "#FF0000".to_string(),
            created_at: "2026-01-01 00:00:00".to_string(),
            updated_at: "2026-01-01 00:00:00".to_string(),
        },
        Tag {
            id: 2,
            name: "Review".to_string(),
            color: "#00FF00".to_string(),
            created_at: "2026-01-02 00:00:00".to_string(),
            updated_at: "2026-01-02 00:00:00".to_string(),
        },
    ];

    let tx = conn.transaction().unwrap();
    let result = batch_insert_tags(&tx, &tags);
    assert!(result.is_ok());
    tx.commit().unwrap();

    let all_tags = list_all_tags(&conn).unwrap();
    assert_eq!(all_tags.len(), 2);
    assert_eq!(all_tags[0].name, "Tutorial");
    assert_eq!(all_tags[1].name, "Review");
}

#[test]
fn test_batch_insert_video_tags_success() {
    let mut conn = setup_test_db();

    // Setup prerequisites
    conn.execute(
        "INSERT INTO channels (id, link, name, avatar, verified, subscribers) VALUES (1, 'https://youtube.com/@test', 'TestChannel', 'avatar.jpg', 1, 1000)",
        [],
    )
    .unwrap();

    conn.execute(
        "INSERT INTO videos (id, channel_id, link, title, thumbnail, uploaded_at, status) 
         VALUES (1, 1, 'https://youtube.com/watch?v=1', 'Test Video', 'thumb.jpg', '2026-01-01', 'SAVED')",
        [],
    )
    .unwrap();

    conn.execute(
        "INSERT INTO tags (id, name, color) VALUES (1, 'Tag1', '#FF0000')",
        [],
    )
    .unwrap();

    conn.execute(
        "INSERT INTO tags (id, name, color) VALUES (2, 'Tag2', '#00FF00')",
        [],
    )
    .unwrap();

    let video_tags = vec![
        VideoTag {
            video_id: 1,
            tag_id: 1,
        },
        VideoTag {
            video_id: 1,
            tag_id: 2,
        },
    ];

    let tx = conn.transaction().unwrap();
    let result = batch_insert_video_tags(&tx, &video_tags);
    assert!(result.is_ok());
    tx.commit().unwrap();

    let all_video_tags = list_all_video_tags(&conn).unwrap();
    assert_eq!(all_video_tags.len(), 2);
}

// =====================
// CLEAR TABLES TEST
// =====================

#[test]
fn test_clear_all_tables() {
    let mut conn = setup_test_db();

    // Setup data in all tables
    conn.execute(
        "INSERT INTO channels (id, link, name, avatar, verified, subscribers) VALUES (1, 'https://youtube.com/@test', 'TestChannel', 'avatar.jpg', 1, 1000)",
        [],
    )
    .unwrap();

    conn.execute(
        "INSERT INTO videos (id, channel_id, link, title, thumbnail, uploaded_at, status) 
         VALUES (1, 1, 'https://youtube.com/watch?v=1', 'Test Video', 'thumb.jpg', '2026-01-01', 'SAVED')",
        [],
    )
    .unwrap();

    conn.execute(
        "INSERT INTO tags (id, name, color) VALUES (1, 'Tag1', '#FF0000')",
        [],
    )
    .unwrap();

    conn.execute(
        "INSERT INTO video_tags (video_id, tag_id) VALUES (1, 1)",
        [],
    )
    .unwrap();

    // Verify data exists
    assert_eq!(list_all_videos(&conn).unwrap().len(), 1);
    assert_eq!(list_all_channels(&conn).unwrap().len(), 1);
    assert_eq!(list_all_tags(&conn).unwrap().len(), 1);
    assert_eq!(list_all_video_tags(&conn).unwrap().len(), 1);

    // Clear all tables
    let tx = conn.transaction().unwrap();
    let result = clear_all_tables(&tx);
    assert!(result.is_ok());
    tx.commit().unwrap();

    // Verify all tables are empty
    assert_eq!(list_all_videos(&conn).unwrap().len(), 0);
    assert_eq!(list_all_channels(&conn).unwrap().len(), 0);
    assert_eq!(list_all_tags(&conn).unwrap().len(), 0);
    assert_eq!(list_all_video_tags(&conn).unwrap().len(), 0);
}

#[test]
fn test_clear_all_tables_already_empty() {
    let mut conn = setup_test_db();

    let tx = conn.transaction().unwrap();
    let result = clear_all_tables(&tx);
    assert!(result.is_ok());
    tx.commit().unwrap();

    assert_eq!(list_all_videos(&conn).unwrap().len(), 0);
    assert_eq!(list_all_channels(&conn).unwrap().len(), 0);
    assert_eq!(list_all_tags(&conn).unwrap().len(), 0);
    assert_eq!(list_all_video_tags(&conn).unwrap().len(), 0);
}

// =====================
// INTEGRATION TESTS
// =====================

#[test]
fn test_batch_operations_complete_workflow() {
    let mut conn = setup_test_db();

    // Create complete database snapshot
    let channels = vec![Channel {
        id: 1,
        link: "https://youtube.com/@test".to_string(),
        name: "Test Channel".to_string(),
        avatar: "avatar.jpg".to_string(),
        verified: true,
        subscribers: Some(10000),
        created_at: "2026-01-01 00:00:00".to_string(),
        updated_at: "2026-01-01 00:00:00".to_string(),
    }];

    let videos = vec![Video {
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
    }];

    let tags = vec![Tag {
        id: 1,
        name: "Tutorial".to_string(),
        color: "#FF0000".to_string(),
        created_at: "2026-01-01 00:00:00".to_string(),
        updated_at: "2026-01-01 00:00:00".to_string(),
    }];

    let video_tags = vec![VideoTag {
        video_id: 1,
        tag_id: 1,
    }];

    // Insert all data in transaction
    let tx = conn.transaction().unwrap();
    batch_insert_channels(&tx, &channels).unwrap();
    batch_insert_videos(&tx, &videos).unwrap();
    batch_insert_tags(&tx, &tags).unwrap();
    batch_insert_video_tags(&tx, &video_tags).unwrap();
    tx.commit().unwrap();

    // Verify all data was inserted correctly
    assert_eq!(list_all_channels(&conn).unwrap().len(), 1);
    assert_eq!(list_all_videos(&conn).unwrap().len(), 1);
    assert_eq!(list_all_tags(&conn).unwrap().len(), 1);
    assert_eq!(list_all_video_tags(&conn).unwrap().len(), 1);

    // Clear and verify
    let tx = conn.transaction().unwrap();
    clear_all_tables(&tx).unwrap();
    tx.commit().unwrap();

    assert_eq!(list_all_channels(&conn).unwrap().len(), 0);
    assert_eq!(list_all_videos(&conn).unwrap().len(), 0);
    assert_eq!(list_all_tags(&conn).unwrap().len(), 0);
    assert_eq!(list_all_video_tags(&conn).unwrap().len(), 0);
}
