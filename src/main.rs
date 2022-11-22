use hyperion::HttpServer;

#[tokio::main]
async fn main() {
    let server = HttpServer::init().await;

    server.listen().await;
}
