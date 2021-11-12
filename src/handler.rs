use std::future::Future;

use async_trait::async_trait;

use crate::{Request, Response};

#[async_trait]
pub trait Handler {
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
