use std::fmt::Debug;

use matchit::Node;

use crate::handler::CloneBoxHandler;
use crate::Handler;
use crate::Method;
use crate::Request;
use crate::Response;
use crate::Route;

#[derive(Clone)]

pub struct Router {
    matchit_node: Node<Route>,
    routes: Vec<Route>,
}

impl Default for Router {
    fn default() -> Self {
        Self {
            matchit_node: Node::new(),
            routes: Default::default(),
        }
    }
}

impl Debug for Router {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Router")
            // .field("matchit_node", &self.matchit_node)
            .field("routes", &self.routes)
            .finish()
    }
}

impl Router {
    pub fn get<T>(&mut self, url: &str, handler: T) -> &mut Self
    where
        T: Handler + Copy + Clone + Send + Sized + 'static,
    {
        self.insert(Method::Get, url, handler);
        self
    }

    pub fn post<T>(&mut self, url: &str, handler: T) -> &mut Self
    where
        T: Handler + Copy + Clone + Send + Sized + 'static,
    {
        self.insert(Method::Post, url, handler);
        self
    }

    pub fn put<T>(&mut self, url: &str, handler: T) -> &mut Self
    where
        T: Handler + Copy + Clone + Send + Sized + 'static,
    {
        self.insert(Method::Put, url, handler);
        self
    }

    pub fn delete<T>(&mut self, url: &str, handler: T) -> &mut Self
    where
        T: Handler + Copy + Clone + Send + Sized + 'static,
    {
        self.insert(Method::Delete, url, handler);
        self
    }

    pub fn nest(&mut self, url: &str, router: Router) -> &mut Self {
        // prepend "/" if it doesn't exist.
        let url = Router::pad_path(url.to_string());

        for mut route in router.routes {
            // If they will not cause a double // in between paths.
            if !(route.url.starts_with("/") && url.ends_with("/")) {
                let mut prefix = url.to_string();
                prefix.push_str(route.url.as_str());
                route.url = prefix;
            }
            self.add_route(route);
        }

        self
    }

    pub(crate) fn compile(&mut self) {
        for route in &self.routes {
            // TODO: handle this error
            let route = route.clone();
            let url = route.url.clone();
            match self.matchit_node.insert(url, route) {
                Ok(_) => (),
                Err(error) => panic!("TODO! handle this: {:?}", error),
            }
        }
    }

    fn insert<T>(&mut self, method: Method, url: &str, handler: T)
    where
        T: Handler + Copy + Clone + Send + Sized + 'static,
    {
        let route = Route {
            url: Router::pad_path(url.to_string()),
            method: method,
            handler: CloneBoxHandler::new(handler),
        };

        self.add_route(route);
    }

    fn add_route(&mut self, route: Route) {
        self.routes.push(route);
    }

    fn pad_path(url: String) -> String {
        if url.starts_with("/") {
            url
        } else {
            let mut path = String::from("/");
            path.push_str(url.as_str());
            path
        }
    }

    pub async fn handle(&self, request: Request) -> Response {
        let url = request.url.clone();
        println!("{:?}", request);
        // let url = request
        // TODO: handle this error
        match self.matchit_node.at(url.as_str()) {
            Ok(matchit_matched) => {
                // TODO: pull our route params and create a request model here.
                let route: &Route = matchit_matched.value;
                route.handler.0.call(request).await
            }
            Err(error) => panic!("TODO! handle this: {:?}", error),
        }
    }
}
