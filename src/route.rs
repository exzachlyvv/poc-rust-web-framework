use crate::handler::CloneBoxHandler;

#[derive(Clone)]
pub struct Route {
    pub method: Method,
    pub(crate) handler: CloneBoxHandler,
}

impl Route {}

#[derive(Clone)]
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
