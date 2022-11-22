use crate::HttpError;
use std::fmt::Display;

#[derive(Debug, Clone, Copy, Default)]
pub(crate) enum HttpMethod {
    #[default]
    GET,
    POST,
    PUT,
    DELETE,
}

impl HttpMethod {
    pub fn new(method_string: &str) -> Result<HttpMethod, HttpError> {
        match method_string.to_uppercase().as_str() {
            "GET" => Ok(HttpMethod::GET),
            "POST" => Ok(HttpMethod::POST),
            "UPDATE" => Ok(HttpMethod::PUT),
            "DELETE" => Ok(HttpMethod::DELETE),
            _ => Err(HttpError::BadRequest {
                message: "Method Not allowed".to_string(),
            }),
        }
    }
}

impl Display for HttpMethod {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let method_string = match self {
            HttpMethod::GET => "GET",
            HttpMethod::POST => "POST",
            HttpMethod::PUT => "PUT",
            HttpMethod::DELETE => "DELETE",
        };

        write!(f, "{method_string}")
    }
}
