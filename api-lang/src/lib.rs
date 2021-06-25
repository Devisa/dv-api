pub mod error;
pub mod prelude;
pub mod grammar;
pub mod ast;
pub mod parse;
pub mod token;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
