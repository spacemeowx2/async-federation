use crate::{
    data_source::{DataSource, DataSourceRequest, HttpOptions},
    utils::ResponseExt,
    Result,
};
use async_graphql::{
    parser::{parse_schema, types},
    Request, Response,
};
use async_trait::async_trait;

#[derive(Debug, serde_derive::Deserialize)]
struct RemoteService {
    sdl: String,
}

#[derive(Debug, serde_derive::Deserialize)]
struct RemoteResponse {
    _service: RemoteService,
}

pub struct Remote {
    name: String,
    url: String,
}

pub struct Local {
    name: String,
    document: types::ServiceDocument,
}

#[async_trait]
pub trait ServiceSource {
    async fn resolve(&self, data_source: &dyn DataSource) -> Result<types::ServiceDocument>;
}

#[async_trait]
impl ServiceSource for Remote {
    async fn resolve(&self, data_source: &dyn DataSource) -> Result<types::ServiceDocument> {
        let resp: RemoteResponse = data_source
            .process(DataSourceRequest::new(
                Request::new("query GetServiceDefinition { _service { sdl } }"),
                Some(HttpOptions::new(self.url.clone())),
            ))
            .await?
            .data()?;

        println!("get sdl {}", resp._service.sdl);
        todo!()
    }
}

impl Remote {
    pub fn new<S1: Into<String>, S2: Into<String>>(name: S1, url: S2) -> Self {
        Self {
            name: name.into(),
            url: url.into(),
        }
    }
}
