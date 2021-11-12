use matchit::Node;

use crate::Handler;
use crate::Method;
use crate::Request;
use crate::Route;

pub struct Router {
    matchit_node: Node<Route>,
    // routes: Vec<Route>,
}

impl Default for Router {
    fn default() -> Self {
        Self {
            matchit_node: Node::new(),
            // routes: Default::default(),
        }
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

    fn insert<T>(&mut self, method: Method, url: &str, handler: T)
    where
        T: Handler + Copy + Clone + Send + Sized + 'static,
    {
        let route = Route {
            method: method,
            handler: Box::new(handler),
        };

        // TODO: handle this error
        match self.matchit_node.insert(url, route) {
            Ok(_) => (),
            Err(error) => panic!("TODO! handle this: {:?}", error),
        }
    }

    pub async fn handle(&self, url: &str) {
        // TODO: handle this error
        match self.matchit_node.at(url) {
            Ok(matchit_matched) => {
                // TODO: pull our route params and create a request model here.
                let route: &Route = matchit_matched.value;
                let response = route.handler.call(Request {}).await;
                println!("{:?}", response);
            }
            Err(error) => panic!("TODO! handle this: {:?}", error),
        }
    }
}
