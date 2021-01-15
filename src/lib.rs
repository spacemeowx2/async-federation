pub use error::Error;
pub type Result<T, E = Error> = std::result::Result<T, E>;

mod data_source;
mod error;
pub mod gateway;
pub mod schema;
pub mod source;
mod utils;
