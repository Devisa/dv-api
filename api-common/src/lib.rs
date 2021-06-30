pub mod types;
pub mod query;
pub mod proc;
pub mod models;
pub mod util;
pub mod error;
pub mod auth;
pub mod prelude;


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
