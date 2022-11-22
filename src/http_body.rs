#[derive(Debug, Clone)]
pub struct HttpBody {
    bytes: Vec<u8>,
}

impl HttpBody {
    pub fn new(bytes: Vec<u8>) -> HttpBody {
        HttpBody { bytes }
    }

    pub fn as_bytes(&self) -> Vec<u8> {
        self.bytes.clone()
    }

    pub fn len(&self) -> usize {
        self.bytes.len()
    }
}
