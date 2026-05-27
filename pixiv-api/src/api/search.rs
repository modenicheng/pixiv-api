use crate::PixivApi;
use crate::models::ApiResponse;
use reqwest::Method;

impl PixivApi {
    /// Search illustrations.
    pub async fn search_illust(
        &self,
        word: &str,
        sort: Option<&str>,
        duration: Option<&str>,
        search_target: Option<&str>,
        offset: Option<u32>,
    ) -> crate::Result<ApiResponse<serde_json::Value>> {
        let mut path = format!("/v1/search/illust?word={word}");
        if let Some(s) = sort {
            path.push_str(&format!("&sort={s}"));
        }
        if let Some(d) = duration {
            path.push_str(&format!("&duration={d}"));
        }
        if let Some(t) = search_target {
            path.push_str(&format!("&search_target={t}"));
        }
        if let Some(o) = offset {
            path.push_str(&format!("&offset={o}"));
        }
        self.request(Method::GET, &path).await
    }

    /// Search novels.
    pub async fn search_novel(
        &self,
        word: &str,
        sort: Option<&str>,
        search_target: Option<&str>,
        offset: Option<u32>,
    ) -> crate::Result<ApiResponse<serde_json::Value>> {
        let mut path = format!("/v1/search/novel?word={word}");
        if let Some(s) = sort {
            path.push_str(&format!("&sort={s}"));
        }
        if let Some(t) = search_target {
            path.push_str(&format!("&search_target={t}"));
        }
        if let Some(o) = offset {
            path.push_str(&format!("&offset={o}"));
        }
        self.request(Method::GET, &path).await
    }

    /// Search users.
    pub async fn search_user(
        &self,
        word: &str,
        offset: Option<u32>,
    ) -> crate::Result<ApiResponse<serde_json::Value>> {
        let mut path = format!("/v1/search/user?word={word}");
        if let Some(o) = offset {
            path.push_str(&format!("&offset={o}"));
        }
        self.request(Method::GET, &path).await
    }

    /// Get trending illustration tags.
    pub async fn trending_tags_illust(&self) -> crate::Result<ApiResponse<serde_json::Value>> {
        self.request(Method::GET, "/v1/trending-tags/illust").await
    }
}
