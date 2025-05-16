use bytes::Bytes;
use std::fmt;
use std::str::FromStr;
use thiserror::Error;

use crate::headers::Headers;

/// HTTP request methods
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Method {
    GET,
    POST,
    PUT,
    DELETE,
    HEAD,
    OPTIONS,
    PATCH,
    CONNECT,
    TRACE,
}

/// Represents an HTTP request
#[derive(Debug)]
pub struct Request {
    /// HTTP method
    pub method: Method,
    /// Request URI
    pub uri: String,
    /// HTTP version
    pub version: (u8, u8),
    /// Headers
    pub headers: Headers,
    /// Request body
    pub body: Bytes,
}

#[derive(Debug, Error)]
pub enum RequestError {
    #[error("Invalid method: {0}")]
    InvalidMethod(String),
    #[error("Invalid request")]
    InvalidRequest,
    #[error("Error parsing headers: {0}")]
    HeaderError(#[from] crate::headers::HeaderError),
    #[error("Parse error: {0}")]
    ParseError(String),
}

impl FromStr for Method {
    type Err = RequestError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_uppercase().as_str() {
            "GET" => Ok(Method::GET),
            "POST" => Ok(Method::POST),
            "PUT" => Ok(Method::PUT),
            "DELETE" => Ok(Method::DELETE),
            "HEAD" => Ok(Method::HEAD),
            "OPTIONS" => Ok(Method::OPTIONS),
            "PATCH" => Ok(Method::PATCH),
            "CONNECT" => Ok(Method::CONNECT),
            "TRACE" => Ok(Method::TRACE),
            _ => Err(RequestError::InvalidMethod(s.to_string())),
        }
    }
}

impl fmt::Display for Method {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let method_str = match self {
            Method::GET => "GET",
            Method::POST => "POST",
            Method::PUT => "PUT",
            Method::DELETE => "DELETE",
            Method::HEAD => "HEAD",
            Method::OPTIONS => "OPTIONS",
            Method::PATCH => "PATCH",
            Method::CONNECT => "CONNECT",
            Method::TRACE => "TRACE",
        };
        write!(f, "{}", method_str)
    }
}

impl Request {
    /// Create a new HTTP request
    pub fn new(method: Method, uri: String, headers: Headers, body: Bytes) -> Self {
        Request {
            method,
            uri,
            version: (1, 1), // Default to HTTP/1.1
            headers,
            body,
        }
    }
}

/// Parse raw HTTP request bytes into a Request object
pub fn parse_request(data: &[u8]) -> Result<Request, RequestError> {
    let mut headers = [httparse::EMPTY_HEADER; 64];
    let mut req = httparse::Request::new(&mut headers);

    let parsed_len = match req.parse(data) {
        Ok(httparse::Status::Complete(len)) => len,
        Ok(httparse::Status::Partial) => return Err(RequestError::ParseError("Incomplete request".to_string())),
        Err(e) => return Err(RequestError::ParseError(e.to_string())),
    };

    let method = req.method
        .ok_or_else(|| RequestError::ParseError("Missing method".to_string()))?;
    let method = Method::from_str(method)?;

    let uri = req.path
        .ok_or_else(|| RequestError::ParseError("Missing URI".to_string()))?
        .to_string();

    let version = req.version
        .ok_or_else(|| RequestError::ParseError("Missing version".to_string()))?;
    let version = (1, if version == 1 { 1 } else { 0 });

    let mut http_headers = Headers::new();
    for header in req.headers {
        let value = std::str::from_utf8(header.value)
            .map_err(|_| RequestError::ParseError("Invalid header value encoding".to_string()))?;
        http_headers.insert(header.name, value)?;
    }

    let body = if parsed_len < data.len() {
        Bytes::copy_from_slice(&data[parsed_len..])
    } else {
        Bytes::new()
    };

    Ok(Request {
        method,
        uri,
        version,
        headers: http_headers,
        body,
    })
}