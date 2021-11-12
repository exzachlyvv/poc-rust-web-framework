use crate::Handler;

pub struct Route {
    pub method: Method,
    pub handler: Box<dyn Handler>,
}

impl Route {
    pub fn get<T>(handler: T) -> Self
    where
        T: Handler + Copy + Clone + Send + Sized + 'static,
    {
        Route {
            method: Method::Get,
            handler: Box::new(handler),
        }
    }
}

#[derive(Clone)]
pub enum Method {
    Get,
    Post,
    Put,
    Delete,
}
