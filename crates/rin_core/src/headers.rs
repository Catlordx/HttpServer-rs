use std::collections::HashMap;
use std::fmt;
use std::str::FromStr;
use thiserror::Error;

/// A collection of HTTP headers
#[derive(Debug, Default, Clone)]
pub struct Headers {
    headers: HashMap<HeaderName, HeaderValue>,
}

/// Represents an HTTP header name
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct HeaderName(String);

/// Represents an HTTP header value
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HeaderValue(String);

/// Common HTTP headers as constants
pub const CONTENT_TYPE: &str = "Content-Type";
pub const CONTENT_LENGTH: &str = "Content-Length";
pub const HOST: &str = "Host";
pub const USER_AGENT: &str = "User-Agent";
pub const ACCEPT: &str = "Accept";

#[derive(Debug, Error)]
pub enum HeaderError {
    #[error("Invalid header name: {0}")]
    InvalidName(String),
    #[error("Invalid header value")]
    InvalidValue,
}

impl Headers {
    /// Create a new, empty headers collection
    pub fn new() -> Self {
        Headers {
            headers: HashMap::new(),
        }
    }

    /// Insert a header into the collection
    pub fn insert<K, V>(&mut self, name: K, value: V) -> Result<(), HeaderError>
    where
        K: TryInto<HeaderName, Error = HeaderError>,
        V: TryInto<HeaderValue, Error = HeaderError>,
    {
        let name = name.try_into()?;
        let value = value.try_into()?;
        self.headers.insert(name, value);
        Ok(())
    }

    /// Get a header value by name
    pub fn get<K>(&self, name: K) -> Option<&HeaderValue>
    where
        K: AsRef<str>,
    {
        self.headers.get(&HeaderName(name.as_ref().to_string()))
    }

    /// Get an iterator over all headers
    pub fn iter(&self) -> impl Iterator<Item = (&HeaderName, &HeaderValue)> {
        self.headers.iter()
    }
}

impl FromStr for HeaderName {
    type Err = HeaderError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // Simple validation: header names should be ASCII and not contain spaces or control chars
        if s.trim().is_empty() || s.chars().any(|c| c.is_control() || c.is_whitespace()) {
            return Err(HeaderError::InvalidName(s.to_string()));
        }
        Ok(HeaderName(s.to_string()))
    }
}

impl TryFrom<String> for HeaderName {
    type Error = HeaderError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        HeaderName::from_str(&value)
    }
}

impl TryFrom<&str> for HeaderName {
    type Error = HeaderError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        HeaderName::from_str(value)
    }
}

impl FromStr for HeaderValue {
    type Err = HeaderError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // Simple validation: header values shouldn't contain certain control characters
        if s.chars().any(|c| c.is_control() && c != '\t') {
            return Err(HeaderError::InvalidValue);
        }
        Ok(HeaderValue(s.to_string()))
    }
}

impl TryFrom<String> for HeaderValue {
    type Error = HeaderError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        HeaderValue::from_str(&value)
    }
}

impl TryFrom<&str> for HeaderValue {
    type Error = HeaderError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        HeaderValue::from_str(value)
    }
}

impl fmt::Display for HeaderName {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl fmt::Display for HeaderValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}