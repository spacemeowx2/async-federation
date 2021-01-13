use std::borrow::Cow;

use async_graphql::{
    parser::{parse_schema, types},
    registry, ContainerType, ContextSelectionSet, ObjectType, OutputType, Positioned, Type,
};
use async_trait::async_trait;

pub use error::Error;
pub type Result<T, E = Error> = std::result::Result<T, E>;

mod data_source;
mod error;
pub mod gateway;
pub mod source;
mod utils;

#[derive(Default, Debug, Clone)]
pub struct Query;

impl Type for Query {
    fn type_name() -> Cow<'static, str> {
        Cow::Borrowed("Query")
    }

    fn create_type_info(registry: &mut registry::Registry) -> String {
        registry.create_type::<Self, _>(|_| registry::MetaType::Object {
            name: "Query".to_string(),
            description: None,
            fields: Default::default(),
            cache_control: Default::default(),
            extends: false,
            keys: None,
            visible: None,
        })
    }
}

#[async_trait]
impl OutputType for Query {
    async fn resolve(
        &self,
        ctx: &ContextSelectionSet<'_>,
        field: &Positioned<types::Field>,
    ) -> async_graphql::ServerResult<async_graphql::Value> {
        todo!()
    }
}

pub struct ServiceDefinition {
    document: types::ServiceDocument,
    url: String,
    name: String,
}
