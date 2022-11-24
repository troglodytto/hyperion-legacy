use crate::{HttpBody, HttpHeader, HttpVersion};
use chrono;
use std::fmt::Display;

#[derive(Debug, Clone)]
pub struct HttpResponse {
    status: u16,
    headers: Vec<HttpHeader>,
    version: HttpVersion,
    body: Option<HttpBody>,
}

impl HttpResponse {
    pub fn new(status: u16, body: Option<HttpBody>) -> HttpResponse {
        let mut headers = vec![
            HttpHeader {
                name: "Server".to_string(),
                value: "Hyperion".to_string(),
            },
            HttpHeader {
                name: "Date".to_string(),
                value: chrono::Utc::now().to_rfc3339(),
            },
        ];

        headers.push(HttpHeader {
            name: "Content-Length".to_string(),
            value: match &body {
                Some(body) => body.len().to_string(),
                None => "0".to_string(),
            },
        });

        HttpResponse {
            status,
            headers,
            version: HttpVersion::default(),
            body,
        }
    }

    fn get_status_text<'a>(status_code: u16) -> &'a str {
        match status_code {
            201 => "Created",
            200 => "OK",
            204 => "No Content",
            205 => "Reset Content",
            301 => "Moved Permanently",
            304 => "Not Modified",
            307 => "Temporary Redirect",
            400 => "Bad Request",
            401 => "Unauthorized",
            403 => "Forbidden",
            404 => "Not Found",
            405 => "Method Not Allowed",
            418 => "I'm a teapot",
            429 => "Too Many Requests ",
            501 => "Not Implemented",
            502 => "Bad Gateway",
            503 => "Service Unavailable",
            500 | _ => "Internal Server Error",
        }
    }

    pub fn as_bytes(&self) -> Vec<u8> {
        let mut response = Vec::from(format!("{self}").as_bytes());

        if let Some(body) = &self.body {
            response.extend(body.as_bytes());
        }

        response
    }

    pub fn add_header(&mut self, header: HttpHeader) {
        self.headers.push(header);
    }
}

impl Display for HttpResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "HTTP/{} {} {}\r\n",
            self.version,
            self.status,
            HttpResponse::get_status_text(self.status)
        )?;

        for header in &self.headers {
            write!(f, "{header}\r\n")?;
        }

        write!(f, "\r\n")?;

        Ok(())
    }
}
