use rusqlite::Connection;
use watchy_lib::{
    db::{
        dto::{CreateTag, UpdateTag},
        schema_init::init_schema,
    },
    services::tags,
};

fn setup_test_db() -> Connection {
    let conn = Connection::open_in_memory().expect("Failed to create in-memory database");
    init_schema(&conn).expect("Failed to initialize schema");
    conn
}

// =====================
// LIST TAGS TESTS
// =====================

#[test]
fn test_list_tags_single() {
    let conn = setup_test_db();

    let new_tag = CreateTag {
        name: "Tutorial".to_string(),
        color: "#FF0000".to_string(),
    };

    tags::add_tag(&conn, new_tag).unwrap();

    let result = tags::list_tags(&conn);
    assert!(result.is_ok());

    let tag_list = result.unwrap();
    assert_eq!(tag_list.len(), 1);
    assert_eq!(tag_list[0].name, "Tutorial");
    assert_eq!(tag_list[0].color, "#FF0000");
}

#[test]
fn test_list_tags_multiple() {
    let conn = setup_test_db();

    let tag1 = CreateTag {
        name: "Tutorial".to_string(),
        color: "#FF0000".to_string(),
    };
    let tag2 = CreateTag {
        name: "Review".to_string(),
        color: "#00FF00".to_string(),
    };
    let tag3 = CreateTag {
        name: "Live".to_string(),
        color: "#0000FF".to_string(),
    };

    tags::add_tag(&conn, tag1).unwrap();
    std::thread::sleep(std::time::Duration::from_millis(10));
    tags::add_tag(&conn, tag2).unwrap();
    std::thread::sleep(std::time::Duration::from_millis(10));
    tags::add_tag(&conn, tag3).unwrap();

    let result = tags::list_tags(&conn);
    assert!(result.is_ok());

    let tag_list = result.unwrap();
    assert_eq!(tag_list.len(), 3);

    // Should be ordered by created_at ASC
    assert_eq!(tag_list[0].name, "Tutorial");
    assert_eq!(tag_list[1].name, "Review");
    assert_eq!(tag_list[2].name, "Live");
}

// =====================
// ADD TAG TESTS
// =====================

#[test]
fn test_add_tag_success() {
    let conn = setup_test_db();

    let new_tag = CreateTag {
        name: "Programming".to_string(),
        color: "#FF5733".to_string(),
    };

    let result = tags::add_tag(&conn, new_tag);
    assert!(result.is_ok());

    let tag = result.unwrap();
    assert_eq!(tag.name, "Programming");
    assert_eq!(tag.color, "#FF5733");
    assert!(tag.id > 0);
    assert!(!tag.created_at.is_empty());
    assert!(!tag.updated_at.is_empty());
}

#[test]
fn test_add_tag_duplicate_name_fails() {
    let conn = setup_test_db();

    let tag1 = CreateTag {
        name: "Tutorial".to_string(),
        color: "#FF0000".to_string(),
    };
    let tag2 = CreateTag {
        name: "Tutorial".to_string(),
        color: "#00FF00".to_string(),
    };

    let result1 = tags::add_tag(&conn, tag1);
    assert!(result1.is_ok());

    let result2 = tags::add_tag(&conn, tag2);
    assert!(result2.is_err());
}

#[test]
fn test_add_tag_with_hex_color() {
    let conn = setup_test_db();

    let new_tag = CreateTag {
        name: "Important".to_string(),
        color: "#ABCDEF".to_string(),
    };

    let tag = tags::add_tag(&conn, new_tag).unwrap();
    assert_eq!(tag.color, "#ABCDEF");
}

// =====================
// EDIT TAG TESTS
// =====================

#[test]
fn test_edit_tag_success() {
    let conn = setup_test_db();

    let new_tag = CreateTag {
        name: "Original".to_string(),
        color: "#FF0000".to_string(),
    };

    let tag = tags::add_tag(&conn, new_tag).unwrap();

    std::thread::sleep(std::time::Duration::from_millis(1100));

    let update = UpdateTag {
        name: "Updated".to_string(),
        color: "#00FF00".to_string(),
    };

    let result = tags::edit_tag(&conn, tag.id, update);
    assert!(result.is_ok());

    let updated = result.unwrap();
    assert_eq!(updated.name, "Updated");
    assert_eq!(updated.color, "#00FF00");
    assert_eq!(updated.id, tag.id);
}

#[test]
fn test_edit_tag_updates_timestamp() {
    let conn = setup_test_db();

    let new_tag = CreateTag {
        name: "Test".to_string(),
        color: "#FF0000".to_string(),
    };

    let tag = tags::add_tag(&conn, new_tag).unwrap();
    let original_updated_at = tag.updated_at.clone();

    std::thread::sleep(std::time::Duration::from_millis(1100));

    let update = UpdateTag {
        name: "Modified".to_string(),
        color: tag.color.clone(),
    };

    let updated = tags::edit_tag(&conn, tag.id, update).unwrap();
    assert_ne!(updated.updated_at, original_updated_at);
    assert_eq!(updated.created_at, tag.created_at);
}

#[test]
fn test_edit_tag_not_found() {
    let conn = setup_test_db();

    let update = UpdateTag {
        name: "NonExistent".to_string(),
        color: "#FF0000".to_string(),
    };

    // The UPDATE succeeds with 0 rows, but the subsequent SELECT fails
    let result = tags::edit_tag(&conn, 99999, update);
    assert!(result.is_err());
}

#[test]
fn test_edit_tag_duplicate_name_fails() {
    let conn = setup_test_db();

    let tag1 = CreateTag {
        name: "Tag1".to_string(),
        color: "#FF0000".to_string(),
    };
    let tag2 = CreateTag {
        name: "Tag2".to_string(),
        color: "#00FF00".to_string(),
    };

    tags::add_tag(&conn, tag1).unwrap();
    let tag2_created = tags::add_tag(&conn, tag2).unwrap();

    // Try to rename tag2 to tag1
    let update = UpdateTag {
        name: "Tag1".to_string(),
        color: "#0000FF".to_string(),
    };

    let result = tags::edit_tag(&conn, tag2_created.id, update);
    assert!(result.is_err());
}

#[test]
fn test_edit_tag_only_color() {
    let conn = setup_test_db();

    let new_tag = CreateTag {
        name: "KeepName".to_string(),
        color: "#FF0000".to_string(),
    };

    let tag = tags::add_tag(&conn, new_tag).unwrap();

    let update = UpdateTag {
        name: tag.name.clone(),
        color: "#00FF00".to_string(),
    };

    let updated = tags::edit_tag(&conn, tag.id, update).unwrap();
    assert_eq!(updated.name, "KeepName");
    assert_eq!(updated.color, "#00FF00");
}

#[test]
fn test_edit_tag_only_name() {
    let conn = setup_test_db();

    let new_tag = CreateTag {
        name: "OldName".to_string(),
        color: "#FF0000".to_string(),
    };

    let tag = tags::add_tag(&conn, new_tag).unwrap();

    let update = UpdateTag {
        name: "NewName".to_string(),
        color: tag.color.clone(),
    };

    let updated = tags::edit_tag(&conn, tag.id, update).unwrap();
    assert_eq!(updated.name, "NewName");
    assert_eq!(updated.color, "#FF0000");
}

// =====================
// DELETE TAG TESTS
// =====================

#[test]
fn test_delete_tag_success() {
    let conn = setup_test_db();

    let new_tag = CreateTag {
        name: "ToDelete".to_string(),
        color: "#FF0000".to_string(),
    };

    let tag = tags::add_tag(&conn, new_tag).unwrap();

    let result = tags::delete_tag(&conn, tag.id);
    assert!(result.is_ok());

    // Verify tag is gone
    let all_tags = tags::list_tags(&conn).unwrap();
    assert_eq!(all_tags.len(), 0);
}

#[test]
fn test_delete_tag_not_found() {
    let conn = setup_test_db();

    let result = tags::delete_tag(&conn, 99999);
    assert!(result.is_err());
    assert!(result.unwrap_err().contains("not found"));
}

#[test]
fn test_delete_tag_removes_from_list() {
    let conn = setup_test_db();

    let tag1 = CreateTag {
        name: "Tag1".to_string(),
        color: "#FF0000".to_string(),
    };
    let tag2 = CreateTag {
        name: "Tag2".to_string(),
        color: "#00FF00".to_string(),
    };
    let tag3 = CreateTag {
        name: "Tag3".to_string(),
        color: "#0000FF".to_string(),
    };

    tags::add_tag(&conn, tag1).unwrap();
    let tag2_created = tags::add_tag(&conn, tag2).unwrap();
    tags::add_tag(&conn, tag3).unwrap();

    tags::delete_tag(&conn, tag2_created.id).unwrap();

    let remaining = tags::list_tags(&conn).unwrap();
    assert_eq!(remaining.len(), 2);

    let names: Vec<&str> = remaining.iter().map(|t| t.name.as_str()).collect();
    assert!(names.contains(&"Tag1"));
    assert!(!names.contains(&"Tag2"));
    assert!(names.contains(&"Tag3"));
}

// =====================
// INTEGRATION TESTS
// =====================

#[test]
fn test_complete_tag_workflow() {
    let conn = setup_test_db();

    // 1. Create several tags
    let tutorial = CreateTag {
        name: "Tutorial".to_string(),
        color: "#FF0000".to_string(),
    };
    let review = CreateTag {
        name: "Review".to_string(),
        color: "#00FF00".to_string(),
    };
    let live = CreateTag {
        name: "Live".to_string(),
        color: "#0000FF".to_string(),
    };

    let tutorial_tag = tags::add_tag(&conn, tutorial).unwrap();
    let _review_tag = tags::add_tag(&conn, review).unwrap();
    let live_tag = tags::add_tag(&conn, live).unwrap();

    // 2. List all tags
    let all_tags = tags::list_tags(&conn).unwrap();
    assert_eq!(all_tags.len(), 3);

    // 3. Update one tag
    std::thread::sleep(std::time::Duration::from_millis(1100));
    let update = UpdateTag {
        name: "Tutorial - Updated".to_string(),
        color: "#FF5733".to_string(),
    };
    let updated_tutorial = tags::edit_tag(&conn, tutorial_tag.id, update).unwrap();
    assert_eq!(updated_tutorial.name, "Tutorial - Updated");
    assert_ne!(updated_tutorial.updated_at, tutorial_tag.updated_at);

    // 4. Delete one tag
    tags::delete_tag(&conn, live_tag.id).unwrap();

    // 5. Verify final state
    let final_tags = tags::list_tags(&conn).unwrap();
    assert_eq!(final_tags.len(), 2);

    let names: Vec<&str> = final_tags.iter().map(|t| t.name.as_str()).collect();
    assert!(names.contains(&"Tutorial - Updated"));
    assert!(names.contains(&"Review"));
    assert!(!names.contains(&"Live"));
}

#[test]
fn test_tag_color_variations() {
    let conn = setup_test_db();

    let colors = vec![
        "#000000", // Black
        "#FFFFFF", // White
        "#FF0000", // Red
        "#00FF00", // Green
        "#0000FF", // Blue
        "#FFFF00", // Yellow
        "#FF00FF", // Magenta
        "#00FFFF", // Cyan
    ];

    for (i, color) in colors.iter().enumerate() {
        let tag = CreateTag {
            name: format!("Color{}", i),
            color: color.to_string(),
        };
        let created = tags::add_tag(&conn, tag).unwrap();
        assert_eq!(created.color, *color);
    }

    let all_tags = tags::list_tags(&conn).unwrap();
    assert_eq!(all_tags.len(), colors.len());
}

#[test]
fn test_tag_name_variations() {
    let conn = setup_test_db();

    let names = vec![
        "Simple",
        "With Space",
        "With-Dash",
        "With_Underscore",
        "With.Dot",
        "With123Numbers",
        "UPPERCASE",
        "lowercase",
        "MixedCase",
    ];

    for name in &names {
        let tag = CreateTag {
            name: name.to_string(),
            color: "#000000".to_string(),
        };
        let created = tags::add_tag(&conn, tag).unwrap();
        assert_eq!(created.name, *name);
    }

    let all_tags = tags::list_tags(&conn).unwrap();
    assert_eq!(all_tags.len(), names.len());
}

#[test]
fn test_tag_operations_isolation() {
    let conn = setup_test_db();

    // Create multiple tags
    for i in 0..5 {
        let tag = CreateTag {
            name: format!("Tag{}", i),
            color: "#000000".to_string(),
        };
        tags::add_tag(&conn, tag).unwrap();
    }

    let tags_list = tags::list_tags(&conn).unwrap();
    let middle_tag = &tags_list[2];

    // Update middle tag shouldn't affect others
    let update = UpdateTag {
        name: "UpdatedMiddle".to_string(),
        color: "#FF0000".to_string(),
    };
    tags::edit_tag(&conn, middle_tag.id, update).unwrap();

    let updated_list = tags::list_tags(&conn).unwrap();
    assert_eq!(updated_list.len(), 5);
    assert_eq!(updated_list[2].name, "UpdatedMiddle");
    assert_eq!(updated_list[0].name, "Tag0");
    assert_eq!(updated_list[4].name, "Tag4");
}
