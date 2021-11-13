use std::future::Future;

use async_trait::async_trait;

use crate::{Request, Response};

#[async_trait]
pub trait Handler: Send + Sync + 'static {
    async fn call(&self, request: Request) -> Response;
}

#[async_trait]
impl<F, Fut> Handler for F
where
    F: FnOnce(Request) -> Fut + Clone + Send + Sized + Sync + Copy + 'static,
    Fut: Future<Output = Response> + Send,
{
    async fn call(&self, request: Request) -> Response {
        self(request).await
    }
}

pub(crate) struct CloneBoxHandler(pub Box<dyn CloneHandler + Send>);

impl CloneBoxHandler {
    pub(crate) fn new<H>(inner: H) -> Self
    where
        H: Handler + Clone + Send + 'static,
        // S::Future: Send + 'static,
    {
        // let inner = inner.map_future(|f| Box::pin(f) as _);
        CloneBoxHandler(Box::new(inner))
    }
}

impl Clone for CloneBoxHandler {
    fn clone(&self) -> Self {
        Self(self.0.clone_box())
    }
}

pub(crate) trait CloneHandler: Handler {
    fn clone_box(&self) -> Box<dyn CloneHandler + Send>;
}

impl<T> CloneHandler for T
where
    T: Handler + Send + Clone + 'static,
{
    fn clone_box(&self) -> Box<dyn CloneHandler + Send> {
        Box::new(self.clone())
    }
}
