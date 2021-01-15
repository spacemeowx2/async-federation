#![allow(dead_code)]

use std::borrow::Cow;

use async_graphql::ServerError;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Response error: {0:?}")]
    Response(Vec<ServerError>),
    #[error("Error from graphql: {0}")]
    GraphqlValue(#[from] async_graphql::DeserializerError),
    #[error("Invalid input: {0}")]
    InvalidInput(Cow<'static, str>),
    #[error("InvalidMethod: {0:?}")]
    InvalidMethod(#[from] http::method::InvalidMethod),
    #[cfg(feature = "reqwest")]
    #[error("reqwest error: {0}")]
    Reqwest(#[from] reqwest::Error),
    #[error("serde_json {0}")]
    SerdeJson(#[from] serde_json::Error),
    #[error("parse error: {0}")]
    Parse(#[from] async_graphql::parser::Error),
}

impl From<Vec<ServerError>> for Error {
    fn from(e: Vec<ServerError>) -> Self {
        Error::Response(e)
    }
}

impl Error {
    pub fn invalid_input(s: &'static str) -> Error {
        Error::InvalidInput(Cow::Borrowed(s))
    }
}
