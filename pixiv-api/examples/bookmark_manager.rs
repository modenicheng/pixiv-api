/// Manage bookmarks: list, add, and remove illustration bookmarks.
///
/// Run: cargo run -p pixiv-client --example bookmark_manager
use pixiv_client::PixivApi;

#[tokio::main]
async fn main() -> Result<(), pixiv_client::PixivError> {
    let api = PixivApi::new();

    let token =
        std::env::var("PIXIV_REFRESH_TOKEN").expect("Set PIXIV_REFRESH_TOKEN environment variable");
    api.auth(&token).await?;
    let my_id = api.user_id().await.expect("Not authenticated");
    println!("Authenticated as user {}\n", my_id);

    // List your public bookmarks
    println!("--- Your Bookmarked Illustrations ---\n");
    let bookmarks = api
        .user_bookmarks_illust(my_id, Some("public"), None, None)
        .await?;

    if let Some(data) = &bookmarks.data {
        for illust in data.illusts.iter().take(5) {
            println!(
                "  [{}] {} by {}",
                illust.id,
                illust.title,
                illust
                    .user
                    .as_ref()
                    .and_then(|u| u.name.as_deref())
                    .unwrap_or("unknown")
            );
        }
        if data.illusts.is_empty() {
            println!("  No bookmarks found");
        }
    }

    // Bookmark an illustration (replace with a real ID)
    let target_id: u64 = 12345;
    println!("\n--- Bookmarking illustration {} ---\n", target_id);

    match api
        .illust_bookmark_add(target_id, Some("public"), None)
        .await
    {
        Ok(resp) => {
            println!("Bookmark added!");
            println!("Raw response: {}", serde_json::to_string_pretty(&resp.raw)?);
        }
        Err(e) => eprintln!("Failed to bookmark: {}", e),
    }

    // Check bookmark status
    match api.illust_bookmark_detail(target_id).await {
        Ok(resp) => {
            println!("\nBookmark detail:");
            println!("{}", serde_json::to_string_pretty(&resp.raw)?);
        }
        Err(e) => eprintln!("Failed to get bookmark detail: {}", e),
    }

    // Uncomment to remove the bookmark:
    // match api.illust_bookmark_delete(target_id).await {
    //     Ok(_) => println!("Bookmark removed"),
    //     Err(e) => eprintln!("Failed to remove bookmark: {}", e),
    // }

    Ok(())
}
