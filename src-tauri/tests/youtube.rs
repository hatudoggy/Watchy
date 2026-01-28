use watchy_lib::commands::youtube;

const MOCK_HTML: &str = r#"
<script>
var ytInitialPlayerResponse = {
  "videoDetails": {
    "videoId": "dQw4w9WgXcQ",
    "title": "Test Video",
    "viewCount": "1234",
    "lengthSeconds": "1072",
    "thumbnail": {
      "thumbnails": [
        { "url": "a" },
        { "url": "b" },
        { "url": "c" },
        { "url": "https://img.test/thumb.jpg" }
      ]
    }
  },
  "microformat": {
    "playerMicroformatRenderer": {
      "uploadDate": "2026-01-10",
      "likeCount": "456"
    }
  }
};
</script>

<script>
var ytInitialData = {
  "contents": {
    "twoColumnWatchNextResults": {
      "results": {
        "results": {
          "contents": [
            {},
            {
              "videoSecondaryInfoRenderer": {
                "owner": {
                  "videoOwnerRenderer": {
                    "title": {
                      "runs": [
                        {
                          "text": "TestChannel",
                          "navigationEndpoint": {
                            "commandMetadata": {
                              "webCommandMetadata": {
                                "url": "/@TestChannel"
                              }
                            }
                          }
                        }
                      ]
                    },
                    "thumbnail": {
                      "thumbnails": [
                        { "url": "x" },
                        { "url": "https://img.test/avatar.jpg" }
                      ]
                    },
                    "subscriberCountText": {
                      "simpleText": "1.23M subscribers"
                    },
                    "badges": [{}]
                  }
                }
              }
            }
          ]
        }
      }
    }
  }
};
</script>
"#;

#[test]
fn extracts_player_data() {
    let video = youtube::parse_youtube_player_data(MOCK_HTML).unwrap();

    assert_eq!(video.link, "https://www.youtube.com/watch?v=dQw4w9WgXcQ");
    assert_eq!(video.title, "Test Video");
    assert_eq!(video.thumbnail, "https://img.test/thumb.jpg");
    assert_eq!(video.upload_date, "2026-01-10");
    assert_eq!(video.duration, Some(1072));
    assert_eq!(video.views, Some(1234));
    assert_eq!(video.likes, Some(456));
}

#[test]
fn extracts_channel_data() {
    let channel = youtube::parse_youtube_initial_data(MOCK_HTML).unwrap();

    assert_eq!(channel.link, "https://www.youtube.com/@TestChannel");
    assert_eq!(channel.name, "TestChannel");
    assert_eq!(channel.avatar, "https://img.test/avatar.jpg");
    assert_eq!(channel.subscribers, Some(1230000));
    assert!(channel.verified);
}

#[test]
fn valid_youtube_url() {
    let url = "https://www.youtube.com/watch?v=dQw4w9WgXcQ";
    assert!(youtube::validate_youtube_url(url).is_ok());
}

#[test]
fn rejects_non_video() {
    let url = "https://www.youtube.com/@somechannel";
    assert!(youtube::validate_youtube_url(url).is_err());
}

#[test]
fn rejects_non_youtube() {
    let url = "https://example.com/watch?v=123";
    assert!(youtube::validate_youtube_url(url).is_err());
}

// =====================
// PARSE CHANNEL SUBTITLE TESTS
// =====================

#[test]
fn test_parse_channel_subtitle_with_unicode_formatting() {
    let input = "\u{200E}\u{2068}@syntaxfm\u{2069} • \u{2068}457K subscribers\u{2069}";

    let result = youtube::parse_channel_subtitle(input);

    assert!(result.is_some());
    let subtitle = result.unwrap();
    assert_eq!(subtitle.username, "@syntaxfm");
    assert_eq!(subtitle.subscribers_text, "457K");
}

#[test]
fn test_parse_channel_subtitle_without_unicode() {
    let input = "@testchannel • 1.2M subscribers";

    let result = youtube::parse_channel_subtitle(input);

    assert!(result.is_some());
    let subtitle = result.unwrap();
    assert_eq!(subtitle.username, "@testchannel");
    assert_eq!(subtitle.subscribers_text, "1.2M");
}

#[test]
fn test_parse_channel_subtitle_with_comma() {
    let input = "@channel123 • 1,234 subscribers";

    let result = youtube::parse_channel_subtitle(input);

    assert!(result.is_some());
    let subtitle = result.unwrap();
    assert_eq!(subtitle.username, "@channel123");
    assert_eq!(subtitle.subscribers_text, "1,234");
}

#[test]
fn test_parse_channel_subtitle_without_bullet() {
    // Test the new regex pattern that doesn't require bullet point
    let input = "@channel_name 500K subscribers";

    let result = youtube::parse_channel_subtitle(input);

    assert!(result.is_some());
    let subtitle = result.unwrap();
    assert_eq!(subtitle.username, "@channel_name");
    assert_eq!(subtitle.subscribers_text, "500K");
}

#[test]
fn test_parse_channel_subtitle_sanitizes_username() {
    // Test that username is sanitized to only valid characters
    let input = "@user_name.123🔥invalid 100K subscribers";

    let result = youtube::parse_channel_subtitle(input);

    assert!(result.is_some());
    let subtitle = result.unwrap();
    // Should stop at the emoji and only include valid chars
    assert_eq!(subtitle.username, "@user_name.123");
    assert_eq!(subtitle.subscribers_text, "100K");
}

#[test]
fn test_parse_channel_subtitle_with_special_separator() {
    // Test with various separators between username and subscriber count
    let input = "@handle - 2.5M subscribers";

    let result = youtube::parse_channel_subtitle(input);

    assert!(result.is_some());
    let subtitle = result.unwrap();
    assert_eq!(subtitle.username, "@handle");
    assert_eq!(subtitle.subscribers_text, "2.5M");
}

#[test]
fn test_parse_channel_subtitle_invalid_format() {
    let input = "invalid format without @ symbol";

    let result = youtube::parse_channel_subtitle(input);

    assert!(result.is_none());
}

#[test]
fn test_parse_channel_subtitle_no_subscribers_keyword() {
    // Should fail if "subscribers" keyword is missing
    let input = "@channel • 1.2M";

    let result = youtube::parse_channel_subtitle(input);

    assert!(result.is_none());
}

// =====================
// PARSE SUBSCRIBER COUNT TESTS
// =====================

#[test]
fn test_parse_subscriber_count_million() {
    let result = youtube::parse_subscriber_count("1.5M subscribers");
    assert_eq!(result, Some(1_500_000));
}

#[test]
fn test_parse_subscriber_count_thousand() {
    let result = youtube::parse_subscriber_count("457K subscribers");
    assert_eq!(result, Some(457_000));
}

#[test]
fn test_parse_subscriber_count_plain() {
    let result = youtube::parse_subscriber_count("1,234 subscribers");
    assert_eq!(result, Some(1234));
}
