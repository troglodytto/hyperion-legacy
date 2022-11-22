use std::fmt::Display;
use std::io;
use thiserror::Error;

use crate::HttpResponse;

#[derive(Error, Debug)]
pub enum HttpError {
    BadRequest { message: String },
    IoError(#[from] io::Error),
}

impl HttpError {
    pub fn as_response(&self) -> HttpResponse {
        match self {
            HttpError::BadRequest { message } => HttpResponse::new(400, None, None),
            HttpError::IoError(_) => HttpResponse::new(500, None, None),
        }
    }

    fn get_status_code(&self) -> u16 {
        match self {
            HttpError::BadRequest { message: _ } => 400,
            HttpError::IoError(_) => 500,
        }
    }
}

impl Display for HttpError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            HttpError::BadRequest { message } => write!(f, "{message}"),
            HttpError::IoError(error) => write!(f, "{error}"),
        }
    }
}
