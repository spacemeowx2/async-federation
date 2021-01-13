use crate::Result;
use async_graphql::{Request, Response};
use async_trait::async_trait;

/// DataSource http options
pub struct HttpOptions {
    pub method: String,
    pub url: String,
    pub headers: http::header::HeaderMap,
}

impl HttpOptions {
    pub fn new(url: String) -> Self {
        Self {
            method: "POST".to_string(),
            url,
            headers: Default::default(),
        }
    }
}

/// DataSource request
pub struct DataSourceRequest {
    pub request: Request,
    pub http: Option<HttpOptions>,
}

impl DataSourceRequest {
    pub fn new(request: Request, http: Option<HttpOptions>) -> Self {
        Self { request, http }
    }
}

#[async_trait]
pub trait DataSource: Send + Sync {
    async fn process(&self, request: DataSourceRequest) -> Result<Response>;
}

#[async_trait]
impl DataSource for Box<dyn DataSource> {
    async fn process(&self, request: DataSourceRequest) -> Result<Response> {
        DataSource::process(&*self, request).await
    }
}

#[cfg(feature = "reqwest")]
pub struct DefaultDataSource;

#[cfg(feature = "reqwest")]
#[async_trait]
impl DataSource for DefaultDataSource {
    async fn process(&self, request: DataSourceRequest) -> Result<Response> {
        // reqwest::get(url)
        todo!()
    }
}
