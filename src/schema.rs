use std::borrow::Cow;

use async_graphql::{
    parser::types, registry, ContainerType, ContextSelectionSet, ObjectType, OutputType,
    Positioned, Type,
};
use async_trait::async_trait;

#[derive(Debug, Clone)]
pub struct Query(Vec<types::ServiceDocument>);

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
