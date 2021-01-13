use crate::Result;
use async_graphql::Response;
use serde::de::DeserializeOwned;

pub trait ResponseExt<T> {
    fn data(self) -> Result<T>;
}

impl<T> ResponseExt<T> for Response
where
    T: DeserializeOwned,
{
    fn data(self) -> Result<T> {
        let data = self.into_result()?.data;
        Ok(async_graphql::from_value(data)?)
    }
}
