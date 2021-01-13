use async_graphql::ServerError;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Response error {0:?}")]
    Response(Vec<ServerError>),
    #[error("Error from graphql {0:?}")]
    GraphqlValue(#[from] async_graphql::DeserializerError),
}

impl From<Vec<ServerError>> for Error {
    fn from(e: Vec<ServerError>) -> Self {
        Error::Response(e)
    }
}
