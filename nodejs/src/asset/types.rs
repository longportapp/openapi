/// Statement item
#[napi_derive::napi(object)]
#[derive(Debug, Clone)]
pub struct StatementItem {
    /// Statement date (integer, e.g. 20250301)
    pub dt: i32,
    /// File key used to request the download URL
    pub file_key: String,
}

impl From<longport::asset::StatementItem> for StatementItem {
    fn from(item: longport::asset::StatementItem) -> Self {
        Self {
            dt: item.dt,
            file_key: item.file_key,
        }
    }
}

/// Response for get statement list
#[napi_derive::napi(object)]
#[derive(Debug, Clone)]
pub struct GetStatementListResponse {
    /// List of statement items
    pub list: Vec<StatementItem>,
}

impl From<longport::asset::GetStatementListResponse> for GetStatementListResponse {
    fn from(resp: longport::asset::GetStatementListResponse) -> Self {
        Self {
            list: resp.list.into_iter().map(Into::into).collect(),
        }
    }
}

/// Response for get statement download URL
#[napi_derive::napi(object)]
#[derive(Debug, Clone)]
pub struct GetStatementDownloadUrlResponse {
    /// Presigned download URL
    pub url: String,
}

impl From<longport::asset::GetStatementResponse> for GetStatementDownloadUrlResponse {
    fn from(resp: longport::asset::GetStatementResponse) -> Self {
        Self { url: resp.url }
    }
}
