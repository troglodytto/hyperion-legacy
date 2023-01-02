use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
enum ServerKind {
    Files {
        /// Look for files in this directory when serving
        file_root: String,
        /// Look for these files when serving the index page
        index: Vec<String>,
    },
    Proxy {
        pass: String,
    },
}

#[derive(Debug, Serialize, Deserialize)]
struct ServerConfig {
    location: String,
    port: u16,
    server_kind: ServerKind,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct HyperionConfig {
    servers: Vec<ServerConfig>,
}

impl HyperionConfig {
    pub fn new() -> HyperionConfig {
        HyperionConfig {
            servers: vec![
                ServerConfig {
                    location: "/".to_string(),
                    port: 82,
                    server_kind: ServerKind::Files {
                        file_root: "/var/www/html".to_string(),
                        index: vec!["index".to_string(), "index.html".to_string()],
                    },
                },
                ServerConfig {
                    location: "/".to_string(),
                    port: 83,
                    server_kind: ServerKind::Proxy {
                        pass: "localhost:80".to_string(),
                    },
                },
            ],
        }
    }
}
