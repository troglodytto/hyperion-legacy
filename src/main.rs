use hyperion::{HttpBody, HttpHeader, HttpRequest, HttpResponse, HttpServer, HyperionConfig};
use std::io::{Read, Result};

fn read_file(path: &str, content_type: &str) -> HttpResponse {
    let mut response = HttpResponse::new(200, Some(HttpBody::new(std::fs::read(path).unwrap())));

    response.add_header(HttpHeader {
        name: "Content-Type".to_string(),
        value: content_type.to_string(),
    });

    response
}

async fn index_html(_req: HttpRequest) -> HttpResponse {
    read_file("index.html", "text/html")
}

async fn styles_css(_req: HttpRequest) -> HttpResponse {
    read_file("styles/main.css", "text/css")
}

#[tokio::main]
async fn main() -> Result<()> {
    let server = HttpServer::init().await;

    server.add_handler("/", index_html).await;
    server.add_handler("/styles/main.css", styles_css).await;

    server.listen().await;

    // @todo Add error handling to config parsing and shift it into HyperionConfig struct
    let config = HyperionConfig::new();
    std::fs::write("config.yml", serde_yaml::to_string(&config).unwrap()).unwrap();
    std::fs::File::open("config.yml").map(|mut file| {
        let mut file_contents = String::new();
        file.read_to_string(&mut file_contents);
        serde_yaml::from_str::<HyperionConfig>(&file_contents)
    })?;

    Ok(())
}
