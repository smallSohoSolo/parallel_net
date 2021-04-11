use hyper::{Body, Client};
use hyper::client::HttpConnector;
use phf::phf_map;
use std::collections::HashMap;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;

pub fn newEngine(id: String) -> Engine {
    Engine {
        isRunning: false,
        isCanceled: false,
        id,
    }
}

pub struct Engine {
    isRunning: bool,
    isCanceled: bool,
    /// 平台方传过来的唯一id
    id: String,
}

pub fn request(id: String) {
    let mut engine = newEngine(id);
    engine.isRunning = true;
    log::debug!("我真的跪了");
}