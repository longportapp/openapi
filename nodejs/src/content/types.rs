use chrono::{DateTime, Utc};
use longport_nodejs_macros::JsObject;

/// Topic author
#[napi_derive::napi]
#[derive(Debug, JsObject, Clone)]
#[js(remote = "longport::content::TopicAuthor")]
pub struct TopicAuthor {
    /// Member ID
    member_id: String,
    /// Display name
    name: String,
    /// Avatar URL
    avatar: String,
}

/// Topic image
#[napi_derive::napi]
#[derive(Debug, JsObject, Clone)]
#[js(remote = "longport::content::TopicImage")]
pub struct TopicImage {
    /// Original image URL
    url: String,
    /// Small thumbnail URL
    sm: String,
    /// Large image URL
    lg: String,
}

/// My topic item (topic created by the current authenticated user)
#[napi_derive::napi]
#[derive(Debug, JsObject, Clone)]
#[js(remote = "longport::content::OwnedTopic")]
pub struct OwnedTopic {
    /// Topic ID
    id: String,
    /// Title
    title: String,
    /// Plain text excerpt
    description: String,
    /// Markdown body
    body: String,
    /// Author
    author: TopicAuthor,
    /// Related stock tickers
    #[js(array)]
    tickers: Vec<String>,
    /// Hashtag names
    #[js(array)]
    hashtags: Vec<String>,
    /// Images
    #[js(array)]
    images: Vec<TopicImage>,
    /// Likes count
    likes_count: i32,
    /// Comments count
    comments_count: i32,
    /// Views count
    views_count: i32,
    /// Shares count
    shares_count: i32,
    /// Content type: "article" or "post"
    topic_type: String,
    /// URL to the full topic page
    detail_url: String,
    /// Created time
    #[js(datetime)]
    created_at: DateTime<Utc>,
    /// Updated time
    #[js(datetime)]
    updated_at: DateTime<Utc>,
}

/// Topic item
#[napi_derive::napi]
#[derive(Debug, JsObject, Clone)]
#[js(remote = "longport::content::TopicItem")]
pub struct TopicItem {
    /// Topic ID
    id: String,
    /// Title
    title: String,
    /// Description
    description: String,
    /// URL
    url: String,
    /// Published time
    #[js(datetime)]
    published_at: DateTime<Utc>,
    /// Comments count
    comments_count: i32,
    /// Likes count
    likes_count: i32,
    /// Shares count
    shares_count: i32,
}

/// News item
#[napi_derive::napi]
#[derive(Debug, JsObject, Clone)]
#[js(remote = "longport::content::NewsItem")]
pub struct NewsItem {
    /// News ID
    id: String,
    /// Title
    title: String,
    /// Description
    description: String,
    /// URL
    url: String,
    /// Published time
    #[js(datetime)]
    published_at: DateTime<Utc>,
    /// Comments count
    comments_count: i32,
    /// Likes count
    likes_count: i32,
    /// Shares count
    shares_count: i32,
}
