pub mod httpcodec;
pub mod parse;
mod request;
mod response;
mod types;

pub use self::request::Request;
pub use self::response::Error;
pub use self::response::Response;
pub use self::types::Params;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
