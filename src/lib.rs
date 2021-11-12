mod handler;
mod request;
mod response;
mod route;
mod router;

pub(crate) use handler::Handler;
pub use request::Request;
pub use response::Response;
pub use route::{Method, Route};
pub use router::Router;

async fn hello_world(request: Request) -> Response {
    println!("hello_world1");
    Response {}
}

async fn hello_world2(request: Request) -> Response {
    println!("hello_world2");
    Response {}
}

pub async fn test() {
    let mut router = Router::default();

    router.get("test", hello_world).get("test2", hello_world2);

    router.handle("test2").await;
}
