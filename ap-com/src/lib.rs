pub mod types;
pub mod db;
pub mod rel;
pub mod query;
pub mod proc;
pub mod models;
pub mod util;
pub mod error;
pub mod auth;
pub mod prelude;

pub use types::{
    id::Id, now, private, Status, Role, GroupRole,
    Gender, Expiration,
    auth::{Provider, ProviderType},
    token::{self, Token, AccessToken, RefreshToken, SessionToken}
};
pub use models::Model;
pub use error::DiLibError;
pub use rel::{Linked, LinkedTo};
pub use db::Db;


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
