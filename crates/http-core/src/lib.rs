//! Core HTTP protocol implementation
//!
//! This crate provides fundamental HTTP protocol functionality including
//! request parsing, response construction, header management, and status codes.

mod headers;
mod request;
mod response;
mod status;

pub use headers::{Headers, HeaderName, HeaderValue, CONTENT_TYPE, CONTENT_LENGTH};
pub use request::{Request, Method, parse_request};
pub use response::{Response, ResponseBuilder};
pub use status::StatusCode;

pub fn add1(a: i32, b: i32) -> i32 {
    // This function is here temporarily just to satisfy the existing main.rs
    // It should be removed later
    a + b
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add1() {
        assert_eq!(add1(1, 2), 3);
    }
}