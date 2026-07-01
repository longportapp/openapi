use longport::content::{CreateTopicOptions, MyTopicsOptions};

/// Options for listing topics created by the current authenticated user
#[napi_derive::napi(object)]
#[derive(Debug, Default)]
pub struct MyTopicsRequest {
    /// Page number (default 1)
    pub page: Option<i32>,
    /// Records per page, range 1~500 (default 50)
    pub size: Option<i32>,
    /// Filter by topic type: "article" or "post"; empty returns all
    pub topic_type: Option<String>,
}

impl From<MyTopicsRequest> for MyTopicsOptions {
    fn from(
        MyTopicsRequest {
            page,
            size,
            topic_type,
        }: MyTopicsRequest,
    ) -> Self {
        Self {
            page,
            size,
            topic_type,
        }
    }
}

/// Options for creating a topic
#[napi_derive::napi(object)]
#[derive(Debug)]
pub struct CreateTopicRequest {
    /// Topic title (required)
    pub title: String,
    /// Topic body in Markdown format (required)
    pub body: String,
    /// Content type: "article" (long-form) or "post" (short post, default)
    pub topic_type: Option<String>,
    /// Related stock tickers, format: {symbol}.{market}, max 10
    pub tickers: Option<Vec<String>>,
    /// Hashtag names, max 5
    pub hashtags: Option<Vec<String>>,
}

impl From<CreateTopicRequest> for CreateTopicOptions {
    fn from(
        CreateTopicRequest {
            title,
            body,
            topic_type,
            tickers,
            hashtags,
        }: CreateTopicRequest,
    ) -> Self {
        Self {
            title,
            body,
            topic_type,
            tickers,
            hashtags,
        }
    }
}
