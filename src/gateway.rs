use crate::{
    data_source::{DataSource, DefaultDataSource},
    source::ServiceSource,
    Result,
};

/// An implemention of Apollo gateway
pub struct Gateway {
    service_list: Vec<Box<dyn ServiceSource>>,
    data_source: Box<dyn DataSource>,
}

impl Gateway {
    pub async fn new(service_list: Vec<Box<dyn ServiceSource>>) -> Self {
        Self {
            service_list,
            data_source: Box::new(DefaultDataSource),
        }
    }
    pub async fn serve(&mut self) -> Result<()> {
        for i in self.service_list.iter() {
            i.resolve(&self.data_source);
        }
        Ok(())
    }
}
