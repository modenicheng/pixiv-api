use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use super::common::Tag;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Novel {
    pub id: u64,
    pub title: String,
    #[serde(default)]
    pub caption: Option<String>,
    #[serde(default)]
    pub restrict: Option<i32>,
    #[serde(default)]
    pub x_restrict: Option<i32>,
    #[serde(default)]
    pub is_original: Option<bool>,
    #[serde(default)]
    pub image_urls: Option<super::common::ImageUrls>,
    #[serde(default)]
    pub create_date: Option<DateTime<Utc>>,
    #[serde(default)]
    pub tags: Option<Vec<Tag>>,
    #[serde(default)]
    pub page_count: Option<u32>,
    #[serde(default)]
    pub text_length: Option<u64>,
    #[serde(default)]
    pub user: Option<super::user::UserPreview>,
    #[serde(default)]
    pub series: Option<NovelSeriesInfo>,
    #[serde(default)]
    pub is_bookmarked: Option<bool>,
    #[serde(default)]
    pub total_bookmarks: Option<u64>,
    #[serde(default)]
    pub total_view: Option<u64>,
    #[serde(default)]
    pub total_comments: Option<u64>,
    #[serde(default)]
    pub is_muted: Option<bool>,
    #[serde(default)]
    pub visible: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NovelSeriesInfo {
    pub id: u64,
    pub title: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NovelSeries {
    pub id: u64,
    #[serde(default)]
    pub title: Option<String>,
    #[serde(default)]
    pub caption: Option<String>,
    #[serde(default)]
    pub is_original: Option<bool>,
    #[serde(default)]
    pub is_concluded: Option<bool>,
    #[serde(default)]
    pub content_count: Option<u64>,
    #[serde(default)]
    pub total_character_count: Option<u64>,
    #[serde(default)]
    pub user: Option<super::user::UserPreview>,
    #[serde(default)]
    pub display_text: Option<String>,
    #[serde(default)]
    pub novel_ai_type: Option<i32>,
    #[serde(default)]
    pub cover_image_urls: Option<super::common::ImageUrls>,
    #[serde(default)]
    pub first_novel_id: Option<u64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NovelText {
    #[serde(default)]
    pub novel_text: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_novel_partial() {
        let json = r#"{"id": 999, "title": "My Novel", "text_length": 5000}"#;
        let novel: Novel = serde_json::from_str(json).unwrap();
        assert_eq!(novel.id, 999);
        assert_eq!(novel.text_length, Some(5000));
    }

    #[test]
    fn test_novel_series() {
        let json = r#"{"id": 100, "title": "Series Name"}"#;
        let series: NovelSeries = serde_json::from_str(json).unwrap();
        assert_eq!(series.id, 100);
    }
}
