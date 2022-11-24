use crate::{HttpRequest, HttpResponse};
use futures::{future::BoxFuture, Future};
use std::collections::HashMap;
use tokio::sync::RwLock;

struct RequestHandler {
    func: Box<dyn Fn(HttpRequest) -> BoxFuture<'static, HttpResponse> + Send + Sync + 'static>,
}

impl RequestHandler {
    pub fn new<P>(func: fn(HttpRequest) -> P) -> RequestHandler
    where
        P: Future<Output = HttpResponse> + Send + 'static,
    {
        RequestHandler {
            func: Box::new(move |request| Box::pin(func(request))),
        }
    }

    pub async fn call(&self, request: HttpRequest) -> HttpResponse {
        (self.func)(request).await
    }
}

pub struct HttpRouter {
    handlers: HashMap<String, RequestHandler>,
}

impl HttpRouter {
    pub fn new() -> HttpRouter {
        HttpRouter {
            handlers: HashMap::new(),
        }
    }

    pub fn add_handler<P>(&mut self, path: &str, fun: fn(HttpRequest) -> P)
    where
        P: Future<Output = HttpResponse> + Send + 'static,
    {
        self.handlers
            .insert(path.to_string(), RequestHandler::new(fun));
    }

    pub async fn process_request(&self, request: HttpRequest) -> HttpResponse {
        let path = request.path.clone();

        if let Some(handler) = self.handlers.get(&path) {
            handler.call(request).await
        } else {
            HttpResponse::new(404, None)
        }
    }
}

lazy_static::lazy_static! {
    pub static ref ROUTER: RwLock<HttpRouter> = RwLock::new(
        HttpRouter::new()
    );
}
