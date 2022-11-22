use std::fmt::Display;

#[derive(Debug, Clone)]
pub struct HttpHeader {
    pub name: String,
    pub value: String,
}

impl HttpHeader {
    pub fn new(name: String, value: String) -> HttpHeader {
        HttpHeader { name, value }
    }

    pub fn from(header_line: &str) -> Option<HttpHeader> {
        let mut header = header_line.split(": ");

        let header_name = header.next();
        let header_value = header.next();

        match (header_name, header_value) {
            (Some(name), Some(value)) => Some(HttpHeader {
                name: name.to_string(),
                value: value.to_string(),
            }),
            _ => None,
        }
    }
}

impl Display for HttpHeader {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}: {}", self.name, self.value)
    }
}
