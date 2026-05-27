/// Fetch a user's profile and list their illustrations.
///
/// Run: cargo run -p pixiv-client --example user_profile
use pixiv_client::PixivApi;

#[tokio::main]
async fn main() -> Result<(), pixiv_client::PixivError> {
    let api = PixivApi::new();

    let token =
        std::env::var("PIXIV_REFRESH_TOKEN").expect("Set PIXIV_REFRESH_TOKEN environment variable");
    api.auth(&token).await?;

    // Get your own user ID
    let my_id = api.user_id().await.expect("Not authenticated");
    println!("Your user ID: {}\n", my_id);

    // Fetch user details
    let detail = api.user_detail(my_id).await?;
    if let Some(user) = &detail.data {
        println!("Name: {}", user.user.name.as_deref().unwrap_or("(unknown)"));
        println!(
            "Account: {}",
            user.user.account.as_deref().unwrap_or("(unknown)")
        );
        if let Some(profile) = &user.profile {
            println!(
                "Total illustrations: {}",
                profile.total_illusts.unwrap_or(0)
            );
            println!("Total novels: {}", profile.total_novels.unwrap_or(0));
            println!(
                "Total bookmarks: {}",
                profile.total_illust_bookmarks_public.unwrap_or(0)
            );
        }
    }

    println!("\n--- Recent Illustrations ---\n");

    // Fetch user's illustrations
    let illusts = api.user_illusts(my_id, None, None).await?;
    if let Some(data) = &illusts.data {
        for illust in data.illusts.iter().take(5) {
            println!(
                "  [{}] {} ({} views, {} bookmarks)",
                illust.id,
                illust.title,
                illust.total_view.unwrap_or(0),
                illust.total_bookmarks.unwrap_or(0)
            );
        }
        if data.illusts.is_empty() {
            println!("  No illustrations found");
        }
    }

    Ok(())
}
