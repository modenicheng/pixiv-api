# pixiv-api Rust Project Design

Date: 2026-05-27

## Goal

Build a Rust library + CLI for the Pixiv App API (6.x), achieving full API parity with the Python reference project [pixivpy](https://github.com/upbit/pixivpy).

## Architecture

**Pattern**: Composition with split `impl` blocks across domain files.

Pixivpy uses Python inheritance (`BasePixivAPI` → `AppPixivAPI` → `ByPassSniApi`). Rust has no inheritance, so we use composition: one `PixivApi` struct with methods implemented across multiple files organized by API domain.

This is the industry standard for Rust API clients (used by `octocrab`, `google-apis-rs`, `aws-sdk-rust`).

## Workspace Layout

```
pixiv-api/
├── Cargo.toml              # workspace root
├── pixiv-api/              # library crate
│   ├── Cargo.toml          # features: gfw-bypass
│   └── src/
│       ├── lib.rs           # re-exports, top-level docs
│       ├── api/
│       │   ├── mod.rs       # PixivApi struct + constructor
│       │   ├── auth.rs      # auth(), set_auth(), refresh_token()
│       │   ├── user.rs      # user_detail, user_illusts, user_bookmarks_illust, ...
│       │   ├── illust.rs    # illust_detail, illust_ranking, illust_comments, ...
│       │   ├── novel.rs     # novel_detail, novel_text, novel_series, ...
│       │   ├── search.rs    # search_illust, search_novel, search_user, ...
│       │   └── misc.rs      # ugoira_metadata, showcase_article
│       ├── models/
│       │   ├── mod.rs       # ApiResponse<T> wrapper
│       │   ├── illust.rs    # Illust, ImageUrls, MetaPage, ...
│       │   ├── user.rs      # User, Profile, Workspace, ...
│       │   ├── novel.rs     # Novel, NovelSeries, ...
│       │   ├── search.rs    # SearchSort, SearchDuration enums
│       │   └── common.rs    # Tag, Pagination, Timestamps
│       ├── downloader/
│       │   └── mod.rs       # DownloadManager
│       ├── error.rs         # PixivError enum
│       └── config.rs        # Config, ClientConfig
├── pixiv-dl/               # CLI binary crate
│   ├── Cargo.toml          # depends on pixiv-api + clap + tokio
│   └── src/
│       └── main.rs          # clap CLI: auth/search/illust/download
├── examples/               # usage examples
└── tests/                  # integration tests
```

## Design Decisions

### 1. Response Models — Hybrid Approach

Every API response carries both a typed struct and the raw JSON. This protects against Pixiv API changes breaking the library.

```rust
pub struct ApiResponse<T> {
    /// Parsed typed struct (None if deserialization fails)
    pub data: Option<T>,
    /// Raw JSON — always available regardless of parse success
    pub raw: serde_json::Value,
}
```

Typed structs use `#[serde(default)]` on fields where possible, so partial parse failures don't lose the entire response. Users are documented to always write a raw JSON fallback route.

### 2. Error Handling — Unified Enum

```rust
#[derive(Debug, thiserror::Error)]
pub enum PixivError {
    #[error("authentication failed: {0}")]
    Auth(String),

    #[error("HTTP request failed")]
    Request(#[from] reqwest::Error),

    #[error("API returned status {0}")]
    Status(StatusCode),

    #[error("failed to parse response")]
    Parse(#[from] serde_json::Error),

    #[error("download failed: {0}")]
    Download(String),

    #[error("I/O error")]
    Io(#[from] std::io::Error),
}

pub type Result<T> = std::result::Result<T, PixivError>;
```

### 3. Authentication

Support `refresh_token` flow only (password auth is deprecated by Pixiv).

The auth flow:
1. POST to `https://oauth.secure.pixiv.net/auth/token`
2. Include `x-client-time` (UTC timestamp) and `x-client-hash` (MD5 of timestamp + hash_secret)
3. Store `access_token`, `refresh_token`, `user_id` on the `PixivApi` struct

### 4. SNI Bypass (GFW Feature)

Gated behind `gfw-bypass` feature flag. Even when the feature is compiled, bypass must be explicitly enabled via `with_bypass()`.

```toml
[features]
gfw-bypass = []
```

```rust
impl PixivApi {
    #[cfg(feature = "gfw-bypass")]
    pub fn with_bypass(mut self) -> Self {
        // replace DNS resolver with DoH-based resolution
        self
    }
}
```

### 5. CLI (pixiv-dl)

Uses `clap` (derive API) for argument parsing and `tokio` for async runtime.

Subcommands:
- `auth --token <refresh_token>` — authenticate
- `search <keyword> [--sort <sort>] [--duration <duration>]` — search illustrations
- `illust <id>` — show illustration details
- `download <id> [ids...] [--output <dir>]` — download illustrations

### 6. Dependencies

| Crate | Purpose |
|---|---|
| `reqwest` | HTTP client (with `json` and `rustls-tls` features) |
| `serde` / `serde_json` | JSON serialization/deserialization |
| `thiserror` | Error type derivation |
| `tokio` | Async runtime (with `full` feature) |
| `chrono` | Timestamp handling |
| `md-5` | MD5 hash for auth headers |
| `clap` | CLI argument parsing (with `derive` feature) |

### 7. Pagination

Pixiv API responses include a `next_url` field. The library provides a helper to extract query parameters from it:

```rust
impl PixivApi {
    pub fn parse_next_url(url: &str) -> Option<HashMap<String, String>> { ... }
}
```

## API Endpoints (Full Parity with pixivpy)

### User
- `user_detail(user_id)` — user info
- `user_illusts(user_id, type)` — user's illustrations
- `user_bookmarks_illust(user_id)` — bookmarked illustrations
- `user_bookmarks_novel(user_id)` — bookmarked novels
- `user_related(user_id)` — related users
- `user_recommended()` — recommended users
- `user_following(user_id)` — following list
- `user_follower(user_id)` — followers
- `user_mypixiv(user_id)` — Pixiv friends
- `user_list(user_id)` — user list
- `user_novels(user_id)` — user's novels
- `user_follow_add(user_id)` — follow user
- `user_follow_delete(user_id)` — unfollow user
- `user_bookmark_tags_illust(user_id)` — bookmark tags
- `user_edit_ai_show_settings(illust_ai_type)` — AI settings

### Illustration
- `illust_detail(illust_id)` — illustration info
- `illust_comments(illust_id)` — comments
- `illust_related(illust_id)` — related illustrations
- `illust_recommended()` — recommended illustrations
- `illust_ranking(mode)` — ranking
- `illust_follow()` — followed artists' new works
- `illust_new()` — newest illustrations
- `illust_bookmark_detail(illust_id)` — bookmark status
- `illust_bookmark_add(illust_id)` — add bookmark
- `illust_bookmark_delete(illust_id)` — remove bookmark

### Novel
- `novel_detail(novel_id)` — novel info
- `novel_comments(novel_id)` — comments
- `novel_recommended()` — recommended novels
- `novel_new()` — newest novels
- `novel_follow()` — followed artists' new novels
- `novel_series(series_id)` — series info
- `novel_text(novel_id)` — novel text content
- `webview_novel(novel_id)` — webview novel

### Search
- `search_illust(word, sort, duration, ...)` — search illustrations
- `search_novel(word, sort, ...)` — search novels
- `search_user(word)` — search users
- `trending_tags_illust()` — trending tags

### Misc
- `ugoira_metadata(illust_id)` — UGOIRA animation metadata
- `showcase_article(showcase_id)` — showcase articles

## Conventions

- Conventional Commits (enforced by pre-commit hook)
- `cargo fmt` + `cargo clippy -D warnings` on every commit
- All public API items must have doc comments
- Tests in `tests/` directory, examples in `examples/`
