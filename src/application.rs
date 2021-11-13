use hyper::server::conn::AddrStream;
use hyper::service::{make_service_fn, service_fn};
use hyper::{Body, Server};
use std::convert::Infallible;
use std::net::SocketAddr;
use std::sync::{Arc, Mutex};

use crate::{application, Handler, Method, Request, Router};

#[derive(Clone)]
pub struct Application {
    router: Router,
}

impl Default for Application {
    fn default() -> Self {
        Self {
            router: Default::default(),
        }
    }
}

impl Application {
    pub fn get<T>(&mut self, url: &str, handler: T) -> &mut Self
    where
        T: Handler + Copy + Clone + Send + Sized + 'static,
    {
        self.router.get(url, handler);
        self
    }

    pub fn post<T>(&mut self, url: &str, handler: T) -> &mut Self
    where
        T: Handler + Copy + Clone + Send + Sized + 'static,
    {
        self.router.post(url, handler);
        self
    }

    pub fn put<T>(&mut self, url: &str, handler: T) -> &mut Self
    where
        T: Handler + Copy + Clone + Send + Sized + 'static,
    {
        self.router.put(url, handler);
        self
    }

    pub fn delete<T>(&mut self, url: &str, handler: T) -> &mut Self
    where
        T: Handler + Copy + Clone + Send + Sized + 'static,
    {
        self.router.delete(url, handler);
        self
    }

    pub async fn start(self) {
        // We'll bind to 127.0.0.1:3000
        let addr = SocketAddr::from(([127, 0, 0, 1], 3000));

        // let route_handler = self.handle_route;

        // A `Service` is needed for every connection, so this
        // creates one from our `hello_world` function.
        // let make_svc = make_service_fn(|_conn| async {
        //     let application = self.clone();
        //     // service_fn converts our function into a `Service`
        //     Ok::<_, Infallible>(service_fn(move |request| {
        //         application.handle_route(request)
        //         // Clone again to ensure that client outlives this closure.
        //         // response_examples(req, client.to_owned())
        //     }))
        // });
        let application = Arc::new(self);
        // let counter = Arc::new(Mutex::new(0));

        // A `MakeService` that produces a `Service` to handle each connection.
        let make_service = make_service_fn(move |conn: &AddrStream| {
            // We have to clone the context to share it with each invocation of
            // `make_service`. If your data doesn't implement `Clone` consider using
            // an `std::sync::Arc`.
            // let application = Arc::new(self);
            let application = application.clone();
            // let counter = counter.clone();

            // You can grab the address of the incoming connection like so.
            // let addr = conn.remote_addr();

            // Create a `Service` for responding to the request.
            let service = service_fn(move |request| handle_route(application.clone(), request));

            // Return the service to hyper.
            async move { Ok::<_, Infallible>(service) }
        });

        let server = Server::bind(&addr).serve(make_service);

        // Run this server for... forever!
        println!("starting server on: {:?}", addr);
        if let Err(e) = server.await {
            eprintln!("server error: {}", e);
        }
    }
}

pub async fn handle_route(
    application: Arc<Application>,
    request: hyper::Request<hyper::Body>,
) -> Result<hyper::Response<Body>, Infallible> {
    // let mut data = counter.lock().unwrap();
    // *data += 1;
    // Ok(hyper::Response::new(Body::from(format!(
    // "Counter: {}\n",
    // data
    // ))))
    let request: Request = request.into();
    // let request: Request = Request::from_hyper(request);
    // let request = Request {
    // url: request.uri().path(),
    // };

    let route = application.router.handle(request).await;

    Ok(route.into())
}
