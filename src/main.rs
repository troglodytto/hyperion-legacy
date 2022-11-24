use hyperion::{HttpHeader, HttpRequest, HttpResponse, HttpServer};

async fn handle_hello(req: HttpRequest) -> HttpResponse {
    let mut response = HttpResponse::new(401, req.body);

    response.add_header(HttpHeader {
        name: "Content-Type".to_string(),
        value: "application/json".to_string(),
    });

    response
}

#[tokio::main]
async fn main() {
    let server = HttpServer::init().await;

    server
        .add_handler("/path", |req| async move {
            return HttpResponse::new(301, None);
        })
        .await;
    server.add_handler("/path2", handle_hello).await;

    server.listen().await;
}
