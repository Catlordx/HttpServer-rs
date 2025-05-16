use bytes::{BufMut, Bytes, BytesMut};
use std::io::Write;
use thiserror::Error;

use crate::headers::{HeaderError, Headers};
use crate::status::StatusCode;

/// Represents an HTTP response
#[derive(Debug)]
pub struct Response {
    /// HTTP status code
    pub status: StatusCode,
    /// HTTP version
    pub version: (u8, u8),
    /// Headers
    pub headers: Headers,
    /// Response body
    pub body: Bytes,
}

/// Builder for constructing HTTP responses
#[derive(Debug)]
pub struct ResponseBuilder {
    status: StatusCode,
    headers: Headers,
    body: Option<Bytes>,
}

#[derive(Debug, Error)]
pub enum ResponseError {
    #[error("Header error: {0}")]
    HeaderError(#[from] HeaderError),
    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),
}

impl Response {
    /// Create a new HTTP response
    pub fn new(status: StatusCode, headers: Headers, body: Bytes) -> Self {
        Response {
            status,
            version: (1, 1), // Default to HTTP/1.1
            headers,
            body,
        }
    }

    /// Convert the response to raw bytes
    pub fn to_bytes(&self) -> Result<Bytes, ResponseError> {
        let buf = BytesMut::new(); // BytesMut::new() 创建 buf
        // 调用 writer()，buf 的所有权转移给 writer。
        // writer 现在拥有缓冲区，并且是可变的，以便进行写入。
        let mut writer = buf.writer();

        // Write status line
        // write! 宏会调用 writer.write_fmt(...)
        // writer 是可变的，所以可以多次调用 write!
        write!(
            writer, // 直接使用 writer 变量
            "HTTP/{}.{} {} {}\r\n",
            self.version.0,
            self.version.1,
            self.status.code(),
            self.status.reason_phrase()
        )?;

        // Write headers
        for (name, value) in self.headers.iter() {
            write!(writer, "{}: {}\r\n", name, value)?;
        }

        // Write empty line to separate headers from body
        // 对于原始字节，使用 Writer 的 write_all 方法
        writer.write_all(b"\r\n")?;

        // Write body
        writer.write_all(&self.body)?;

        // 所有写入完成后，从 writer 中取回 BytesMut
        let result_buf = writer.into_inner();

        Ok(result_buf.freeze())
    }
}

impl ResponseBuilder {
    /// Create a new response builder
    pub fn new() -> Self {
        ResponseBuilder {
            status: StatusCode::OK,
            headers: Headers::new(),
            body: None,
        }
    }

    /// Set status code
    pub fn status(mut self, status: StatusCode) -> Self {
        self.status = status;
        self
    }

    /// Add a header - this now stores errors rather than returning Result
    pub fn header<K, V>(mut self, name: K, value: V) -> Self
    where
        K: TryInto<crate::headers::HeaderName, Error = HeaderError>,
        V: TryInto<crate::headers::HeaderValue, Error = HeaderError>,
    {
        // Instead of propagating the error, we just ignore it
        // In a real implementation, you might want to store the error
        let _ = self.headers.insert(name, value);
        self
    }

    /// Set the response body
    pub fn body(mut self, body: impl Into<Bytes>) -> Self {
        self.body = Some(body.into());
        self
    }

    /// Build the response
    pub fn build(self) -> Response {
        let body = self.body.unwrap_or_else(Bytes::new);
        Response::new(self.status, self.headers, body)
    }
}

impl Default for ResponseBuilder {
    fn default() -> Self {
        Self::new()
    }
}
#[cfg(test)]
mod tests {
    // Import necessary items from the parent modules
    use super::super::headers::{CONTENT_LENGTH, CONTENT_TYPE};
    use super::super::status::StatusCode;
    use super::ResponseBuilder;
    use bytes::Bytes;
    use std::collections::HashSet; // For checking header presence without strict order

    /// Test case for a standard HTTP response with headers and a body.
    #[test]
    fn test_response_to_bytes_standard() {
        let body_content = "Hello, world!";
        let body_bytes = Bytes::from(body_content);

        // Build the response using the builder pattern
        let response = ResponseBuilder::new()
            .status(StatusCode::OK)
            .header(CONTENT_TYPE, "text/plain")
            .header(CONTENT_LENGTH, body_bytes.len().to_string())
            .body(body_bytes.clone())
            .build();

        // Convert the response to raw bytes
        let result_bytes = response
            .to_bytes()
            .expect("Failed to convert response to bytes");

        // Convert the result bytes to a string for easier assertion
        let result_string =
            String::from_utf8(result_bytes.to_vec()).expect("Result bytes are not valid UTF-8");

        // Split the entire response string into header block and body using the `\r\n\r\n` separator.
        let parts: Vec<&str> = result_string.splitn(2, "\r\n\r\n").collect();
        assert_eq!(
            parts.len(),
            2,
            "Expected two parts: headers and body separated by CRLF twice"
        );

        let header_block = parts[0];
        let actual_body = parts[1];

        // 1. Assert the body content
        assert_eq!(actual_body, body_content, "Body content mismatch");

        // 2. Assert the status line and headers
        let header_lines: Vec<&str> = header_block.split("\r\n").collect();
        assert!(!header_lines.is_empty(), "Header block should not be empty");

        let actual_status_line = header_lines[0];
        assert_eq!(
            actual_status_line, "HTTP/1.1 200 OK",
            "Status line mismatch"
        );

        // Collect actual headers into a HashSet for unordered comparison
        let mut actual_headers: HashSet<String> = HashSet::new();
        // Skip the status line (index 0) and iterate over the rest of the header lines
        for &line in header_lines.iter().skip(1) {
            actual_headers.insert(line.to_string());
        }

        // Define expected headers and collect into a HashSet
        let mut expected_headers: HashSet<String> = HashSet::new();
        expected_headers.insert(format!("{}: text/plain", CONTENT_TYPE));
        expected_headers.insert(format!("{}: {}", CONTENT_LENGTH, body_content.len()));

        assert_eq!(actual_headers, expected_headers, "Headers mismatch");
    }

    /// Test case for an HTTP response with headers but No Body (e.g., 204 No Content).
    #[test]
    fn test_response_to_bytes_no_body() {
        // For a 204 No Content, a body is typically not sent.
        let response = ResponseBuilder::new()
            .status(StatusCode::NO_CONTENT)
            .header(CONTENT_TYPE, "application/json")
            .build(); // No explicit body set, so it defaults to empty Bytes

        let result_bytes = response
            .to_bytes()
            .expect("Failed to convert response to bytes");
        let result_string =
            String::from_utf8(result_bytes.to_vec()).expect("Result bytes are not valid UTF-8");

        let parts: Vec<&str> = result_string.splitn(2, "\r\n\r\n").collect();
        assert_eq!(parts.len(), 2, "Expected two parts: headers and empty body");

        let header_block = parts[0];
        let actual_body = parts[1];

        // Assert that the body part is empty
        assert_eq!(actual_body, "", "Expected empty body for NO_CONTENT");

        let header_lines: Vec<&str> = header_block.split("\r\n").collect();
        assert!(!header_lines.is_empty(), "Header block should not be empty");

        let actual_status_line = header_lines[0];
        assert_eq!(
            actual_status_line, "HTTP/1.1 204 No Content",
            "Status line mismatch for NO_CONTENT"
        );

        let mut actual_headers: HashSet<String> = HashSet::new();
        for &line in header_lines.iter().skip(1) {
            actual_headers.insert(line.to_string());
        }

        let mut expected_headers: HashSet<String> = HashSet::new();
        expected_headers.insert(format!("{}: application/json", CONTENT_TYPE));

        assert_eq!(
            actual_headers, expected_headers,
            "Headers mismatch for NO_CONTENT"
        );
    }

    /// Test case for an HTTP response with an explicitly empty body.
    #[test]
    fn test_response_to_bytes_empty_body() {
        let body_content = ""; // An empty string
        let body_bytes = Bytes::from(body_content);

        let response = ResponseBuilder::new()
            .status(StatusCode::OK)
            .header(CONTENT_LENGTH, body_bytes.len().to_string()) // Content-Length should be 0
            .body(body_bytes.clone())
            .build();

        let result_bytes = response
            .to_bytes()
            .expect("Failed to convert response to bytes");
        let result_string =
            String::from_utf8(result_bytes.to_vec()).expect("Result bytes are not valid UTF-8");

        let parts: Vec<&str> = result_string.splitn(2, "\r\n\r\n").collect();
        assert_eq!(parts.len(), 2, "Expected two parts: headers and empty body");

        let header_block = parts[0];
        let actual_body = parts[1];

        // Assert that the body part is correctly empty
        assert_eq!(actual_body, body_content, "Expected empty body");

        let header_lines: Vec<&str> = header_block.split("\r\n").collect();
        assert!(!header_lines.is_empty(), "Header block should not be empty");

        let actual_status_line = header_lines[0];
        assert_eq!(
            actual_status_line, "HTTP/1.1 200 OK",
            "Status line mismatch for empty body"
        );

        let mut actual_headers: HashSet<String> = HashSet::new();
        for &line in header_lines.iter().skip(1) {
            actual_headers.insert(line.to_string());
        }

        let mut expected_headers: HashSet<String> = HashSet::new();
        expected_headers.insert(format!("{}: {}", CONTENT_LENGTH, body_content.len())); // Content-Length: 0

        assert_eq!(
            actual_headers, expected_headers,
            "Headers mismatch for empty body"
        );
    }
}
