use crate::{Error, Result};
use async_graphql::{Request, Response};
use async_trait::async_trait;
use http::{header::HeaderMap, method::Method};

/// DataSource http options
pub struct HttpOptions {
    pub method: Method,
    pub url: String,
    pub headers: HeaderMap,
}

impl HttpOptions {
    pub fn new(url: String) -> Self {
        Self {
            method: Method::POST,
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
impl<T: DataSource + ?Sized> DataSource for Box<T> {
    async fn process(&self, request: DataSourceRequest) -> Result<Response> {
        T::process(&self, request).await
    }
}

#[cfg(feature = "reqwest")]
pub struct DefaultDataSource;

#[cfg(feature = "reqwest")]
#[async_trait]
impl DataSource for DefaultDataSource {
    async fn process(&self, request: DataSourceRequest) -> Result<Response> {
        let client = reqwest::Client::new();
        let http = request
            .http
            .ok_or(Error::invalid_input("request don't have http field"))?;
        let resp = client
            .request(http.method, &http.url)
            .headers(http.headers)
            .body(serde_json::to_string(&request.request)?)
            .send()
            .await?;

        Ok(resp.json().await?)
    }
}
