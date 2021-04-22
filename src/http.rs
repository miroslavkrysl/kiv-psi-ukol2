//! HTTP protocol related structures and functions.

use std::fmt;
use std::fmt::{Display, Formatter};
use thiserror::Error;

/// HTTP request parsing error.
#[derive(Error, Debug)]
pub enum HttpParseError {
    #[error("Malformed HTTP request line.")]
    MalformedRequestLine,
}

/// HTTP request method
/// Only GET method is relevant in this project
#[derive(Clone)]
pub enum HttpMethod {
    Get,
    Other(String),
}

impl HttpMethod {
    pub fn to_string(&self) -> String {
        match self {
            HttpMethod::Get => String::from("GET"),
            HttpMethod::Other(method) => method.clone(),
        }
    }

    /// Parses the HTTP request method.
    fn parse(method_string: &str) -> Self {
        match method_string {
            "GET" => HttpMethod::Get,
            unknown => HttpMethod::Other(String::from(unknown)),
        }
    }
}

impl Display for HttpMethod {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_string())
    }
}

/// HTTP protocol version
#[derive(Clone)]
pub enum HttpVersion {
    Http1_1,
    Other(String),
}

impl HttpVersion {
    pub fn to_string(&self) -> String {
        match self {
            HttpVersion::Http1_1 => String::from("HTTP/1.1"),
            HttpVersion::Other(version) => version.clone(),
        }
    }

    /// Parses the HTTP version.
    fn parse(version_string: &str) -> Self {
        match version_string {
            "HTTP/1.1" => HttpVersion::Http1_1,
            unknown => HttpVersion::Other(String::from(unknown)),
        }
    }
}

/// HTTP Response.
/// No headers are needed in this project.
#[derive(Clone)]
pub struct HttpResponse {
    version: HttpVersion,
    status_code: u16,
    content: Option<Vec<u8>>,
}

impl HttpResponse {
    pub fn new(version: HttpVersion, status_code: u16, content: Option<Vec<u8>>) -> Self {
        HttpResponse {
            version,
            status_code,
            content,
        }
    }

    pub fn status_code(&self) -> u16 {
        self.status_code
    }

    pub fn content(&self) -> &Option<Vec<u8>> {
        &self.content
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::new();

        bytes.extend(self.version.to_string().bytes());
        bytes.push(b' ');
        bytes.extend(self.status_code.to_string().bytes());
        bytes.extend_from_slice(b"\r\n\r\n");

        if let Some(ref content) = self.content {
            bytes.extend_from_slice(content);
        }

        return bytes;
    }
}

/// HTTP request line.
#[derive(Clone)]
pub struct HttpRequestLine {
    method: HttpMethod,
    uri: String,
    version: HttpVersion,
}

impl HttpRequestLine {
    /// Parses the HTTP request line.
    /// Return `None` if the request line is incomplete.
    pub fn parse(request_line: &str) -> Result<Option<Self>, HttpParseError> {
        // get the whole line until first CRLF
        let line = match request_line.find("\r\n") {
            None => return Ok(None),
            Some(i) => &request_line[..i],
        };

        // split line by SP
        let mut parts = line.split(' ');

        let method = match parts.next() {
            None => return Err(HttpParseError::MalformedRequestLine),
            Some(method_string) => HttpMethod::parse(method_string),
        };

        let uri = match parts.next() {
            None => return Err(HttpParseError::MalformedRequestLine),
            Some(uri_string) => String::from(uri_string),
        };

        let version = match parts.next() {
            None => return Err(HttpParseError::MalformedRequestLine),
            Some(version_string) => HttpVersion::parse(version_string),
        };

        if let Some(_) = parts.next() {
            return Err(HttpParseError::MalformedRequestLine);
        }

        return Ok(Some(HttpRequestLine {
            method,
            uri,
            version,
        }));
    }

    pub fn method(&self) -> &HttpMethod {
        &self.method
    }

    pub fn uri(&self) -> &str {
        &self.uri
    }

    pub fn version(&self) -> &HttpVersion {
        &self.version
    }
}
