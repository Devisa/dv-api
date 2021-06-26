pub mod db;
pub mod error;
pub mod listen;
pub mod query;
pub mod types;


pub use types::{Id, Model};
pub use error::DdbError;
pub use db::Db;
pub use query::Query;

/* #[cfg(feature = "postgres")]
pub use db::pg::Db;

#[cfg(feature = "sqlite")]
pub use db::sqlite::Db;

 */
