mod parse;
mod request;
mod response;

pub use self::request::Request;
pub use self::response::Response;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
