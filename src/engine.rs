use hyper::{Client, Response};
use tokio::runtime::Runtime;
use hyper::Error;
use std::convert::TryInto;
use hyper::client::HttpConnector;
use hyper::body::Body;
use std::borrow::Borrow;
use hyper::http::response::Parts;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;

pub fn new_engine() -> Engine {
    Engine {
        runtime: tokio::runtime::Builder::new_multi_thread()
            .enable_all()
            .build().unwrap(),
    }
}

async fn req_async_body_string(success: fn(Parts, String), error: fn(Error)) {
    let client: Client<HttpConnector, Body> = Client::builder().build_http();
    let uri = "http://httpbin.org/ip".parse().unwrap();
    match client.get(uri).await {
        Ok(ok) => {
            let parts = ok.into_parts();
            match hyper::body::to_bytes(parts.1).await {
                Ok(bytes) => {
                    success(parts.0, String::from_utf8(bytes.to_vec()).unwrap());
                }
                Err(err) => {
                    error(err);
                }
            };
        }
        Err(err) => {
            error(err)
        }
    };
}

pub struct Engine {
    runtime: Runtime,
}

impl Engine {
    pub fn request(&self, success: fn(Parts, String), error: fn(Error)) {
        self.runtime.spawn(req_async_body_string(success, error));
    }
}