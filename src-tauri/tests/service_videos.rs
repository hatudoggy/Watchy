use rusqlite::Connection;
use watchy_lib::{
    commands::youtube::{YoutubeChannelMetadata, YoutubeMetadata, YoutubeVideoMetadata},
    db::{
        dto::{UpdateVideo, VideoFilter, VideoSort},
        schema::VideoStatus,
        schema_init::init_schema,
    },
    services::videos,
};

fn setup_test_db() -> Connection {
    let conn = Connection::open_in_memory().expect("Failed to create in-memory database");
    init_schema(&conn).expect("Failed to initialize schema");
    conn
}

fn create_test_metadata(video_id: &str, title: &str, channel_name: &str) -> YoutubeMetadata {
    YoutubeMetadata {
        video: YoutubeVideoMetadata {
            link: format!("https://youtube.com/watch?v={}", video_id),
            title: title.to_string(),
            thumbnail: "https://img.test/thumb.jpg".to_string(),
            upload_date: "2026-01-15".to_string(),
            duration: Some(300),
            views: Some(1000),
            likes: Some(50),
        },
        channel: YoutubeChannelMetadata {
            link: format!("https://youtube.com/@{}", channel_name),
            name: channel_name.to_string(),
            avatar: "https://img.test/avatar.jpg".to_string(),
            subscribers: Some(100000),
            verified: true,
        },
    }
}

// =====================
// LIST VIDEOS TESTS
// =====================

#[test]
fn test_list_videos_with_filter() {
    let mut conn = setup_test_db();

    let metadata1 = create_test_metadata("abc123", "Video 1", "Channel1");
    let metadata2 = create_test_metadata("xyz789", "Video 2", "Channel2");

    let video1 = videos::add_video(&mut conn, metadata1, None).unwrap();
    videos::add_video(&mut conn, metadata2, None).unwrap();

    let filter = VideoFilter {
        channel_id: Some(video1.channel_id),
        status: None,
        tag_ids: None,
        search: None,
    };

    let result = videos::list_videos(&conn, Some(filter), None);
    assert!(result.is_ok());

    let vids = result.unwrap();
    assert_eq!(vids.len(), 1);
    assert_eq!(vids[0].video.title, "Video 1");
}

#[test]
fn test_list_videos_with_sort() {
    let mut conn = setup_test_db();

    let metadata1 = create_test_metadata("abc123", "Zebra Video", "Channel1");
    let metadata2 = create_test_metadata("xyz789", "Apple Video", "Channel1");

    videos::add_video(&mut conn, metadata1, None).unwrap();
    videos::add_video(&mut conn, metadata2, None).unwrap();

    let result = videos::list_videos(&conn, None, Some(VideoSort::TitleAsc));
    assert!(result.is_ok());

    let vids = result.unwrap();
    assert_eq!(vids.len(), 2);
    assert_eq!(vids[0].video.title, "Apple Video");
    assert_eq!(vids[1].video.title, "Zebra Video");
}

// =====================
// ADD VIDEO TESTS
// =====================

#[test]
fn test_add_video_creates_new_channel() {
    let mut conn = setup_test_db();

    let metadata = create_test_metadata("abc123", "Test Video", "TestChannel");

    let result = videos::add_video(&mut conn, metadata, None);
    assert!(result.is_ok());

    let video = result.unwrap();
    assert_eq!(video.title, "Test Video");
    assert_eq!(video.link, "https://youtube.com/watch?v=abc123");
    assert!(matches!(video.status, VideoStatus::SAVED));

    // Verify channel was created
    let channel = videos::get_channel_details(&conn, video.channel_id);
    assert!(channel.is_ok());
    assert_eq!(channel.unwrap().name, "TestChannel");
}

#[test]
fn test_add_video_reuses_existing_channel() {
    let mut conn = setup_test_db();

    let metadata1 = create_test_metadata("abc123", "Video 1", "SameChannel");
    let metadata2 = create_test_metadata("xyz789", "Video 2", "SameChannel");

    let video1 = videos::add_video(&mut conn, metadata1, None).unwrap();
    let video2 = videos::add_video(&mut conn, metadata2, None).unwrap();

    // Both videos should use the same channel
    assert_eq!(video1.channel_id, video2.channel_id);
}

#[test]
fn test_add_video_duplicate_link_fails() {
    let mut conn = setup_test_db();

    let metadata1 = create_test_metadata("abc123", "Video 1", "Channel1");
    let metadata2 = create_test_metadata("abc123", "Video 2", "Channel2");

    let result1 = videos::add_video(&mut conn, metadata1, None);
    assert!(result1.is_ok());

    let result2 = videos::add_video(&mut conn, metadata2, None);
    assert!(result2.is_err());
}

#[test]
fn test_add_video_sets_default_status() {
    let mut conn = setup_test_db();

    let metadata = create_test_metadata("abc123", "Test Video", "TestChannel");
    let video = videos::add_video(&mut conn, metadata, None).unwrap();

    assert!(matches!(video.status, VideoStatus::SAVED));
}

// =====================
// EDIT VIDEO TESTS
// =====================

#[test]
fn test_edit_video_success() {
    let mut conn = setup_test_db();

    let metadata = create_test_metadata("abc123", "Original Title", "Channel1");
    let video = videos::add_video(&mut conn, metadata, None).unwrap();

    let update = UpdateVideo {
        link: Some(video.link.clone()),
        title: Some("Updated Title".to_string()),
        thumbnail: Some(video.thumbnail),
        duration: Some(500),
        views: Some(2000),
        likes: Some(100),
        uploaded_at: Some("2026-01-16".to_string()),
        status: Some(VideoStatus::WATCHED),
    };

    let result = videos::edit_video(&conn, video.id, update);
    assert!(result.is_ok());

    let updated = result.unwrap();
    assert_eq!(updated.title, "Updated Title");
    assert_eq!(updated.duration, Some(500));
    assert!(matches!(updated.status, VideoStatus::WATCHED));
}

#[test]
fn test_edit_video_not_found() {
    let conn = setup_test_db();

    let update = UpdateVideo {
        link: Some("https://youtube.com/watch?v=nonexistent".to_string()),
        title: Some("Title".to_string()),
        thumbnail: Some("thumb.jpg".to_string()),
        duration: Some(300),
        views: Some(1000),
        likes: Some(50),
        uploaded_at: Some("2026-01-15".to_string()),
        status: Some(VideoStatus::SAVED),
    };

    // The UPDATE succeeds with 0 rows, but the subsequent SELECT fails
    let result = videos::edit_video(&conn, 99999, update);
    assert!(result.is_err());
}

#[test]
fn test_edit_video_changes_status() {
    let mut conn = setup_test_db();

    let metadata = create_test_metadata("abc123", "Test Video", "Channel1");
    let video = videos::add_video(&mut conn, metadata, None).unwrap();

    assert!(matches!(video.status, VideoStatus::SAVED));

    let update = UpdateVideo {
        link: Some(video.link.clone()),
        title: Some(video.title.clone()),
        thumbnail: Some(video.thumbnail.clone()),
        duration: video.duration,
        views: video.views,
        likes: video.likes,
        uploaded_at: Some(video.uploaded_at.clone()),
        status: Some(VideoStatus::TOWATCH),
    };

    let updated = videos::edit_video(&conn, video.id, update).unwrap();
    assert!(matches!(updated.status, VideoStatus::TOWATCH));
}

// =====================
// DELETE VIDEO TESTS
// =====================

#[test]
fn test_delete_video_success() {
    let mut conn = setup_test_db();

    let metadata = create_test_metadata("abc123", "Test Video", "Channel1");
    let video = videos::add_video(&mut conn, metadata, None).unwrap();

    let result = videos::delete_video(&conn, video.id);
    assert!(result.is_ok());

    // Verify video is gone
    let all_videos = videos::list_videos(&conn, None, None).unwrap();
    assert_eq!(all_videos.len(), 0);
}

#[test]
fn test_delete_video_not_found() {
    let conn = setup_test_db();

    let result = videos::delete_video(&conn, 99999);
    assert!(result.is_err());
    assert!(result.unwrap_err().contains("not found"));
}

#[test]
fn test_delete_video_does_not_delete_channel() {
    let mut conn = setup_test_db();

    let metadata = create_test_metadata("abc123", "Test Video", "TestChannel");
    let video = videos::add_video(&mut conn, metadata, None).unwrap();

    let channel_id = video.channel_id;

    videos::delete_video(&conn, video.id).unwrap();

    // Channel should still exist
    let channel = videos::get_channel_details(&conn, channel_id);
    assert!(channel.is_ok());
}

// =====================
// TAG MANAGEMENT TESTS
// =====================

#[test]
fn test_add_tag_to_video_creates_new_tag() {
    let mut conn = setup_test_db();

    let metadata = create_test_metadata("abc123", "Test Video", "Channel1");
    let video = videos::add_video(&mut conn, metadata, None).unwrap();

    let result = videos::add_tag_to_video(&mut conn, video.id, "Tutorial".to_string());
    assert!(result.is_ok());

    let tag = result.unwrap();
    assert_eq!(tag.name, "Tutorial");
    assert_eq!(tag.color, "#000000");
}

#[test]
fn test_add_tag_to_video_reuses_existing_tag() {
    let mut conn = setup_test_db();

    let metadata1 = create_test_metadata("abc123", "Video 1", "Channel1");
    let metadata2 = create_test_metadata("xyz789", "Video 2", "Channel1");

    let video1 = videos::add_video(&mut conn, metadata1, None).unwrap();
    let video2 = videos::add_video(&mut conn, metadata2, None).unwrap();

    let tag1 = videos::add_tag_to_video(&mut conn, video1.id, "Tutorial".to_string()).unwrap();
    let tag2 = videos::add_tag_to_video(&mut conn, video2.id, "Tutorial".to_string()).unwrap();

    // Should be the same tag
    assert_eq!(tag1.id, tag2.id);
}

#[test]
fn test_add_multiple_tags_to_video() {
    let mut conn = setup_test_db();

    let metadata = create_test_metadata("abc123", "Test Video", "Channel1");
    let video = videos::add_video(&mut conn, metadata, None).unwrap();

    let tag1 = videos::add_tag_to_video(&mut conn, video.id, "Tutorial".to_string()).unwrap();
    let tag2 = videos::add_tag_to_video(&mut conn, video.id, "Beginner".to_string()).unwrap();

    assert_ne!(tag1.id, tag2.id);
}

#[test]
fn test_add_tag_to_nonexistent_video_fails() {
    let mut conn = setup_test_db();

    let result = videos::add_tag_to_video(&mut conn, 99999, "Tutorial".to_string());
    assert!(result.is_err());
}

#[test]
fn test_remove_tag_from_video_success() {
    let mut conn = setup_test_db();

    let metadata = create_test_metadata("abc123", "Test Video", "Channel1");
    let video = videos::add_video(&mut conn, metadata, None).unwrap();
    let tag = videos::add_tag_to_video(&mut conn, video.id, "Tutorial".to_string()).unwrap();

    let result = videos::remove_tag_from_video(&conn, video.id, tag.id);
    assert!(result.is_ok());
}

#[test]
fn test_remove_tag_from_video_not_found() {
    let mut conn = setup_test_db();

    let metadata = create_test_metadata("abc123", "Test Video", "Channel1");
    let video = videos::add_video(&mut conn, metadata, None).unwrap();

    let result = videos::remove_tag_from_video(&conn, video.id, 99999);
    assert!(result.is_err());
    assert!(result.unwrap_err().contains("not found"));
}

// =====================
// CHANNEL DETAILS TESTS
// =====================

#[test]
fn test_get_channel_details_success() {
    let mut conn = setup_test_db();

    let metadata = create_test_metadata("abc123", "Test Video", "TestChannel");
    let video = videos::add_video(&mut conn, metadata, None).unwrap();

    let result = videos::get_channel_details(&conn, video.channel_id);
    assert!(result.is_ok());

    let channel = result.unwrap();
    assert_eq!(channel.name, "TestChannel");
    assert_eq!(channel.link, "https://youtube.com/@TestChannel");
    assert_eq!(channel.subscribers, Some(100000));
    assert!(channel.verified);
}

#[test]
fn test_get_channel_details_not_found() {
    let conn = setup_test_db();

    let result = videos::get_channel_details(&conn, 99999);
    assert!(result.is_err());
}

// =====================
// INTEGRATION TESTS
// =====================

#[test]
fn test_complete_video_workflow() {
    let mut conn = setup_test_db();

    // 1. Add video with new channel
    let metadata = create_test_metadata("abc123", "Learning Rust", "RustChannel");
    let video = videos::add_video(&mut conn, metadata, None).unwrap();
    assert!(matches!(video.status, VideoStatus::SAVED));

    // 2. Add tags to video
    let tag1 = videos::add_tag_to_video(&mut conn, video.id, "Programming".to_string()).unwrap();
    let tag2 = videos::add_tag_to_video(&mut conn, video.id, "Tutorial".to_string()).unwrap();

    // 3. Update video status
    let update = UpdateVideo {
        link: Some(video.link.clone()),
        title: Some(video.title.clone()),
        thumbnail: Some(video.thumbnail.clone()),
        duration: video.duration,
        views: video.views,
        likes: video.likes,
        uploaded_at: Some(video.uploaded_at.clone()),
        status: Some(VideoStatus::WATCHED),
    };
    let updated_video = videos::edit_video(&conn, video.id, update).unwrap();
    assert!(matches!(updated_video.status, VideoStatus::WATCHED));

    // 4. List videos with filter
    let filter = VideoFilter {
        channel_id: Some(video.channel_id),
        status: Some(VideoStatus::WATCHED),
        tag_ids: Some(vec![tag1.id]),
        search: None,
    };
    let filtered = videos::list_videos(&conn, Some(filter), None).unwrap();
    assert_eq!(filtered.len(), 1);

    // 5. Remove one tag
    videos::remove_tag_from_video(&conn, video.id, tag2.id).unwrap();

    // 6. Delete video
    videos::delete_video(&conn, video.id).unwrap();
    let all_videos = videos::list_videos(&conn, None, None).unwrap();
    assert_eq!(all_videos.len(), 0);

    // 7. Channel should still exist
    let channel = videos::get_channel_details(&conn, video.channel_id);
    assert!(channel.is_ok());
}

#[test]
fn test_multiple_videos_same_channel_workflow() {
    let mut conn = setup_test_db();

    // Add multiple videos to same channel
    let metadata1 = create_test_metadata("abc123", "Video 1", "SharedChannel");
    let metadata2 = create_test_metadata("def456", "Video 2", "SharedChannel");
    let metadata3 = create_test_metadata("ghi789", "Video 3", "SharedChannel");

    let video1 = videos::add_video(&mut conn, metadata1, None).unwrap();
    let video2 = videos::add_video(&mut conn, metadata2, None).unwrap();
    let video3 = videos::add_video(&mut conn, metadata3, None).unwrap();

    // Verify same channel
    assert_eq!(video1.channel_id, video2.channel_id);
    assert_eq!(video2.channel_id, video3.channel_id);

    // Filter by channel
    let filter = VideoFilter {
        channel_id: Some(video1.channel_id),
        status: None,
        tag_ids: None,
        search: None,
    };
    let channel_videos = videos::list_videos(&conn, Some(filter), None).unwrap();
    assert_eq!(channel_videos.len(), 3);
}

// =====================
// ADD VIDEO WITH TAGS TESTS
// =====================

#[test]
fn test_add_video_with_tags_none() {
    let mut conn = setup_test_db();

    let metadata = create_test_metadata("abc123", "Test Video", "TestChannel");
    let result = videos::add_video(&mut conn, metadata, None);

    assert!(result.is_ok());
    let video = result.unwrap();

    // Verify video was created
    assert_eq!(video.title, "Test Video");

    // Verify no tags were added
    let videos_with_tags = videos::list_videos(&conn, None, None).unwrap();
    assert_eq!(videos_with_tags.len(), 1);
    assert_eq!(videos_with_tags[0].tags.len(), 0);
}

#[test]
fn test_add_video_with_single_existing_tag() {
    use watchy_lib::{db::dto::CreateTag, services::tags};

    let mut conn = setup_test_db();

    // Create a tag first
    let tag = tags::add_tag(
        &conn,
        CreateTag {
            name: "Tutorial".to_string(),
            color: "#FF0000".to_string(),
        },
    )
    .unwrap();

    let metadata = create_test_metadata("abc123", "Test Video", "TestChannel");
    let result = videos::add_video(&mut conn, metadata, Some(vec![tag.clone()]));

    assert!(result.is_ok());
    let video = result.unwrap();

    // Verify video was created
    assert_eq!(video.title, "Test Video");

    // Verify tag was attached
    let videos_with_tags = videos::list_videos(&conn, None, None).unwrap();
    assert_eq!(videos_with_tags.len(), 1);
    assert_eq!(videos_with_tags[0].tags.len(), 1);
    assert_eq!(videos_with_tags[0].tags[0].id, tag.id);
    assert_eq!(videos_with_tags[0].tags[0].name, "Tutorial");
}

#[test]
fn test_add_video_with_multiple_existing_tags() {
    use watchy_lib::{db::dto::CreateTag, services::tags};

    let mut conn = setup_test_db();

    // Create multiple tags first
    let tag1 = tags::add_tag(
        &conn,
        CreateTag {
            name: "Tutorial".to_string(),
            color: "#FF0000".to_string(),
        },
    )
    .unwrap();

    let tag2 = tags::add_tag(
        &conn,
        CreateTag {
            name: "Beginner".to_string(),
            color: "#00FF00".to_string(),
        },
    )
    .unwrap();

    let tag3 = tags::add_tag(
        &conn,
        CreateTag {
            name: "Rust".to_string(),
            color: "#0000FF".to_string(),
        },
    )
    .unwrap();

    let metadata = create_test_metadata("abc123", "Test Video", "TestChannel");
    let result = videos::add_video(
        &mut conn,
        metadata,
        Some(vec![tag1.clone(), tag2.clone(), tag3.clone()]),
    );

    assert!(result.is_ok());
    let video = result.unwrap();

    // Verify video was created
    assert_eq!(video.title, "Test Video");

    // Verify all tags were attached
    let videos_with_tags = videos::list_videos(&conn, None, None).unwrap();
    assert_eq!(videos_with_tags.len(), 1);
    assert_eq!(videos_with_tags[0].tags.len(), 3);

    // Verify tag IDs match
    let tag_ids: Vec<i64> = videos_with_tags[0].tags.iter().map(|t| t.id).collect();
    assert!(tag_ids.contains(&tag1.id));
    assert!(tag_ids.contains(&tag2.id));
    assert!(tag_ids.contains(&tag3.id));
}

#[test]
fn test_add_video_with_invalid_tag_id() {
    use watchy_lib::db::schema::Tag;

    let mut conn = setup_test_db();

    let metadata = create_test_metadata("abc123", "Test Video", "TestChannel");

    // Create a tag with invalid ID
    let invalid_tag = Tag {
        id: 0,
        name: "Invalid".to_string(),
        color: "#FF0000".to_string(),
        created_at: "2026-01-27".to_string(),
        updated_at: "2026-01-27".to_string(),
    };

    let result = videos::add_video(&mut conn, metadata, Some(vec![invalid_tag]));

    assert!(result.is_err());
    assert_eq!(result.unwrap_err(), "Invalid tag id");

    // Verify video was not created (transaction rolled back)
    let videos_list = videos::list_videos(&conn, None, None).unwrap();
    assert_eq!(videos_list.len(), 0);
}

#[test]
fn test_add_video_with_negative_tag_id() {
    use watchy_lib::db::schema::Tag;

    let mut conn = setup_test_db();

    let metadata = create_test_metadata("abc123", "Test Video", "TestChannel");

    // Create a tag with negative ID
    let invalid_tag = Tag {
        id: -5,
        name: "Invalid".to_string(),
        color: "#FF0000".to_string(),
        created_at: "2026-01-27".to_string(),
        updated_at: "2026-01-27".to_string(),
    };

    let result = videos::add_video(&mut conn, metadata, Some(vec![invalid_tag]));

    assert!(result.is_err());
    assert_eq!(result.unwrap_err(), "Invalid tag id");

    // Verify video was not created (transaction rolled back)
    let videos_list = videos::list_videos(&conn, None, None).unwrap();
    assert_eq!(videos_list.len(), 0);
}

#[test]
fn test_add_video_with_nonexistent_tag_id() {
    use watchy_lib::db::schema::Tag;

    let mut conn = setup_test_db();

    let metadata = create_test_metadata("abc123", "Test Video", "TestChannel");

    // Create a tag with a valid-looking ID that doesn't exist in database
    let nonexistent_tag = Tag {
        id: 99999,
        name: "Nonexistent".to_string(),
        color: "#FF0000".to_string(),
        created_at: "2026-01-27".to_string(),
        updated_at: "2026-01-27".to_string(),
    };

    let result = videos::add_video(&mut conn, metadata, Some(vec![nonexistent_tag]));

    // Should fail due to foreign key constraint
    assert!(result.is_err());

    // Verify video was not created (transaction rolled back)
    let videos_list = videos::list_videos(&conn, None, None).unwrap();
    assert_eq!(videos_list.len(), 0);
}

#[test]
fn test_add_video_with_tags_creates_channel() {
    use watchy_lib::{db::dto::CreateTag, services::tags};

    let mut conn = setup_test_db();

    // Create tags first
    let tag = tags::add_tag(
        &conn,
        CreateTag {
            name: "Tutorial".to_string(),
            color: "#FF0000".to_string(),
        },
    )
    .unwrap();

    let metadata = create_test_metadata("abc123", "Test Video", "NewChannel");
    let result = videos::add_video(&mut conn, metadata, Some(vec![tag.clone()]));

    assert!(result.is_ok());
    let video = result.unwrap();

    // Verify channel was created
    let channel = videos::get_channel_details(&conn, video.channel_id);
    assert!(channel.is_ok());
    assert_eq!(channel.unwrap().name, "NewChannel");

    // Verify tag was attached
    let videos_with_tags = videos::list_videos(&conn, None, None).unwrap();
    assert_eq!(videos_with_tags[0].tags.len(), 1);
}

#[test]
fn test_add_video_with_duplicate_link_and_tags_fails() {
    use watchy_lib::{db::dto::CreateTag, services::tags};

    let mut conn = setup_test_db();

    // Create tag first
    let tag = tags::add_tag(
        &conn,
        CreateTag {
            name: "Tutorial".to_string(),
            color: "#FF0000".to_string(),
        },
    )
    .unwrap();

    let metadata1 = create_test_metadata("abc123", "Video 1", "Channel1");
    let result1 = videos::add_video(&mut conn, metadata1, Some(vec![tag.clone()]));
    assert!(result1.is_ok());

    // Try to add video with same link
    let metadata2 = create_test_metadata("abc123", "Video 2", "Channel2");
    let result2 = videos::add_video(&mut conn, metadata2, Some(vec![tag.clone()]));
    assert!(result2.is_err());
    assert_eq!(result2.unwrap_err(), "Video already exists");
}

#[test]
fn test_add_multiple_videos_with_same_tags() {
    use watchy_lib::{db::dto::CreateTag, services::tags};

    let mut conn = setup_test_db();

    // Create shared tags
    let tag1 = tags::add_tag(
        &conn,
        CreateTag {
            name: "Tutorial".to_string(),
            color: "#FF0000".to_string(),
        },
    )
    .unwrap();

    let tag2 = tags::add_tag(
        &conn,
        CreateTag {
            name: "Beginner".to_string(),
            color: "#00FF00".to_string(),
        },
    )
    .unwrap();

    // Add first video with tags
    let metadata1 = create_test_metadata("abc123", "Video 1", "Channel1");
    videos::add_video(&mut conn, metadata1, Some(vec![tag1.clone(), tag2.clone()])).unwrap();

    // Add second video with same tags
    let metadata2 = create_test_metadata("xyz789", "Video 2", "Channel1");
    videos::add_video(&mut conn, metadata2, Some(vec![tag1.clone(), tag2.clone()])).unwrap();

    // Both videos should have the same tags
    let videos_with_tags = videos::list_videos(&conn, None, None).unwrap();
    assert_eq!(videos_with_tags.len(), 2);

    for video_item in videos_with_tags {
        assert_eq!(video_item.tags.len(), 2);
        let tag_ids: Vec<i64> = video_item.tags.iter().map(|t| t.id).collect();
        assert!(tag_ids.contains(&tag1.id));
        assert!(tag_ids.contains(&tag2.id));
    }
}

// =====================
// GET VIDEO TESTS
// =====================

#[test]
fn test_get_video_with_tags() {
    use watchy_lib::{db::dto::CreateTag, services::tags};

    let mut conn = setup_test_db();

    // Create tags
    let tag1 = tags::add_tag(
        &conn,
        CreateTag {
            name: "Tutorial".to_string(),
            color: "#FF0000".to_string(),
        },
    )
    .unwrap();

    let tag2 = tags::add_tag(
        &conn,
        CreateTag {
            name: "Review".to_string(),
            color: "#00FF00".to_string(),
        },
    )
    .unwrap();

    // Add video with tags
    let metadata = create_test_metadata("abc123", "Test Video", "TestChannel");
    let video =
        videos::add_video(&mut conn, metadata, Some(vec![tag1.clone(), tag2.clone()])).unwrap();

    // Get video by ID
    let result = videos::get_video(&conn, video.id);
    assert!(result.is_ok());

    let video_item = result.unwrap();
    assert_eq!(video_item.video.id, video.id);
    assert_eq!(video_item.video.title, "Test Video");
    assert_eq!(video_item.video.channel_name, "TestChannel");
    assert_eq!(
        video_item.video.channel_link,
        "https://youtube.com/@TestChannel"
    );
    assert_eq!(video_item.tags.len(), 2);

    let tag_ids: Vec<i64> = video_item.tags.iter().map(|t| t.id).collect();
    assert!(tag_ids.contains(&tag1.id));
    assert!(tag_ids.contains(&tag2.id));
}

#[test]
fn test_get_video_without_tags() {
    let mut conn = setup_test_db();

    // Add video without tags
    let metadata = create_test_metadata("abc123", "Test Video", "TestChannel");
    let video = videos::add_video(&mut conn, metadata, None).unwrap();

    // Get video by ID
    let result = videos::get_video(&conn, video.id);
    assert!(result.is_ok());

    let video_item = result.unwrap();
    assert_eq!(video_item.video.id, video.id);
    assert_eq!(video_item.video.title, "Test Video");
    assert_eq!(video_item.tags.len(), 0);
}

#[test]
fn test_get_video_not_found() {
    let conn = setup_test_db();

    // Try to get non-existent video
    let result = videos::get_video(&conn, 999);
    assert!(result.is_err());
}
