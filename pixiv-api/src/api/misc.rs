use crate::PixivApi;
use crate::models::ApiResponse;
use reqwest::Method;

impl PixivApi {
    /// Get UGOIRA animation metadata.
    pub async fn ugoira_metadata(
        &self,
        illust_id: u64,
    ) -> crate::Result<ApiResponse<serde_json::Value>> {
        self.request(
            Method::GET,
            &format!("/v1/ugoira/metadata?illust_id={illust_id}"),
        )
        .await
    }

    /// Get showcase article.
    pub async fn showcase_article(
        &self,
        showcase_id: &str,
    ) -> crate::Result<ApiResponse<serde_json::Value>> {
        self.request(
            Method::GET,
            &format!("/v1/showcase/article?showcase_id={showcase_id}"),
        )
        .await
    }
}
