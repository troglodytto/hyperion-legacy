use crate::HttpHeader;
use chrono::{DateTime, Utc};
use std::fmt::Display;

#[derive(Debug, Clone)]
pub struct HttpCookie {
    pub name: String,
    value: String,
    secure: bool,
    domain: Option<String>,
    path: Option<String>,
    expires: Option<DateTime<Utc>>,
    same_site: Option<String>,
}

impl HttpCookie {
    pub fn to_header(&self) -> HttpHeader {
        let attributes: Vec<_> = self
            .get_attributes()
            .iter()
            .map(|attribute| format!("{}={}", attribute.0, attribute.1))
            .collect();

        HttpHeader {
            name: "Cookie".to_string(),
            value: format!("{}={}; {}", self.name, self.value, attributes.join("; ")),
        }
    }

    pub fn from_header(header: &HttpHeader) -> HttpCookie {
        HttpCookie {
            name: "Cookie".to_string(),
            value: "".to_string(),
            secure: false,
            domain: None,
            path: None,
            expires: None,
            same_site: None,
        }
    }

    pub fn get_attributes(&self) -> Vec<(String, String)> {
        let mut attributes = vec![];

        if self.secure {
            attributes.push(("Secure".to_string(), "".to_string()));
        }

        if let Some(same_site) = &self.same_site {
            attributes.push(("Same-Site".to_string(), same_site.to_string()));
        }

        if let Some(domain) = &self.domain {
            attributes.push(("Domain".to_string(), domain.to_string()));
        }

        if let Some(path) = &self.path {
            attributes.push(("Path".to_string(), path.to_string()));
        }

        if let Some(expires) = &self.expires {
            attributes.push(("Expires".to_string(), expires.format("%+").to_string()));
        }

        attributes
    }
}

impl Display for HttpCookie {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_header())
    }
}
