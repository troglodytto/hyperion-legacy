use crate::{HttpBody, HttpCookie, HttpError, HttpHeader, HttpMethod};
use std::fmt::Display;

#[derive(Debug, Clone, Copy, Default)]
pub enum HttpVersion {
    Http1_0,
    #[default]
    Http1_1,
}

impl Display for HttpVersion {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let version = match self {
            HttpVersion::Http1_0 => 1.0,
            HttpVersion::Http1_1 => 1.1,
        };

        write!(f, "{version}",)
    }
}

#[derive(Debug, Clone)]
pub struct HttpRequest {
    version: HttpVersion,
    method: HttpMethod,
    headers: Vec<HttpHeader>,
    path: String,
    body: Option<HttpBody>, // @todo
}

impl HttpRequest {
    pub fn new(request_bytes: Vec<u8>) -> Result<Self, HttpError> {
        let request_metadata = match std::str::from_utf8(&request_bytes) {
            Ok(data) => Ok(data),
            Err(_) => Err(HttpError::BadRequest {
                message: "Invalid UTF-8 Sequence in headers".to_string(),
            }),
        }?;

        let mut request_metadata = request_metadata.split("\r\n");

        let request_line = match request_metadata.next() {
            Some(request_line) => Ok(request_line),
            None => Err(HttpError::BadRequest {
                message: "Request Line not found".to_string(),
            }),
        }?;

        let mut request_line = request_line.split(" ");

        let method = match request_line.next() {
            Some(method_string) => HttpMethod::new(method_string),
            None => Err(HttpError::BadRequest {
                message: "Method Not Allowed".to_string(),
            }),
        }?;

        let path = match request_line.next() {
            Some(path) => Ok(String::from(path)),
            None => Err(HttpError::BadRequest {
                message: "Invalid Path".to_string(),
            }),
        }?;

        let version = match request_line.next() {
            Some(version) => Ok(match &version[version.len() - 3..] {
                "1.1" => HttpVersion::Http1_1,
                "1.0" => HttpVersion::Http1_0,
                _ => HttpVersion::default(),
            }),
            None => Err(HttpError::BadRequest {
                message: "Invalid HTTP Version".to_string(),
            }),
        }?;

        let headers = request_metadata.filter_map(HttpHeader::from).collect();

        Ok(HttpRequest {
            version,
            method,
            headers,
            path,
            body: None,
        })
    }

    pub fn get_content_length(&self) -> Option<usize> {
        self.get_header("Content-Length").map_or(None, |header| {
            header
                .value
                .parse()
                .map_or(None, |content_length| Some(content_length))
        })
    }

    pub fn get_content_type(&self) -> Option<String> {
        self.get_header("Content-Type")
            .map(|header| header.value.clone())
    }

    pub fn set_body(&mut self, body: HttpBody) {
        self.body = Some(body);
    }

    pub fn get_header(&self, name: &str) -> Option<&HttpHeader> {
        self.headers.iter().find(|header| header.name == name)
    }

    pub fn get_cookies(&self) -> Vec<HttpCookie> {
        self.headers
            .iter()
            .filter(|header| header.name == "Cookie")
            .map(HttpCookie::from_header)
            .collect()
    }

    pub fn get_cookie(&self, name: String) -> Option<HttpCookie> {
        self.get_cookies()
            .iter()
            .find(|cookie| cookie.name == name)
            .map(|cookie| cookie.clone())
    }
}

impl Display for HttpRequest {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            formatter,
            "{} {} HTTP/{}\r\n",
            self.method, self.path, self.version
        )?;

        for header in self.headers.clone() {
            write!(formatter, "{header}\r\n")?;
        }

        write!(formatter, "\r\n")?;

        if let Some(body) = &self.body {
            match std::str::from_utf8(&body.as_bytes()) {
                Ok(string) => write!(formatter, "{string}")?,
                Err(_) => write!(formatter, "{:?}", self.get_content_type())?,
            };
        }

        Ok(())
    }
}
