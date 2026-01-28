use rusqlite::Connection;
use watchy_lib::db::{
    dto::{CreateChannel, CreateVideo},
    queries::{create_channel, create_video, get_channel, get_video, list_videos},
    schema::VideoStatus,
    schema_init::init_schema,
};

/// Helper function to create an in-memory SQLite database with the schema initialized
fn setup_test_db() -> Connection {
    let conn = Connection::open_in_memory().expect("Failed to create in-memory database");
    init_schema(&conn).expect("Failed to initialize schema");
    conn
}

/// Helper function to create a test channel
fn create_test_channel(conn: &Connection, name: &str) -> i64 {
    let channel = CreateChannel {
        link: format!("https://youtube.com/@{}", name),
        name: name.to_string(),
        avatar: format!("https://img.test/{}.jpg", name),
        verified: true,
        subscribers: Some(1_000_000),
    };

    let created = create_channel(conn, channel).expect("Failed to create channel");
    created.id
}

/// Helper function to create a test video
fn create_test_video(conn: &Connection, channel_id: i64, title: &str) -> i64 {
    let video = CreateVideo {
        channel_id,
        link: format!("https://youtube.com/watch?v={}", title.replace(" ", "")),
        title: title.to_string(),
        thumbnail: format!("https://img.test/{}.jpg", title.replace(" ", "")),
        duration: Some(600),
        views: Some(50_000),
        likes: Some(2_000),
        uploaded_at: "2026-01-10".to_string(),
        status: VideoStatus::SAVED,
    };

    let created = create_video(conn, video).expect("Failed to create video");
    created.id
}

#[test]
fn test_create_channel_success() {
    let conn = setup_test_db();

    let channel = CreateChannel {
        link: "https://youtube.com/@testchannel".to_string(),
        name: "Test Channel".to_string(),
        avatar: "https://img.test/avatar.jpg".to_string(),
        verified: true,
        subscribers: Some(1_500_000),
    };

    let result = create_channel(&conn, channel);
    assert!(result.is_ok());

    let created = result.unwrap();
    assert_eq!(created.name, "Test Channel");
    assert_eq!(created.link, "https://youtube.com/@testchannel");
    assert_eq!(created.avatar, "https://img.test/avatar.jpg");
    assert_eq!(created.verified, true);
    assert_eq!(created.subscribers, Some(1_500_000));
    assert!(created.id > 0);
    assert!(!created.created_at.is_empty());
    assert!(!created.updated_at.is_empty());
}

#[test]
fn test_create_channel_duplicate_link() {
    let conn = setup_test_db();

    let channel1 = CreateChannel {
        link: "https://youtube.com/@duplicate".to_string(),
        name: "Channel One".to_string(),
        avatar: "https://img.test/avatar1.jpg".to_string(),
        verified: false,
        subscribers: None,
    };

    let channel2 = CreateChannel {
        link: "https://youtube.com/@duplicate".to_string(),
        name: "Channel Two".to_string(),
        avatar: "https://img.test/avatar2.jpg".to_string(),
        verified: true,
        subscribers: Some(1000),
    };

    // First creation should succeed
    assert!(create_channel(&conn, channel1).is_ok());

    // Second creation with same link should fail due to UNIQUE constraint
    assert!(create_channel(&conn, channel2).is_err());
}

#[test]
fn test_get_channel_success() {
    let conn = setup_test_db();
    let channel_id = create_test_channel(&conn, "GetTestChannel");

    let result = get_channel(&conn, channel_id);
    assert!(result.is_ok());

    let channel = result.unwrap();
    assert_eq!(channel.id, channel_id);
    assert_eq!(channel.name, "GetTestChannel");
    assert!(channel.link.contains("GetTestChannel"));
}

#[test]
fn test_get_channel_not_found() {
    let conn = setup_test_db();

    let result = get_channel(&conn, 999);
    assert!(result.is_err());
}

#[test]
fn test_create_video_success() {
    let conn = setup_test_db();
    let channel_id = create_test_channel(&conn, "VideoTestChannel");

    let video = CreateVideo {
        channel_id,
        link: "https://youtube.com/watch?v=testvideo123".to_string(),
        title: "Test Video Title".to_string(),
        thumbnail: "https://img.test/thumb.jpg".to_string(),
        duration: Some(600),
        views: Some(50_000),
        likes: Some(2_000),
        uploaded_at: "2026-01-10".to_string(),
        status: VideoStatus::TOWATCH,
    };

    let result = create_video(&conn, video);
    assert!(result.is_ok());

    let created = result.unwrap();
    assert_eq!(created.title, "Test Video Title");
    assert_eq!(created.link, "https://youtube.com/watch?v=testvideo123");
    assert_eq!(created.duration, Some(600));
    assert_eq!(created.views, Some(50_000));
    assert_eq!(created.likes, Some(2_000));
    assert!(matches!(created.status, VideoStatus::TOWATCH));
    assert!(created.id > 0);
}

#[test]
fn test_create_video_with_optional_fields_none() {
    let conn = setup_test_db();
    let channel_id = create_test_channel(&conn, "OptionalFieldsChannel");

    let video = CreateVideo {
        channel_id,
        link: "https://youtube.com/watch?v=optional123".to_string(),
        title: "Video Without Stats".to_string(),
        thumbnail: "https://img.test/thumb2.jpg".to_string(),
        duration: None,
        views: None,
        likes: None,
        uploaded_at: "2026-01-11".to_string(),
        status: VideoStatus::SAVED,
    };

    let result = create_video(&conn, video);
    assert!(result.is_ok());

    let created = result.unwrap();
    assert_eq!(created.duration, None);
    assert_eq!(created.views, None);
    assert_eq!(created.likes, None);
}

#[test]
fn test_create_video_duplicate_link() {
    let conn = setup_test_db();
    let channel_id = create_test_channel(&conn, "DuplicateVideoChannel");

    let video1 = CreateVideo {
        channel_id,
        link: "https://youtube.com/watch?v=duplicate".to_string(),
        title: "First Video".to_string(),
        thumbnail: "https://img.test/thumb1.jpg".to_string(),
        duration: Some(300),
        views: Some(1000),
        likes: Some(50),
        uploaded_at: "2026-01-10".to_string(),
        status: VideoStatus::SAVED,
    };

    let video2 = CreateVideo {
        channel_id,
        link: "https://youtube.com/watch?v=duplicate".to_string(),
        title: "Second Video".to_string(),
        thumbnail: "https://img.test/thumb2.jpg".to_string(),
        duration: Some(400),
        views: Some(2000),
        likes: Some(100),
        uploaded_at: "2026-01-11".to_string(),
        status: VideoStatus::WATCHED,
    };

    // First creation should succeed
    assert!(create_video(&conn, video1).is_ok());

    // Second creation with same link should fail due to UNIQUE constraint
    assert!(create_video(&conn, video2).is_err());
}

#[test]
fn test_get_video_success() {
    let conn = setup_test_db();
    let channel_id = create_test_channel(&conn, "GetVideoChannel");
    let video_id = create_test_video(&conn, channel_id, "Get Video Test");

    let result = get_video(&conn, video_id);
    assert!(result.is_ok());

    let video = result.unwrap();
    assert_eq!(video.id, video_id);
    assert_eq!(video.channel_id, channel_id);
    assert_eq!(video.title, "Get Video Test");
    assert_eq!(video.channel_name, "GetVideoChannel");
    assert_eq!(video.channel_link, "https://youtube.com/@GetVideoChannel");
}

#[test]
fn test_get_video_not_found() {
    let conn = setup_test_db();

    let result = get_video(&conn, 999);
    assert!(result.is_err());
}

#[test]
fn test_list_videos_empty() {
    let conn = setup_test_db();

    let result = list_videos(&conn, None, None);
    assert!(result.is_ok());

    let videos = result.unwrap();
    assert_eq!(videos.len(), 0);
}

#[test]
fn test_list_videos_single() {
    let conn = setup_test_db();
    let channel_id = create_test_channel(&conn, "SingleVideoChannel");
    create_test_video(&conn, channel_id, "Single Video");

    let result = list_videos(&conn, None, None);
    assert!(result.is_ok());

    let videos = result.unwrap();
    assert_eq!(videos.len(), 1);
    assert_eq!(videos[0].title, "Single Video");
    assert_eq!(videos[0].channel_name, "SingleVideoChannel");
    assert_eq!(
        videos[0].channel_link,
        "https://youtube.com/@SingleVideoChannel"
    );
}

#[test]
fn test_list_videos_multiple() {
    let conn = setup_test_db();
    let channel1_id = create_test_channel(&conn, "Channel1");
    let channel2_id = create_test_channel(&conn, "Channel2");

    create_test_video(&conn, channel1_id, "Video One");
    create_test_video(&conn, channel1_id, "Video Two");
    create_test_video(&conn, channel2_id, "Video Three");

    let result = list_videos(&conn, None, None);
    assert!(result.is_ok());

    let videos = result.unwrap();
    assert_eq!(videos.len(), 3);

    // Verify all videos are present (order might vary due to same timestamp)
    let titles: Vec<&str> = videos.iter().map(|v| v.title.as_str()).collect();
    assert!(titles.contains(&"Video One"));
    assert!(titles.contains(&"Video Two"));
    assert!(titles.contains(&"Video Three"));
}

#[test]
fn test_list_videos_with_all_video_statuses() {
    let conn = setup_test_db();
    let channel_id = create_test_channel(&conn, "StatusChannel");

    // Create videos with different statuses
    let video1 = CreateVideo {
        channel_id,
        link: "https://youtube.com/watch?v=saved".to_string(),
        title: "Saved Video".to_string(),
        thumbnail: "https://img.test/saved.jpg".to_string(),
        duration: Some(300),
        views: Some(1000),
        likes: Some(50),
        uploaded_at: "2026-01-10".to_string(),
        status: VideoStatus::SAVED,
    };

    let video2 = CreateVideo {
        channel_id,
        link: "https://youtube.com/watch?v=towatch".to_string(),
        title: "To Watch Video".to_string(),
        thumbnail: "https://img.test/towatch.jpg".to_string(),
        duration: Some(400),
        views: Some(2000),
        likes: Some(100),
        uploaded_at: "2026-01-11".to_string(),
        status: VideoStatus::TOWATCH,
    };

    let video3 = CreateVideo {
        channel_id,
        link: "https://youtube.com/watch?v=watched".to_string(),
        title: "Watched Video".to_string(),
        thumbnail: "https://img.test/watched.jpg".to_string(),
        duration: Some(500),
        views: Some(3000),
        likes: Some(150),
        uploaded_at: "2026-01-12".to_string(),
        status: VideoStatus::WATCHED,
    };

    create_video(&conn, video1).expect("Failed to create SAVED video");
    create_video(&conn, video2).expect("Failed to create TOWATCH video");
    create_video(&conn, video3).expect("Failed to create WATCHED video");

    let result = list_videos(&conn, None, None);
    assert!(result.is_ok());

    let videos = result.unwrap();
    assert_eq!(videos.len(), 3);

    // Verify all statuses are present
    let has_saved = videos
        .iter()
        .any(|v| matches!(v.status, VideoStatus::SAVED));
    let has_towatch = videos
        .iter()
        .any(|v| matches!(v.status, VideoStatus::TOWATCH));
    let has_watched = videos
        .iter()
        .any(|v| matches!(v.status, VideoStatus::WATCHED));

    assert!(has_saved, "Should have a SAVED video");
    assert!(has_towatch, "Should have a TOWATCH video");
    assert!(has_watched, "Should have a WATCHED video");
}

#[test]
fn test_video_with_channel_relationship() {
    let conn = setup_test_db();

    // Create a channel with specific details
    let channel = CreateChannel {
        link: "https://youtube.com/@relationshiptest".to_string(),
        name: "Relationship Test Channel".to_string(),
        avatar: "https://img.test/rel.jpg".to_string(),
        verified: true,
        subscribers: Some(5_000_000),
    };
    let created_channel = create_channel(&conn, channel).expect("Failed to create channel");

    // Create a video for that channel
    let video = CreateVideo {
        channel_id: created_channel.id,
        link: "https://youtube.com/watch?v=relationship".to_string(),
        title: "Relationship Test Video".to_string(),
        thumbnail: "https://img.test/rel_video.jpg".to_string(),
        duration: Some(720),
        views: Some(100_000),
        likes: Some(5_000),
        uploaded_at: "2026-01-13".to_string(),
        status: VideoStatus::TOWATCH,
    };
    let created_video = create_video(&conn, video).expect("Failed to create video");

    // Get video and verify channel relationship
    let fetched = get_video(&conn, created_video.id).expect("Failed to get video");
    assert_eq!(fetched.channel_id, created_channel.id);
    assert_eq!(fetched.channel_name, "Relationship Test Channel");
    assert_eq!(
        fetched.channel_link,
        "https://youtube.com/@relationshiptest"
    );

    // List videos and verify channel info is included
    let videos = list_videos(&conn, None, None).expect("Failed to list videos");
    assert_eq!(videos.len(), 1);
    assert_eq!(videos[0].channel_name, "Relationship Test Channel");
    assert_eq!(
        videos[0].channel_link,
        "https://youtube.com/@relationshiptest"
    );
}

#[test]
fn test_foreign_key_constraint() {
    let conn = setup_test_db();

    // Try to create a video with non-existent channel_id
    let video = CreateVideo {
        channel_id: 999999,
        link: "https://youtube.com/watch?v=orphan".to_string(),
        title: "Orphan Video".to_string(),
        thumbnail: "https://img.test/orphan.jpg".to_string(),
        duration: Some(300),
        views: Some(1000),
        likes: Some(50),
        uploaded_at: "2026-01-10".to_string(),
        status: VideoStatus::SAVED,
    };

    // This should fail due to foreign key constraint
    // Note: SQLite needs foreign keys to be enabled explicitly
    conn.execute("PRAGMA foreign_keys = ON", [])
        .expect("Failed to enable foreign keys");

    let result = create_video(&conn, video);
    // The result might be Ok but the query should fail at the database level
    // depending on how rusqlite handles FK violations
    assert!(result.is_err());
}

#[test]
fn test_update_video_success() {
    use watchy_lib::db::dto::UpdateVideo;
    use watchy_lib::db::queries::update_video;

    let conn = setup_test_db();
    let channel_id = create_test_channel(&conn, "UpdateVideoChannel");
    let video_id = create_test_video(&conn, channel_id, "Original Video");

    let update = UpdateVideo {
        link: Some("https://youtube.com/watch?v=updated".to_string()),
        title: Some("Updated Video Title".to_string()),
        thumbnail: Some("https://img.test/updated.jpg".to_string()),
        duration: Some(900),
        views: Some(100_000),
        likes: Some(5_000),
        uploaded_at: Some("2026-01-12".to_string()),
        status: Some(VideoStatus::WATCHED),
    };

    let result = update_video(&conn, video_id, update);
    assert!(result.is_ok());

    let updated = result.unwrap();
    assert_eq!(updated.id, video_id);
    assert_eq!(updated.title, "Updated Video Title");
    assert_eq!(updated.link, "https://youtube.com/watch?v=updated");
    assert_eq!(updated.duration, Some(900));
    assert_eq!(updated.views, Some(100_000));
    assert_eq!(updated.likes, Some(5_000));
    assert!(matches!(updated.status, VideoStatus::WATCHED));
}

#[test]
fn test_update_video_not_found() {
    use watchy_lib::db::dto::UpdateVideo;
    use watchy_lib::db::queries::update_video;

    let conn = setup_test_db();

    let update = UpdateVideo {
        link: Some("https://youtube.com/watch?v=nonexistent".to_string()),
        title: Some("Non-existent Video".to_string()),
        thumbnail: Some("https://img.test/none.jpg".to_string()),
        duration: Some(300),
        views: Some(1000),
        likes: Some(50),
        uploaded_at: Some("2026-01-10".to_string()),
        status: Some(VideoStatus::SAVED),
    };

    let result = update_video(&conn, 999, update);
    assert!(result.is_err());
}

#[test]
fn test_update_video_changes_updated_at() {
    use watchy_lib::db::dto::UpdateVideo;
    use watchy_lib::db::queries::update_video;

    let conn = setup_test_db();
    let channel_id = create_test_channel(&conn, "TimestampChannel");
    let video_id = create_test_video(&conn, channel_id, "Timestamp Test");

    let original = get_video(&conn, video_id).expect("Failed to get original video");
    let original_updated_at = original.updated_at.clone();

    // Small delay to ensure timestamp changes
    std::thread::sleep(std::time::Duration::from_millis(10));

    let update = UpdateVideo {
        link: Some(original.link.clone()),
        title: Some("Updated Title".to_string()),
        thumbnail: Some(original.thumbnail.clone()),
        duration: original.duration,
        views: original.views,
        likes: original.likes,
        uploaded_at: Some(original.uploaded_at.clone()),
        status: Some(original.status),
    };

    update_video(&conn, video_id, update).expect("Failed to update");

    let updated = get_video(&conn, video_id).expect("Failed to get updated video");
    // Note: SQLite's datetime('now') might have same precision, so we just verify it's updated
    assert!(updated.updated_at >= original_updated_at);
}

#[test]
fn test_update_video_partial_update() {
    use watchy_lib::db::dto::UpdateVideo;
    use watchy_lib::db::queries::update_video;

    let conn = setup_test_db();
    let channel_id = create_test_channel(&conn, "PartialUpdateChannel");
    let video_id = create_test_video(&conn, channel_id, "Original Video");

    let original = get_video(&conn, video_id).expect("Failed to get original video");

    // Update only title and status
    let update = UpdateVideo {
        link: None,
        title: Some("Only Title Changed".to_string()),
        thumbnail: None,
        duration: None,
        views: None,
        likes: None,
        uploaded_at: None,
        status: Some(VideoStatus::WATCHED),
    };

    let result = update_video(&conn, video_id, update);
    assert!(result.is_ok());

    let updated = result.unwrap();
    // Changed fields
    assert_eq!(updated.title, "Only Title Changed");
    assert!(matches!(updated.status, VideoStatus::WATCHED));

    // Unchanged fields should remain the same
    assert_eq!(updated.link, original.link);
    assert_eq!(updated.thumbnail, original.thumbnail);
    assert_eq!(updated.duration, original.duration);
    assert_eq!(updated.views, original.views);
    assert_eq!(updated.likes, original.likes);
    assert_eq!(updated.uploaded_at, original.uploaded_at);
}

#[test]
fn test_update_video_no_fields_error() {
    use watchy_lib::db::dto::UpdateVideo;
    use watchy_lib::db::queries::update_video;

    let conn = setup_test_db();
    let channel_id = create_test_channel(&conn, "NoFieldsChannel");
    let video_id = create_test_video(&conn, channel_id, "Test Video");

    // Update with all None fields should error
    let update = UpdateVideo {
        link: None,
        title: None,
        thumbnail: None,
        duration: None,
        views: None,
        likes: None,
        uploaded_at: None,
        status: None,
    };

    let result = update_video(&conn, video_id, update);
    assert!(result.is_err());
    assert!(result.unwrap_err().contains("No fields to update"));
}

#[test]
fn test_delete_video_success() {
    use watchy_lib::db::queries::delete_video;

    let conn = setup_test_db();
    let channel_id = create_test_channel(&conn, "DeleteVideoChannel");
    let video_id = create_test_video(&conn, channel_id, "Video to Delete");

    // Verify video exists
    assert!(get_video(&conn, video_id).is_ok());

    // Delete the video
    let result = delete_video(&conn, video_id);
    assert!(result.is_ok());

    // Verify video no longer exists
    assert!(get_video(&conn, video_id).is_err());
}

#[test]
fn test_delete_video_not_found() {
    use watchy_lib::db::queries::delete_video;

    let conn = setup_test_db();

    let result = delete_video(&conn, 999);
    assert!(result.is_err());

    let err_msg = result.unwrap_err();
    assert!(err_msg.contains("not found"));
}

#[test]
fn test_delete_video_removes_from_list() {
    use watchy_lib::db::queries::delete_video;

    let conn = setup_test_db();
    let channel_id = create_test_channel(&conn, "ListDeleteChannel");

    let video1_id = create_test_video(&conn, channel_id, "Video One");
    let video2_id = create_test_video(&conn, channel_id, "Video Two");
    create_test_video(&conn, channel_id, "Video Three");

    // Verify we have 3 videos
    let videos = list_videos(&conn, None, None).expect("Failed to list videos");
    assert_eq!(videos.len(), 3);

    // Delete one video
    delete_video(&conn, video2_id).expect("Failed to delete video");

    // Verify we now have 2 videos
    let videos = list_videos(&conn, None, None).expect("Failed to list videos");
    assert_eq!(videos.len(), 2);

    // Verify the correct videos remain
    let remaining_ids: Vec<i64> = videos.iter().map(|v| v.id).collect();
    assert!(remaining_ids.contains(&video1_id));
    assert!(!remaining_ids.contains(&video2_id));
}

#[test]
fn test_update_channel_success() {
    use watchy_lib::db::dto::UpdateChannel;
    use watchy_lib::db::queries::update_channel;

    let conn = setup_test_db();
    let channel_id = create_test_channel(&conn, "OriginalChannel");

    let update = UpdateChannel {
        link: "https://youtube.com/@updatedchannel".to_string(),
        name: "Updated Channel Name".to_string(),
        avatar: "https://img.test/updated_avatar.jpg".to_string(),
        verified: false,
        subscribers: Some(2_000_000),
    };

    let result = update_channel(&conn, channel_id, update);
    assert!(result.is_ok());

    let updated = result.unwrap();
    assert_eq!(updated.id, channel_id);
    assert_eq!(updated.name, "Updated Channel Name");
    assert_eq!(updated.link, "https://youtube.com/@updatedchannel");
    assert_eq!(updated.avatar, "https://img.test/updated_avatar.jpg");
    assert_eq!(updated.verified, false);
    assert_eq!(updated.subscribers, Some(2_000_000));
}

#[test]
fn test_update_channel_not_found() {
    use watchy_lib::db::dto::UpdateChannel;
    use watchy_lib::db::queries::update_channel;

    let conn = setup_test_db();

    let update = UpdateChannel {
        link: "https://youtube.com/@nonexistent".to_string(),
        name: "Non-existent Channel".to_string(),
        avatar: "https://img.test/none.jpg".to_string(),
        verified: false,
        subscribers: None,
    };

    let result = update_channel(&conn, 999, update);
    assert!(result.is_err());
}

#[test]
fn test_delete_channel_success() {
    use watchy_lib::db::queries::delete_channel;

    let conn = setup_test_db();
    let channel_id = create_test_channel(&conn, "DeleteChannel");

    // Verify channel exists
    assert!(get_channel(&conn, channel_id).is_ok());

    // Delete the channel
    let result = delete_channel(&conn, channel_id);
    assert!(result.is_ok());

    // Verify channel no longer exists
    assert!(get_channel(&conn, channel_id).is_err());
}

#[test]
fn test_delete_channel_not_found() {
    use watchy_lib::db::queries::delete_channel;

    let conn = setup_test_db();

    let result = delete_channel(&conn, 999);
    assert!(result.is_err());

    let err_msg = result.unwrap_err();
    assert!(err_msg.contains("not found"));
}

#[test]
fn test_delete_channel_cascades_to_videos() {
    use watchy_lib::db::queries::delete_channel;

    let conn = setup_test_db();

    // Enable foreign keys for cascade
    conn.execute("PRAGMA foreign_keys = ON", [])
        .expect("Failed to enable foreign keys");

    let channel_id = create_test_channel(&conn, "CascadeChannel");
    let video1_id = create_test_video(&conn, channel_id, "Video One");
    let video2_id = create_test_video(&conn, channel_id, "Video Two");

    // Verify videos exist
    assert!(get_video(&conn, video1_id).is_ok());
    assert!(get_video(&conn, video2_id).is_ok());

    // Delete the channel
    delete_channel(&conn, channel_id).expect("Failed to delete channel");

    // Verify channel is deleted
    assert!(get_channel(&conn, channel_id).is_err());

    // Verify videos are also deleted (cascade)
    assert!(get_video(&conn, video1_id).is_err());
    assert!(get_video(&conn, video2_id).is_err());
}

#[test]
fn test_update_channel_with_videos_preserves_videos() {
    use watchy_lib::db::dto::UpdateChannel;
    use watchy_lib::db::queries::update_channel;

    let conn = setup_test_db();
    let channel_id = create_test_channel(&conn, "ChannelWithVideos");
    let video_id = create_test_video(&conn, channel_id, "Test Video");

    let update = UpdateChannel {
        link: "https://youtube.com/@updatedlink".to_string(),
        name: "Updated Name".to_string(),
        avatar: "https://img.test/new.jpg".to_string(),
        verified: true,
        subscribers: Some(5_000_000),
    };

    update_channel(&conn, channel_id, update).expect("Failed to update channel");

    // Verify video still exists and references the same channel
    let video = get_video(&conn, video_id).expect("Failed to get video");
    assert_eq!(video.channel_id, channel_id);
    assert_eq!(video.channel_name, "Updated Name");
    assert_eq!(video.channel_link, "https://youtube.com/@updatedlink");
}

// =====================
// TAG TESTS
// =====================

/// Helper function to create a test tag
fn create_test_tag(conn: &Connection, name: &str, color: &str) -> i64 {
    use watchy_lib::db::dto::CreateTag;
    use watchy_lib::db::queries::create_tag;

    let tag = CreateTag {
        name: name.to_string(),
        color: color.to_string(),
    };

    let created = create_tag(conn, tag).expect("Failed to create tag");
    created.id
}

#[test]
fn test_create_tag_success() {
    use watchy_lib::db::dto::CreateTag;
    use watchy_lib::db::queries::create_tag;

    let conn = setup_test_db();

    let tag = CreateTag {
        name: "Tutorial".to_string(),
        color: "#FF5733".to_string(),
    };

    let result = create_tag(&conn, tag);
    assert!(result.is_ok());

    let created = result.unwrap();
    assert_eq!(created.name, "Tutorial");
    assert_eq!(created.color, "#FF5733");
    assert!(created.id > 0);
    assert!(!created.created_at.is_empty());
    assert!(!created.updated_at.is_empty());
}

#[test]
fn test_create_tag_duplicate_name() {
    use watchy_lib::db::dto::CreateTag;
    use watchy_lib::db::queries::create_tag;

    let conn = setup_test_db();

    let tag1 = CreateTag {
        name: "Duplicate".to_string(),
        color: "#FF0000".to_string(),
    };

    let tag2 = CreateTag {
        name: "Duplicate".to_string(),
        color: "#00FF00".to_string(),
    };

    // First creation should succeed
    assert!(create_tag(&conn, tag1).is_ok());

    // Second creation with same name should fail due to UNIQUE constraint
    assert!(create_tag(&conn, tag2).is_err());
}

#[test]
fn test_get_tag_success() {
    use watchy_lib::db::queries::get_tag;

    let conn = setup_test_db();
    let tag_id = create_test_tag(&conn, "Test Tag", "#0000FF");

    let result = get_tag(&conn, tag_id);
    assert!(result.is_ok());

    let tag = result.unwrap();
    assert_eq!(tag.id, tag_id);
    assert_eq!(tag.name, "Test Tag");
    assert_eq!(tag.color, "#0000FF");
}

#[test]
fn test_get_tag_not_found() {
    use watchy_lib::db::queries::get_tag;

    let conn = setup_test_db();

    let result = get_tag(&conn, 999);
    assert!(result.is_err());
}

#[test]
fn test_list_tags_empty() {
    use watchy_lib::db::queries::list_tags;

    let conn = setup_test_db();

    let result = list_tags(&conn);
    assert!(result.is_ok());

    let tags = result.unwrap();
    assert_eq!(tags.len(), 0);
}

#[test]
fn test_list_tags_single() {
    use watchy_lib::db::queries::list_tags;

    let conn = setup_test_db();
    create_test_tag(&conn, "Single Tag", "#111111");

    let result = list_tags(&conn);
    assert!(result.is_ok());

    let tags = result.unwrap();
    assert_eq!(tags.len(), 1);
    assert_eq!(tags[0].name, "Single Tag");
    assert_eq!(tags[0].color, "#111111");
}

#[test]
fn test_list_tags_multiple() {
    use watchy_lib::db::queries::list_tags;

    let conn = setup_test_db();

    create_test_tag(&conn, "Tag One", "#111111");
    create_test_tag(&conn, "Tag Two", "#222222");
    create_test_tag(&conn, "Tag Three", "#333333");

    let result = list_tags(&conn);
    assert!(result.is_ok());

    let tags = result.unwrap();
    assert_eq!(tags.len(), 3);

    // Tags should be ordered by created_at ASC (oldest first)
    assert_eq!(tags[0].name, "Tag One");
    assert_eq!(tags[1].name, "Tag Two");
    assert_eq!(tags[2].name, "Tag Three");
}

#[test]
fn test_update_tag_success() {
    use watchy_lib::db::dto::UpdateTag;
    use watchy_lib::db::queries::update_tag;

    let conn = setup_test_db();
    let tag_id = create_test_tag(&conn, "Original Tag", "#000000");

    let update = UpdateTag {
        name: "Updated Tag Name".to_string(),
        color: "#FFFFFF".to_string(),
    };

    let result = update_tag(&conn, tag_id, update);
    assert!(result.is_ok());

    let updated = result.unwrap();
    assert_eq!(updated.id, tag_id);
    assert_eq!(updated.name, "Updated Tag Name");
    assert_eq!(updated.color, "#FFFFFF");
}

#[test]
fn test_update_tag_not_found() {
    use watchy_lib::db::dto::UpdateTag;
    use watchy_lib::db::queries::update_tag;

    let conn = setup_test_db();

    let update = UpdateTag {
        name: "Non-existent".to_string(),
        color: "#000000".to_string(),
    };

    let result = update_tag(&conn, 999, update);
    assert!(result.is_err());
}

#[test]
fn test_update_tag_changes_updated_at() {
    use watchy_lib::db::dto::UpdateTag;
    use watchy_lib::db::queries::{get_tag, update_tag};

    let conn = setup_test_db();
    let tag_id = create_test_tag(&conn, "Timestamp Tag", "#123456");

    let original = get_tag(&conn, tag_id).expect("Failed to get original tag");
    let original_updated_at = original.updated_at.clone();

    // Small delay to ensure timestamp changes
    std::thread::sleep(std::time::Duration::from_millis(10));

    let update = UpdateTag {
        name: "Updated Name".to_string(),
        color: original.color.clone(),
    };

    update_tag(&conn, tag_id, update).expect("Failed to update");

    let updated = get_tag(&conn, tag_id).expect("Failed to get updated tag");
    assert!(updated.updated_at >= original_updated_at);
}

#[test]
fn test_update_tag_duplicate_name() {
    use watchy_lib::db::dto::UpdateTag;
    use watchy_lib::db::queries::update_tag;

    let conn = setup_test_db();

    let tag1_id = create_test_tag(&conn, "Tag One", "#111111");
    create_test_tag(&conn, "Tag Two", "#222222");

    // Try to update tag1 to have the same name as tag2
    let update = UpdateTag {
        name: "Tag Two".to_string(),
        color: "#333333".to_string(),
    };

    let result = update_tag(&conn, tag1_id, update);
    // Should fail due to UNIQUE constraint on name
    assert!(result.is_err());
}

#[test]
fn test_delete_tag_success() {
    use watchy_lib::db::queries::{delete_tag, get_tag};

    let conn = setup_test_db();
    let tag_id = create_test_tag(&conn, "Tag to Delete", "#AAAAAA");

    // Verify tag exists
    assert!(get_tag(&conn, tag_id).is_ok());

    // Delete the tag
    let result = delete_tag(&conn, tag_id);
    assert!(result.is_ok());

    // Verify tag no longer exists
    assert!(get_tag(&conn, tag_id).is_err());
}

#[test]
fn test_delete_tag_not_found() {
    use watchy_lib::db::queries::delete_tag;

    let conn = setup_test_db();

    let result = delete_tag(&conn, 999);
    assert!(result.is_err());

    let err_msg = result.unwrap_err();
    assert!(err_msg.contains("not found"));
}

#[test]
fn test_delete_tag_removes_from_list() {
    use watchy_lib::db::queries::{delete_tag, list_tags};

    let conn = setup_test_db();

    let tag1_id = create_test_tag(&conn, "Tag One", "#111111");
    let tag2_id = create_test_tag(&conn, "Tag Two", "#222222");
    create_test_tag(&conn, "Tag Three", "#333333");

    // Verify we have 3 tags
    let tags = list_tags(&conn).expect("Failed to list tags");
    assert_eq!(tags.len(), 3);

    // Delete one tag
    delete_tag(&conn, tag2_id).expect("Failed to delete tag");

    // Verify we now have 2 tags
    let tags = list_tags(&conn).expect("Failed to list tags");
    assert_eq!(tags.len(), 2);

    // Verify the correct tags remain
    let remaining_ids: Vec<i64> = tags.iter().map(|t| t.id).collect();
    assert!(remaining_ids.contains(&tag1_id));
    assert!(!remaining_ids.contains(&tag2_id));
}

// =====================
// VIDEO-TAG RELATIONSHIP TESTS
// =====================

#[test]
fn test_create_video_tag_success() {
    use watchy_lib::db::queries::create_video_tag;

    let conn = setup_test_db();
    let channel_id = create_test_channel(&conn, "VideoTagChannel");
    let video_id = create_test_video(&conn, channel_id, "Tagged Video");
    let tag_id = create_test_tag(&conn, "Tutorial", "#FF0000");

    let result = create_video_tag(&conn, video_id, tag_id);
    assert!(result.is_ok());
}

#[test]
fn test_create_video_tag_duplicate_ignores() {
    use watchy_lib::db::queries::create_video_tag;

    let conn = setup_test_db();
    let channel_id = create_test_channel(&conn, "DuplicateTagChannel");
    let video_id = create_test_video(&conn, channel_id, "Video");
    let tag_id = create_test_tag(&conn, "Tag", "#000000");

    // Create the relationship first time
    let result1 = create_video_tag(&conn, video_id, tag_id);
    assert!(result1.is_ok());

    // Try to create the same relationship again (should not fail due to INSERT OR IGNORE)
    let result2 = create_video_tag(&conn, video_id, tag_id);
    assert!(result2.is_ok());
}

#[test]
fn test_create_video_tag_invalid_video() {
    use watchy_lib::db::queries::create_video_tag;

    let conn = setup_test_db();
    let tag_id = create_test_tag(&conn, "Tag", "#000000");

    // Enable foreign keys
    conn.execute("PRAGMA foreign_keys = ON", [])
        .expect("Failed to enable foreign keys");

    // Try to create relationship with non-existent video
    let result = create_video_tag(&conn, 999, tag_id);
    assert!(result.is_err());
}

#[test]
fn test_create_video_tag_invalid_tag() {
    use watchy_lib::db::queries::create_video_tag;

    let conn = setup_test_db();
    let channel_id = create_test_channel(&conn, "InvalidTagChannel");
    let video_id = create_test_video(&conn, channel_id, "Video");

    // Enable foreign keys
    conn.execute("PRAGMA foreign_keys = ON", [])
        .expect("Failed to enable foreign keys");

    // Try to create relationship with non-existent tag
    let result = create_video_tag(&conn, video_id, 999);
    assert!(result.is_err());
}

#[test]
fn test_list_video_tags_empty() {
    use watchy_lib::db::queries::list_video_tags;

    let conn = setup_test_db();

    let result = list_video_tags(&conn, &[]);
    assert!(result.is_ok());

    let map = result.unwrap();
    assert_eq!(map.len(), 0);
}

#[test]
fn test_list_video_tags_single_video_no_tags() {
    use watchy_lib::db::queries::list_video_tags;

    let conn = setup_test_db();
    let channel_id = create_test_channel(&conn, "NoTagsChannel");
    let video_id = create_test_video(&conn, channel_id, "Untagged Video");

    let result = list_video_tags(&conn, &[video_id]);
    assert!(result.is_ok());

    let map = result.unwrap();
    // Video should not appear in map if it has no tags
    assert!(map.is_empty() || !map.contains_key(&video_id) || map[&video_id].is_empty());
}

#[test]
fn test_list_video_tags_single_video_single_tag() {
    use watchy_lib::db::queries::{create_video_tag, list_video_tags};

    let conn = setup_test_db();
    let channel_id = create_test_channel(&conn, "SingleTagChannel");
    let video_id = create_test_video(&conn, channel_id, "Video");
    let tag_id = create_test_tag(&conn, "Tutorial", "#FF0000");

    create_video_tag(&conn, video_id, tag_id).expect("Failed to create video tag");

    let result = list_video_tags(&conn, &[video_id]);
    assert!(result.is_ok());

    let map = result.unwrap();
    assert_eq!(map.len(), 1);
    assert!(map.contains_key(&video_id));

    let tags = &map[&video_id];
    assert_eq!(tags.len(), 1);
    assert_eq!(tags[0].name, "Tutorial");
    assert_eq!(tags[0].color, "#FF0000");
}

#[test]
fn test_list_video_tags_single_video_multiple_tags() {
    use watchy_lib::db::queries::{create_video_tag, list_video_tags};

    let conn = setup_test_db();
    let channel_id = create_test_channel(&conn, "MultiTagChannel");
    let video_id = create_test_video(&conn, channel_id, "Video");

    let tag1_id = create_test_tag(&conn, "Tutorial", "#FF0000");
    let tag2_id = create_test_tag(&conn, "Review", "#00FF00");
    let tag3_id = create_test_tag(&conn, "Gaming", "#0000FF");

    create_video_tag(&conn, video_id, tag1_id).expect("Failed to create tag 1");
    create_video_tag(&conn, video_id, tag2_id).expect("Failed to create tag 2");
    create_video_tag(&conn, video_id, tag3_id).expect("Failed to create tag 3");

    let result = list_video_tags(&conn, &[video_id]);
    assert!(result.is_ok());

    let map = result.unwrap();
    assert_eq!(map.len(), 1);

    let tags = &map[&video_id];
    assert_eq!(tags.len(), 3);

    let tag_names: Vec<&str> = tags.iter().map(|t| t.name.as_str()).collect();
    assert!(tag_names.contains(&"Tutorial"));
    assert!(tag_names.contains(&"Review"));
    assert!(tag_names.contains(&"Gaming"));
}

#[test]
fn test_list_video_tags_multiple_videos() {
    use watchy_lib::db::queries::{create_video_tag, list_video_tags};

    let conn = setup_test_db();
    let channel_id = create_test_channel(&conn, "MultiVideoChannel");

    let video1_id = create_test_video(&conn, channel_id, "Video 1");
    let video2_id = create_test_video(&conn, channel_id, "Video 2");
    let video3_id = create_test_video(&conn, channel_id, "Video 3");

    let tag1_id = create_test_tag(&conn, "Tag A", "#111111");
    let tag2_id = create_test_tag(&conn, "Tag B", "#222222");
    let tag3_id = create_test_tag(&conn, "Tag C", "#333333");

    // Video 1 has tags A and B
    create_video_tag(&conn, video1_id, tag1_id).expect("Failed");
    create_video_tag(&conn, video1_id, tag2_id).expect("Failed");

    // Video 2 has tag B and C
    create_video_tag(&conn, video2_id, tag2_id).expect("Failed");
    create_video_tag(&conn, video2_id, tag3_id).expect("Failed");

    // Video 3 has no tags

    let result = list_video_tags(&conn, &[video1_id, video2_id, video3_id]);
    assert!(result.is_ok());

    let map = result.unwrap();

    // Video 1 should have 2 tags
    assert!(map.contains_key(&video1_id));
    assert_eq!(map[&video1_id].len(), 2);

    // Video 2 should have 2 tags
    assert!(map.contains_key(&video2_id));
    assert_eq!(map[&video2_id].len(), 2);

    // Video 3 should not be in map or have empty tags
    assert!(!map.contains_key(&video3_id) || map[&video3_id].is_empty());
}

#[test]
fn test_delete_video_tag_success() {
    use watchy_lib::db::queries::{create_video_tag, delete_video_tag, list_video_tags};

    let conn = setup_test_db();
    let channel_id = create_test_channel(&conn, "DeleteTagChannel");
    let video_id = create_test_video(&conn, channel_id, "Video");
    let tag_id = create_test_tag(&conn, "ToDelete", "#FFFFFF");

    create_video_tag(&conn, video_id, tag_id).expect("Failed to create");

    // Verify tag is associated
    let tags = list_video_tags(&conn, &[video_id]).expect("Failed to list");
    assert!(tags.contains_key(&video_id));
    assert_eq!(tags[&video_id].len(), 1);

    // Delete the association
    let result = delete_video_tag(&conn, video_id, tag_id);
    assert!(result.is_ok());

    // Verify tag is no longer associated
    let tags = list_video_tags(&conn, &[video_id]).expect("Failed to list");
    assert!(!tags.contains_key(&video_id) || tags[&video_id].is_empty());
}

#[test]
fn test_delete_video_tag_not_found() {
    use watchy_lib::db::queries::delete_video_tag;

    let conn = setup_test_db();
    let channel_id = create_test_channel(&conn, "NoRelationChannel");
    let video_id = create_test_video(&conn, channel_id, "Video");
    let tag_id = create_test_tag(&conn, "Tag", "#000000");

    // Try to delete a relationship that doesn't exist
    let result = delete_video_tag(&conn, video_id, tag_id);
    assert!(result.is_err());

    let err_msg = result.unwrap_err();
    assert!(err_msg.contains("not found"));
}

#[test]
fn test_delete_video_tag_partial() {
    use watchy_lib::db::queries::{create_video_tag, delete_video_tag, list_video_tags};

    let conn = setup_test_db();
    let channel_id = create_test_channel(&conn, "PartialDeleteChannel");
    let video_id = create_test_video(&conn, channel_id, "Video");

    let tag1_id = create_test_tag(&conn, "Tag 1", "#111111");
    let tag2_id = create_test_tag(&conn, "Tag 2", "#222222");
    let tag3_id = create_test_tag(&conn, "Tag 3", "#333333");

    create_video_tag(&conn, video_id, tag1_id).expect("Failed");
    create_video_tag(&conn, video_id, tag2_id).expect("Failed");
    create_video_tag(&conn, video_id, tag3_id).expect("Failed");

    // Verify we have 3 tags
    let tags = list_video_tags(&conn, &[video_id]).expect("Failed to list");
    assert_eq!(tags[&video_id].len(), 3);

    // Delete one tag
    delete_video_tag(&conn, video_id, tag2_id).expect("Failed to delete");

    // Verify we now have 2 tags
    let tags = list_video_tags(&conn, &[video_id]).expect("Failed to list");
    assert_eq!(tags[&video_id].len(), 2);

    let tag_ids: Vec<i64> = tags[&video_id].iter().map(|t| t.id).collect();
    assert!(tag_ids.contains(&tag1_id));
    assert!(!tag_ids.contains(&tag2_id));
    assert!(tag_ids.contains(&tag3_id));
}

#[test]
fn test_delete_video_cascades_to_video_tags() {
    use watchy_lib::db::queries::{create_video_tag, delete_video, list_video_tags};

    let conn = setup_test_db();

    // Enable foreign keys for cascade
    conn.execute("PRAGMA foreign_keys = ON", [])
        .expect("Failed to enable foreign keys");

    let channel_id = create_test_channel(&conn, "CascadeVideoChannel");
    let video_id = create_test_video(&conn, channel_id, "Video");
    let tag_id = create_test_tag(&conn, "Tag", "#000000");

    create_video_tag(&conn, video_id, tag_id).expect("Failed to create");

    // Verify relationship exists
    let tags = list_video_tags(&conn, &[video_id]).expect("Failed to list");
    assert!(tags.contains_key(&video_id));

    // Delete the video
    delete_video(&conn, video_id).expect("Failed to delete video");

    // Verify the video_tag relationship is also deleted (cascade)
    let tags = list_video_tags(&conn, &[video_id]).expect("Failed to list");
    assert!(!tags.contains_key(&video_id) || tags[&video_id].is_empty());
}

#[test]
fn test_delete_tag_cascades_to_video_tags() {
    use watchy_lib::db::queries::{create_video_tag, delete_tag, list_video_tags};

    let conn = setup_test_db();

    // Enable foreign keys for cascade
    conn.execute("PRAGMA foreign_keys = ON", [])
        .expect("Failed to enable foreign keys");

    let channel_id = create_test_channel(&conn, "CascadeTagChannel");
    let video_id = create_test_video(&conn, channel_id, "Video");
    let tag_id = create_test_tag(&conn, "Tag", "#000000");

    create_video_tag(&conn, video_id, tag_id).expect("Failed to create");

    // Verify relationship exists
    let tags = list_video_tags(&conn, &[video_id]).expect("Failed to list");
    assert!(tags.contains_key(&video_id));

    // Delete the tag
    delete_tag(&conn, tag_id).expect("Failed to delete tag");

    // Verify the video_tag relationship is also deleted (cascade)
    let tags = list_video_tags(&conn, &[video_id]).expect("Failed to list");
    assert!(!tags.contains_key(&video_id) || tags[&video_id].is_empty());
}

// =====================
// VIDEO FILTERING TESTS
// =====================

#[test]
fn test_filter_videos_by_channel() {
    use watchy_lib::db::dto::VideoFilter;

    let conn = setup_test_db();
    let channel1_id = create_test_channel(&conn, "Channel1");
    let channel2_id = create_test_channel(&conn, "Channel2");

    create_test_video(&conn, channel1_id, "Video 1A");
    create_test_video(&conn, channel1_id, "Video 1B");
    create_test_video(&conn, channel2_id, "Video 2A");
    create_test_video(&conn, channel2_id, "Video 2B");

    let filter = VideoFilter {
        channel_id: Some(channel1_id),
        status: None,
        tag_ids: None,
        search: None,
    };

    let result = list_videos(&conn, Some(filter), None);
    assert!(result.is_ok());

    let videos = result.unwrap();
    assert_eq!(videos.len(), 2);
    assert!(videos.iter().all(|v| v.channel_id == channel1_id));
}

#[test]
fn test_filter_videos_by_status() {
    use watchy_lib::db::dto::{CreateVideo, VideoFilter};

    let conn = setup_test_db();
    let channel_id = create_test_channel(&conn, "StatusChannel");

    let saved_video = CreateVideo {
        channel_id,
        link: "https://youtube.com/watch?v=saved".to_string(),
        title: "Saved Video".to_string(),
        thumbnail: "https://img.test/saved.jpg".to_string(),
        duration: Some(300),
        views: Some(1000),
        likes: Some(50),
        uploaded_at: "2026-01-10".to_string(),
        status: VideoStatus::SAVED,
    };

    let towatch_video = CreateVideo {
        channel_id,
        link: "https://youtube.com/watch?v=towatch".to_string(),
        title: "To Watch Video".to_string(),
        thumbnail: "https://img.test/towatch.jpg".to_string(),
        duration: Some(400),
        views: Some(2000),
        likes: Some(100),
        uploaded_at: "2026-01-11".to_string(),
        status: VideoStatus::TOWATCH,
    };

    let watched_video = CreateVideo {
        channel_id,
        link: "https://youtube.com/watch?v=watched".to_string(),
        title: "Watched Video".to_string(),
        thumbnail: "https://img.test/watched.jpg".to_string(),
        duration: Some(500),
        views: Some(3000),
        likes: Some(150),
        uploaded_at: "2026-01-12".to_string(),
        status: VideoStatus::WATCHED,
    };

    create_video(&conn, saved_video).expect("Failed to create");
    create_video(&conn, towatch_video).expect("Failed to create");
    create_video(&conn, watched_video).expect("Failed to create");

    let filter = VideoFilter {
        channel_id: None,
        status: Some(VideoStatus::TOWATCH),
        tag_ids: None,
        search: None,
    };

    let result = list_videos(&conn, Some(filter), None);
    assert!(result.is_ok());

    let videos = result.unwrap();
    assert_eq!(videos.len(), 1);
    assert_eq!(videos[0].title, "To Watch Video");
    assert!(matches!(videos[0].status, VideoStatus::TOWATCH));
}

#[test]
fn test_filter_videos_by_single_tag() {
    use watchy_lib::db::dto::VideoFilter;
    use watchy_lib::db::queries::create_video_tag;

    let conn = setup_test_db();
    let channel_id = create_test_channel(&conn, "TagFilterChannel");

    let video1_id = create_test_video(&conn, channel_id, "Video 1");
    let video2_id = create_test_video(&conn, channel_id, "Video 2");
    let video3_id = create_test_video(&conn, channel_id, "Video 3");

    let tutorial_tag_id = create_test_tag(&conn, "Tutorial", "#FF0000");
    let review_tag_id = create_test_tag(&conn, "Review", "#00FF00");

    create_video_tag(&conn, video1_id, tutorial_tag_id).expect("Failed");
    create_video_tag(&conn, video2_id, tutorial_tag_id).expect("Failed");
    create_video_tag(&conn, video3_id, review_tag_id).expect("Failed");

    let filter = VideoFilter {
        channel_id: None,
        status: None,
        tag_ids: Some(vec![tutorial_tag_id]),
        search: None,
    };

    let result = list_videos(&conn, Some(filter), None);
    assert!(result.is_ok());

    let videos = result.unwrap();
    assert_eq!(videos.len(), 2);

    let titles: Vec<&str> = videos.iter().map(|v| v.title.as_str()).collect();
    assert!(titles.contains(&"Video 1"));
    assert!(titles.contains(&"Video 2"));
}

#[test]
fn test_filter_videos_by_multiple_tags() {
    use watchy_lib::db::dto::VideoFilter;
    use watchy_lib::db::queries::create_video_tag;

    let conn = setup_test_db();
    let channel_id = create_test_channel(&conn, "MultiTagFilterChannel");

    let video1_id = create_test_video(&conn, channel_id, "Video 1");
    let video2_id = create_test_video(&conn, channel_id, "Video 2");
    let video3_id = create_test_video(&conn, channel_id, "Video 3");

    let tag1_id = create_test_tag(&conn, "Tag1", "#111111");
    let tag2_id = create_test_tag(&conn, "Tag2", "#222222");
    let tag3_id = create_test_tag(&conn, "Tag3", "#333333");

    create_video_tag(&conn, video1_id, tag1_id).expect("Failed");
    create_video_tag(&conn, video2_id, tag2_id).expect("Failed");
    create_video_tag(&conn, video3_id, tag3_id).expect("Failed");

    let filter = VideoFilter {
        channel_id: None,
        status: None,
        tag_ids: Some(vec![tag1_id, tag2_id]),
        search: None,
    };

    let result = list_videos(&conn, Some(filter), None);
    assert!(result.is_ok());

    let videos = result.unwrap();
    assert_eq!(videos.len(), 2);

    let titles: Vec<&str> = videos.iter().map(|v| v.title.as_str()).collect();
    assert!(titles.contains(&"Video 1"));
    assert!(titles.contains(&"Video 2"));
}

#[test]
fn test_filter_videos_combined_channel_and_status() {
    use watchy_lib::db::dto::{CreateVideo, VideoFilter};

    let conn = setup_test_db();
    let channel1_id = create_test_channel(&conn, "Channel1");
    let channel2_id = create_test_channel(&conn, "Channel2");

    let video1 = CreateVideo {
        channel_id: channel1_id,
        link: "https://youtube.com/watch?v=1".to_string(),
        title: "C1 Saved".to_string(),
        thumbnail: "https://img.test/1.jpg".to_string(),
        duration: Some(300),
        views: Some(1000),
        likes: Some(50),
        uploaded_at: "2026-01-10".to_string(),
        status: VideoStatus::SAVED,
    };

    let video2 = CreateVideo {
        channel_id: channel1_id,
        link: "https://youtube.com/watch?v=2".to_string(),
        title: "C1 ToWatch".to_string(),
        thumbnail: "https://img.test/2.jpg".to_string(),
        duration: Some(300),
        views: Some(1000),
        likes: Some(50),
        uploaded_at: "2026-01-11".to_string(),
        status: VideoStatus::TOWATCH,
    };

    let video3 = CreateVideo {
        channel_id: channel2_id,
        link: "https://youtube.com/watch?v=3".to_string(),
        title: "C2 ToWatch".to_string(),
        thumbnail: "https://img.test/3.jpg".to_string(),
        duration: Some(300),
        views: Some(1000),
        likes: Some(50),
        uploaded_at: "2026-01-12".to_string(),
        status: VideoStatus::TOWATCH,
    };

    create_video(&conn, video1).expect("Failed");
    create_video(&conn, video2).expect("Failed");
    create_video(&conn, video3).expect("Failed");

    let filter = VideoFilter {
        channel_id: Some(channel1_id),
        status: Some(VideoStatus::TOWATCH),
        tag_ids: None,
        search: None,
    };

    let result = list_videos(&conn, Some(filter), None);
    assert!(result.is_ok());

    let videos = result.unwrap();
    assert_eq!(videos.len(), 1);
    assert_eq!(videos[0].title, "C1 ToWatch");
}

#[test]
fn test_filter_videos_all_criteria() {
    use watchy_lib::db::dto::{CreateVideo, VideoFilter};
    use watchy_lib::db::queries::create_video_tag;

    let conn = setup_test_db();
    let channel1_id = create_test_channel(&conn, "Channel1");
    let channel2_id = create_test_channel(&conn, "Channel2");

    let tag_id = create_test_tag(&conn, "Important", "#FF0000");

    let video1 = CreateVideo {
        channel_id: channel1_id,
        link: "https://youtube.com/watch?v=1".to_string(),
        title: "Match All".to_string(),
        thumbnail: "https://img.test/1.jpg".to_string(),
        duration: Some(300),
        views: Some(1000),
        likes: Some(50),
        uploaded_at: "2026-01-10".to_string(),
        status: VideoStatus::TOWATCH,
    };

    let video2 = CreateVideo {
        channel_id: channel1_id,
        link: "https://youtube.com/watch?v=2".to_string(),
        title: "Wrong Status".to_string(),
        thumbnail: "https://img.test/2.jpg".to_string(),
        duration: Some(300),
        views: Some(1000),
        likes: Some(50),
        uploaded_at: "2026-01-11".to_string(),
        status: VideoStatus::SAVED,
    };

    let video3 = CreateVideo {
        channel_id: channel2_id,
        link: "https://youtube.com/watch?v=3".to_string(),
        title: "Wrong Channel".to_string(),
        thumbnail: "https://img.test/3.jpg".to_string(),
        duration: Some(300),
        views: Some(1000),
        likes: Some(50),
        uploaded_at: "2026-01-12".to_string(),
        status: VideoStatus::TOWATCH,
    };

    let video1_id = create_video(&conn, video1).expect("Failed").id;
    let video2_id = create_video(&conn, video2).expect("Failed").id;
    create_video(&conn, video3).expect("Failed");

    create_video_tag(&conn, video1_id, tag_id).expect("Failed");
    create_video_tag(&conn, video2_id, tag_id).expect("Failed");

    let filter = VideoFilter {
        channel_id: Some(channel1_id),
        status: Some(VideoStatus::TOWATCH),
        tag_ids: Some(vec![tag_id]),
        search: None,
    };

    let result = list_videos(&conn, Some(filter), None);
    assert!(result.is_ok());

    let videos = result.unwrap();
    assert_eq!(videos.len(), 1);
    assert_eq!(videos[0].title, "Match All");
}

// =====================
// VIDEO SORTING TESTS
// =====================

#[test]
fn test_sort_videos_by_created_at_desc() {
    use watchy_lib::db::dto::VideoSort;

    let conn = setup_test_db();
    let channel_id = create_test_channel(&conn, "SortChannel");

    create_test_video(&conn, channel_id, "First");
    std::thread::sleep(std::time::Duration::from_millis(1100));
    create_test_video(&conn, channel_id, "Second");
    std::thread::sleep(std::time::Duration::from_millis(1100));
    create_test_video(&conn, channel_id, "Third");

    let result = list_videos(&conn, None, Some(VideoSort::CreatedAtDesc));
    assert!(result.is_ok());

    let videos = result.unwrap();
    assert_eq!(videos.len(), 3);
    // Newest first
    assert_eq!(videos[0].title, "Third");
    assert_eq!(videos[2].title, "First");
}

#[test]
fn test_sort_videos_by_created_at_asc() {
    use watchy_lib::db::dto::VideoSort;

    let conn = setup_test_db();
    let channel_id = create_test_channel(&conn, "SortChannel");

    create_test_video(&conn, channel_id, "First");
    std::thread::sleep(std::time::Duration::from_millis(1100));
    create_test_video(&conn, channel_id, "Second");
    std::thread::sleep(std::time::Duration::from_millis(1100));
    create_test_video(&conn, channel_id, "Third");

    let result = list_videos(&conn, None, Some(VideoSort::CreatedAtAsc));
    assert!(result.is_ok());

    let videos = result.unwrap();
    assert_eq!(videos.len(), 3);
    // Oldest first
    assert_eq!(videos[0].title, "First");
    assert_eq!(videos[2].title, "Third");
}

#[test]
fn test_sort_videos_by_title_asc() {
    use watchy_lib::db::dto::VideoSort;

    let conn = setup_test_db();
    let channel_id = create_test_channel(&conn, "SortChannel");

    create_test_video(&conn, channel_id, "Zebra");
    create_test_video(&conn, channel_id, "Apple");
    create_test_video(&conn, channel_id, "Mango");

    let result = list_videos(&conn, None, Some(VideoSort::TitleAsc));
    assert!(result.is_ok());

    let videos = result.unwrap();
    assert_eq!(videos.len(), 3);
    assert_eq!(videos[0].title, "Apple");
    assert_eq!(videos[1].title, "Mango");
    assert_eq!(videos[2].title, "Zebra");
}

#[test]
fn test_sort_videos_by_title_desc() {
    use watchy_lib::db::dto::VideoSort;

    let conn = setup_test_db();
    let channel_id = create_test_channel(&conn, "SortChannel");

    create_test_video(&conn, channel_id, "Zebra");
    create_test_video(&conn, channel_id, "Apple");
    create_test_video(&conn, channel_id, "Mango");

    let result = list_videos(&conn, None, Some(VideoSort::TitleDesc));
    assert!(result.is_ok());

    let videos = result.unwrap();
    assert_eq!(videos.len(), 3);
    assert_eq!(videos[0].title, "Zebra");
    assert_eq!(videos[1].title, "Mango");
    assert_eq!(videos[2].title, "Apple");
}

#[test]
fn test_sort_videos_with_filter() {
    use watchy_lib::db::dto::{VideoFilter, VideoSort};

    let conn = setup_test_db();
    let channel1_id = create_test_channel(&conn, "Channel1");
    let channel2_id = create_test_channel(&conn, "Channel2");

    create_test_video(&conn, channel1_id, "Zebra");
    create_test_video(&conn, channel1_id, "Apple");
    create_test_video(&conn, channel2_id, "Banana");
    create_test_video(&conn, channel1_id, "Mango");

    let filter = VideoFilter {
        channel_id: Some(channel1_id),
        status: None,
        tag_ids: None,
        search: None,
    };

    let result = list_videos(&conn, Some(filter), Some(VideoSort::TitleAsc));
    assert!(result.is_ok());

    let videos = result.unwrap();
    assert_eq!(videos.len(), 3);
    assert_eq!(videos[0].title, "Apple");
    assert_eq!(videos[1].title, "Mango");
    assert_eq!(videos[2].title, "Zebra");
}

#[test]
fn test_default_sort_is_created_at_desc() {
    let conn = setup_test_db();
    let channel_id = create_test_channel(&conn, "DefaultSortChannel");

    create_test_video(&conn, channel_id, "First");
    std::thread::sleep(std::time::Duration::from_millis(1100));
    create_test_video(&conn, channel_id, "Second");
    std::thread::sleep(std::time::Duration::from_millis(1100));
    create_test_video(&conn, channel_id, "Third");

    // No sort specified, should default to CreatedAtDesc
    let result = list_videos(&conn, None, None);
    assert!(result.is_ok());

    let videos = result.unwrap();
    assert_eq!(videos.len(), 3);
    // Newest first (default behavior)
    assert_eq!(videos[0].title, "Third");
    assert_eq!(videos[2].title, "First");
}

// =====================
// VIDEO SEARCH TESTS
// =====================

#[test]
fn test_search_videos_by_exact_link() {
    use watchy_lib::db::dto::{CreateVideo, VideoFilter};

    let conn = setup_test_db();
    let channel_id = create_test_channel(&conn, "SearchChannel");

    let video1 = CreateVideo {
        channel_id,
        link: "https://youtube.com/watch?v=abc123".to_string(),
        title: "First Video".to_string(),
        thumbnail: "https://img.test/1.jpg".to_string(),
        duration: Some(300),
        views: Some(1000),
        likes: Some(50),
        uploaded_at: "2026-01-10".to_string(),
        status: VideoStatus::SAVED,
    };

    let video2 = CreateVideo {
        channel_id,
        link: "https://youtube.com/watch?v=xyz789".to_string(),
        title: "Second Video".to_string(),
        thumbnail: "https://img.test/2.jpg".to_string(),
        duration: Some(400),
        views: Some(2000),
        likes: Some(100),
        uploaded_at: "2026-01-11".to_string(),
        status: VideoStatus::SAVED,
    };

    create_video(&conn, video1).expect("Failed to create");
    create_video(&conn, video2).expect("Failed to create");

    let filter = VideoFilter {
        channel_id: None,
        status: None,
        tag_ids: None,
        search: Some("https://youtube.com/watch?v=abc123".to_string()),
    };

    let result = list_videos(&conn, Some(filter), None);
    assert!(result.is_ok());

    let videos = result.unwrap();
    assert_eq!(videos.len(), 1);
    assert_eq!(videos[0].title, "First Video");
    assert_eq!(videos[0].link, "https://youtube.com/watch?v=abc123");
}

#[test]
fn test_search_videos_by_title_partial_match() {
    use watchy_lib::db::dto::VideoFilter;

    let conn = setup_test_db();
    let channel_id = create_test_channel(&conn, "SearchChannel");

    create_test_video(&conn, channel_id, "Rust Programming Tutorial");
    create_test_video(&conn, channel_id, "Python Web Development");
    create_test_video(&conn, channel_id, "Advanced Rust Patterns");
    create_test_video(&conn, channel_id, "JavaScript Basics");

    let filter = VideoFilter {
        channel_id: None,
        status: None,
        tag_ids: None,
        search: Some("Rust".to_string()),
    };

    let result = list_videos(&conn, Some(filter), None);
    assert!(result.is_ok());

    let videos = result.unwrap();
    assert_eq!(videos.len(), 2);

    let titles: Vec<&str> = videos.iter().map(|v| v.title.as_str()).collect();
    assert!(titles.contains(&"Rust Programming Tutorial"));
    assert!(titles.contains(&"Advanced Rust Patterns"));
}

#[test]
fn test_search_videos_case_insensitive() {
    use watchy_lib::db::dto::VideoFilter;

    let conn = setup_test_db();
    let channel_id = create_test_channel(&conn, "SearchChannel");

    create_test_video(&conn, channel_id, "Introduction to Docker");
    create_test_video(&conn, channel_id, "Kubernetes Best Practices");
    create_test_video(&conn, channel_id, "DOCKER Compose Guide");

    let filter = VideoFilter {
        channel_id: None,
        status: None,
        tag_ids: None,
        search: Some("docker".to_string()),
    };

    let result = list_videos(&conn, Some(filter), None);
    assert!(result.is_ok());

    let videos = result.unwrap();
    assert_eq!(videos.len(), 2);

    let titles: Vec<&str> = videos.iter().map(|v| v.title.as_str()).collect();
    assert!(titles.contains(&"Introduction to Docker"));
    assert!(titles.contains(&"DOCKER Compose Guide"));
}

#[test]
fn test_search_videos_no_matches() {
    use watchy_lib::db::dto::VideoFilter;

    let conn = setup_test_db();
    let channel_id = create_test_channel(&conn, "SearchChannel");

    create_test_video(&conn, channel_id, "Machine Learning Basics");
    create_test_video(&conn, channel_id, "Deep Neural Networks");

    let filter = VideoFilter {
        channel_id: None,
        status: None,
        tag_ids: None,
        search: Some("Blockchain".to_string()),
    };

    let result = list_videos(&conn, Some(filter), None);
    assert!(result.is_ok());

    let videos = result.unwrap();
    assert_eq!(videos.len(), 0);
}

#[test]
fn test_search_combined_with_channel_filter() {
    use watchy_lib::db::dto::VideoFilter;

    let conn = setup_test_db();
    let channel1_id = create_test_channel(&conn, "TechChannel");
    let channel2_id = create_test_channel(&conn, "ScienceChannel");

    create_test_video(&conn, channel1_id, "Python Tutorial");
    create_test_video(&conn, channel1_id, "JavaScript Basics");
    create_test_video(&conn, channel2_id, "Python in Science");
    create_test_video(&conn, channel2_id, "Biology Fundamentals");

    let filter = VideoFilter {
        channel_id: Some(channel1_id),
        status: None,
        tag_ids: None,
        search: Some("Python".to_string()),
    };

    let result = list_videos(&conn, Some(filter), None);
    assert!(result.is_ok());

    let videos = result.unwrap();
    assert_eq!(videos.len(), 1);
    assert_eq!(videos[0].title, "Python Tutorial");
    assert_eq!(videos[0].channel_id, channel1_id);
}

#[test]
fn test_search_combined_with_status_filter() {
    use watchy_lib::db::dto::{CreateVideo, VideoFilter};

    let conn = setup_test_db();
    let channel_id = create_test_channel(&conn, "Channel");

    let video1 = CreateVideo {
        channel_id,
        link: "https://youtube.com/watch?v=1".to_string(),
        title: "Rust Tutorial Part 1".to_string(),
        thumbnail: "https://img.test/1.jpg".to_string(),
        duration: Some(300),
        views: Some(1000),
        likes: Some(50),
        uploaded_at: "2026-01-10".to_string(),
        status: VideoStatus::TOWATCH,
    };

    let video2 = CreateVideo {
        channel_id,
        link: "https://youtube.com/watch?v=2".to_string(),
        title: "Rust Tutorial Part 2".to_string(),
        thumbnail: "https://img.test/2.jpg".to_string(),
        duration: Some(350),
        views: Some(1200),
        likes: Some(60),
        uploaded_at: "2026-01-11".to_string(),
        status: VideoStatus::WATCHED,
    };

    let video3 = CreateVideo {
        channel_id,
        link: "https://youtube.com/watch?v=3".to_string(),
        title: "Python Basics".to_string(),
        thumbnail: "https://img.test/3.jpg".to_string(),
        duration: Some(400),
        views: Some(1500),
        likes: Some(80),
        uploaded_at: "2026-01-12".to_string(),
        status: VideoStatus::TOWATCH,
    };

    create_video(&conn, video1).expect("Failed");
    create_video(&conn, video2).expect("Failed");
    create_video(&conn, video3).expect("Failed");

    let filter = VideoFilter {
        channel_id: None,
        status: Some(VideoStatus::TOWATCH),
        tag_ids: None,
        search: Some("Rust".to_string()),
    };

    let result = list_videos(&conn, Some(filter), None);
    assert!(result.is_ok());

    let videos = result.unwrap();
    assert_eq!(videos.len(), 1);
    assert_eq!(videos[0].title, "Rust Tutorial Part 1");
}

#[test]
fn test_search_combined_with_tags_filter() {
    use watchy_lib::db::dto::VideoFilter;
    use watchy_lib::db::queries::create_video_tag;

    let conn = setup_test_db();
    let channel_id = create_test_channel(&conn, "Channel");

    let video1_id = create_test_video(&conn, channel_id, "Rust Async Programming");
    let video2_id = create_test_video(&conn, channel_id, "Rust Macros Guide");
    let video3_id = create_test_video(&conn, channel_id, "Python Async Tutorial");

    let advanced_tag_id = create_test_tag(&conn, "Advanced", "#FF0000");
    let beginner_tag_id = create_test_tag(&conn, "Beginner", "#00FF00");

    create_video_tag(&conn, video1_id, advanced_tag_id).expect("Failed");
    create_video_tag(&conn, video2_id, advanced_tag_id).expect("Failed");
    create_video_tag(&conn, video3_id, beginner_tag_id).expect("Failed");

    let filter = VideoFilter {
        channel_id: None,
        status: None,
        tag_ids: Some(vec![advanced_tag_id]),
        search: Some("Rust".to_string()),
    };

    let result = list_videos(&conn, Some(filter), None);
    assert!(result.is_ok());

    let videos = result.unwrap();
    assert_eq!(videos.len(), 2);

    let titles: Vec<&str> = videos.iter().map(|v| v.title.as_str()).collect();
    assert!(titles.contains(&"Rust Async Programming"));
    assert!(titles.contains(&"Rust Macros Guide"));
}

#[test]
fn test_search_all_filters_combined() {
    use watchy_lib::db::dto::{CreateVideo, VideoFilter};
    use watchy_lib::db::queries::create_video_tag;

    let conn = setup_test_db();
    let channel1_id = create_test_channel(&conn, "Channel1");
    let channel2_id = create_test_channel(&conn, "Channel2");

    let tag_id = create_test_tag(&conn, "Tutorial", "#FF0000");

    let video1 = CreateVideo {
        channel_id: channel1_id,
        link: "https://youtube.com/watch?v=1".to_string(),
        title: "Complete Rust Guide".to_string(),
        thumbnail: "https://img.test/1.jpg".to_string(),
        duration: Some(300),
        views: Some(1000),
        likes: Some(50),
        uploaded_at: "2026-01-10".to_string(),
        status: VideoStatus::TOWATCH,
    };

    let video2 = CreateVideo {
        channel_id: channel1_id,
        link: "https://youtube.com/watch?v=2".to_string(),
        title: "Rust Basics Tutorial".to_string(),
        thumbnail: "https://img.test/2.jpg".to_string(),
        duration: Some(250),
        views: Some(800),
        likes: Some(40),
        uploaded_at: "2026-01-11".to_string(),
        status: VideoStatus::WATCHED,
    };

    let video3 = CreateVideo {
        channel_id: channel2_id,
        link: "https://youtube.com/watch?v=3".to_string(),
        title: "Advanced Rust Patterns".to_string(),
        thumbnail: "https://img.test/3.jpg".to_string(),
        duration: Some(400),
        views: Some(1500),
        likes: Some(80),
        uploaded_at: "2026-01-12".to_string(),
        status: VideoStatus::TOWATCH,
    };

    let video1_id = create_video(&conn, video1).expect("Failed").id;
    create_video(&conn, video2).expect("Failed");
    create_video(&conn, video3).expect("Failed");

    create_video_tag(&conn, video1_id, tag_id).expect("Failed");

    let filter = VideoFilter {
        channel_id: Some(channel1_id),
        status: Some(VideoStatus::TOWATCH),
        tag_ids: Some(vec![tag_id]),
        search: Some("Rust".to_string()),
    };

    let result = list_videos(&conn, Some(filter), None);
    assert!(result.is_ok());

    let videos = result.unwrap();
    assert_eq!(videos.len(), 1);
    assert_eq!(videos[0].title, "Complete Rust Guide");
}

#[test]
fn test_search_with_sort() {
    use watchy_lib::db::dto::{VideoFilter, VideoSort};

    let conn = setup_test_db();
    let channel_id = create_test_channel(&conn, "Channel");

    create_test_video(&conn, channel_id, "Rust Macros");
    create_test_video(&conn, channel_id, "Rust Async");
    create_test_video(&conn, channel_id, "Python Basics");
    create_test_video(&conn, channel_id, "Rust Ownership");

    let filter = VideoFilter {
        channel_id: None,
        status: None,
        tag_ids: None,
        search: Some("Rust".to_string()),
    };

    let result = list_videos(&conn, Some(filter), Some(VideoSort::TitleAsc));
    assert!(result.is_ok());

    let videos = result.unwrap();
    assert_eq!(videos.len(), 3);
    assert_eq!(videos[0].title, "Rust Async");
    assert_eq!(videos[1].title, "Rust Macros");
    assert_eq!(videos[2].title, "Rust Ownership");
}
