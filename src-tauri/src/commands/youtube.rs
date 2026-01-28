use regex::Regex;
use reqwest::header::{HeaderMap, HeaderValue, ACCEPT, ACCEPT_LANGUAGE, USER_AGENT};
use serde::{Deserialize, Serialize};
use tauri::Url;

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct YoutubeMetadata {
    pub video: YoutubeVideoMetadata,
    pub channel: YoutubeChannelMetadata,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct YoutubeVideoMetadata {
    pub link: String,
    pub title: String,
    pub thumbnail: String,
    pub upload_date: String,

    pub duration: Option<u64>,
    pub views: Option<u64>,
    pub likes: Option<u64>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct YoutubeChannelMetadata {
    pub link: String,
    pub name: String,
    pub avatar: String,
    pub subscribers: Option<u64>,
    pub verified: bool,
}

#[tauri::command]
pub async fn fetch_youtube_metadata(url: &str) -> Result<YoutubeMetadata, String> {
    let _ = validate_youtube_url(url)?;

    // Run a get request to the youtube url to extract the JSON-LD of the html which will contain data like title, views, channel, etc.
    let mut headers = HeaderMap::new();
    // Theses headers are used so that youtube doesn't block the request
    headers.insert(
        USER_AGENT,
        HeaderValue::from_static(
            "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 \
        (KHTML, like Gecko) Chrome/120.0.0.0 Safari/537.36",
        ),
    );
    headers.insert(
        ACCEPT,
        HeaderValue::from_static(
            "text/html,application/xhtml+xml,application/xml;q=0.9,image/webp,*/*;q=0.8",
        ),
    );
    headers.insert(ACCEPT_LANGUAGE, HeaderValue::from_static("en-US,en;q=0.9"));

    let client = reqwest::Client::new();

    let html = client
        .get(url)
        .headers(headers)
        .send()
        .await
        .map_err(|_| "Network error".to_string())?
        .text()
        .await
        .map_err(|_| "Failed to read response".to_string())?;

    let video = parse_youtube_player_data(&html)?;
    let channel = parse_youtube_initial_data(&html)?;

    Ok(YoutubeMetadata { video, channel })
}

pub fn validate_youtube_url(url: &str) -> Result<url::Url, String> {
    // Check if the url string is a valid url, if not, return an error message
    let parsed = Url::parse(url).map_err(|_| "Invalid URL".to_string())?;

    let host = parsed.host_str().unwrap_or("");

    if !host.contains("youtube.com") && !host.contains("youtu.be") {
        return Err("Not a Youtube URL".to_string());
    }

    // Check if the url is a valid youtube video, if not, return an error message
    let is_video = parsed.query_pairs().any(|(k, _)| k == "v") || host.contains("youtu.be");
    if !is_video {
        return Err("Not a Youtube video URL".to_string());
    }

    Ok(parsed)
}

pub fn parse_youtube_player_data(html: &str) -> Result<YoutubeVideoMetadata, String> {
    let raw = extract_js_object(html, "ytInitialPlayerResponse")?;

    let json: serde_json::Value =
        serde_json::from_str(&raw).map_err(|_| "Invalid ytInitialPlayerResponse")?;

    let video_details = &json["videoDetails"];
    let microformat = &json["microformat"]["playerMicroformatRenderer"];

    let link = format!(
        "https://www.youtube.com/watch?v={}",
        video_details["videoId"].as_str().unwrap_or("")
    );
    let title = video_details["title"].as_str().unwrap_or("").to_string();
    let thumbnail = video_details["thumbnail"]["thumbnails"][3]["url"]
        .as_str()
        .unwrap_or("")
        .to_string();
    let upload_date = microformat["uploadDate"].as_str().unwrap_or("").to_string();
    let duration = video_details["lengthSeconds"]
        .as_str()
        .unwrap_or("")
        .to_string()
        .parse::<u64>()
        .ok();
    let views = video_details["viewCount"]
        .as_str()
        .unwrap_or("")
        .to_string()
        .parse::<u64>()
        .ok();
    let likes = microformat["likeCount"]
        .as_str()
        .unwrap_or("")
        .to_string()
        .parse::<u64>()
        .ok();

    if title.is_empty() {
        return Err("Failed to parse Youtube player metadata".to_string());
    }

    Ok(YoutubeVideoMetadata {
        link,
        title,
        thumbnail,
        upload_date,
        duration,
        views,
        likes,
    })
}

pub fn parse_youtube_initial_data(html: &str) -> Result<YoutubeChannelMetadata, String> {
    let raw = extract_js_object(html, "ytInitialData")?;

    let json: serde_json::Value =
        serde_json::from_str(&raw).map_err(|_| "Invalid ytInitialData")?;

    let owner = &json["contents"]["twoColumnWatchNextResults"]["results"]["results"]["contents"][1]
        ["videoSecondaryInfoRenderer"]["owner"]["videoOwnerRenderer"];

    // Check if the video is a collab or not (Video with multiple channels)
    let is_collab = owner
        .as_object()
        .map_or(false, |obj| !obj.contains_key("thumbnail"));

    if is_collab {
        let main_owner = &owner["navigationEndpoint"]["showDialogCommand"]["panelLoadingStrategy"]
            ["inlineContent"]["dialogViewModel"]["customContent"]["listViewModel"]["listItems"][0]
            ["listItemViewModel"];

        let link = format!(
            "https://www.youtube.com{}",
            main_owner["subtitle"]["content"]
                .as_str()
                .and_then(parse_channel_subtitle)
                .ok_or("Channel link not found")?
                .username
        );
        let name = main_owner["title"]["content"]
            .as_str()
            .unwrap_or("")
            .to_string();

        let avatar = main_owner["leadingAccessory"]["avatarViewModel"]["image"]["sources"]
            .as_array()
            .and_then(|a| a.last())
            .and_then(|v| v["url"].as_str())
            .unwrap_or("")
            .to_string();

        let subscribers = main_owner["subscriberCountText"]["simpleText"]
            .as_str()
            .and_then(parse_channel_subtitle)
            .and_then(|sub| parse_subscriber_count(&sub.subscribers_text));

        let verified: bool = main_owner.get("attachmentRuns").is_some();

        if name.is_empty() {
            return Err("Failed to parse Youtube initial metadata - Collab".to_string());
        }

        return Ok(YoutubeChannelMetadata {
            link,
            name,
            avatar,
            subscribers,
            verified,
        });
    }

    // Not collab
    let link = format!(
        "https://www.youtube.com{}",
        owner["title"]["runs"][0]["navigationEndpoint"]["commandMetadata"]["webCommandMetadata"]
            ["url"]
            .as_str()
            .unwrap_or("")
    );
    let name = owner["title"]["runs"][0]["text"]
        .as_str()
        .unwrap_or("")
        .to_string();

    let avatar = owner["thumbnail"]["thumbnails"]
        .as_array()
        .and_then(|a| a.last())
        .and_then(|v| v["url"].as_str())
        .unwrap_or("")
        .to_string();

    let subscribers = owner["subscriberCountText"]["simpleText"]
        .as_str()
        .and_then(parse_subscriber_count);

    let verified: bool = owner.get("badges").is_some();

    if name.is_empty() {
        return Err("Failed to parse Youtube initial metadata".to_string());
    }

    Ok(YoutubeChannelMetadata {
        link,
        name,
        avatar,
        subscribers,
        verified,
    })
}

#[derive(Debug)]
pub struct ChannelSubtitle {
    pub username: String,
    pub subscribers_text: String,
}

pub fn parse_channel_subtitle(input: &str) -> Option<ChannelSubtitle> {
    let clean: String = input
        .chars()
        .filter(|c| {
            !matches!(
                *c,
                '\u{200E}'
                    | '\u{200F}'
                    | '\u{202A}'
                    | '\u{202B}'
                    | '\u{202C}'
                    | '\u{202D}'
                    | '\u{202E}'
                    | '\u{2066}'
                    | '\u{2067}'
                    | '\u{2068}'
                    | '\u{2069}'
            )
        })
        .collect();

    let re = Regex::new(r"(@\S+)[^\d@]+([\d.,]+[KM]?)\s+subscribers").ok()?;

    let caps = re.captures(&clean)?;

    let raw_username = caps.get(1)?.as_str();

    let username: String = raw_username
        .chars()
        .take_while(|c| c.is_ascii_alphanumeric() || *c == '@' || *c == '_' || *c == '.')
        .collect();

    Some(ChannelSubtitle {
        username,
        subscribers_text: caps.get(2)?.as_str().to_string(),
    })
}

pub fn parse_subscriber_count(s: &str) -> Option<u64> {
    let s = s
        .replace("subscribers", "")
        .replace("subscriber", "")
        .trim()
        .replace(",", "");

    if s.ends_with('M') {
        let n = s.trim_end_matches('M').parse::<f64>().ok()?;
        Some((n * 1_000_000.0).round() as u64)
    } else if s.ends_with('K') {
        let n = s.trim_end_matches('K').parse::<f64>().ok()?;
        Some((n * 1_000.0).round() as u64)
    } else {
        s.parse::<u64>().ok()
    }
}

fn extract_js_object(html: &str, var_name: &str) -> Result<String, String> {
    let marker = format!("var {} = ", var_name);
    let start = html.find(&marker).ok_or(format!("{var_name} not found"))?;
    let mut i = start + marker.len();

    let bytes = html.as_bytes();
    let mut depth = 0;
    let mut in_string = false;

    let mut result = String::new();

    while i < bytes.len() {
        let c = bytes[i] as char;
        result.push(c);

        match c {
            '"' if bytes[i - 1] != b'\\' => in_string = !in_string,
            '{' if !in_string => depth += 1,
            '}' if !in_string => {
                depth -= 1;
                if depth == 0 {
                    break;
                }
            }
            _ => {}
        }

        i += 1;
    }

    Ok(result)
}
