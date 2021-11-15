use std::fmt::Debug;

use crate::handler::CloneBoxHandler;

#[derive(Clone)]
pub struct Route {
    pub url: String,
    pub method: Method,
    pub(crate) handler: CloneBoxHandler,
}

impl Route {}

impl Debug for Route {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Route")
            .field("url", &self.url)
            .field("method", &self.method)
            // .field("handler", &self.handler)
            .finish()
    }
}

#[derive(Debug, Clone)]
pub enum Method {
    Options,
    Get,
    Post,
    Put,
    Delete,
    Head,
    Trace,
    Connect,
    Patch,
}
