use hyper::http::request;

#[derive(Debug)]
pub struct Request {
    pub url: String,
}

impl Into<Request> for hyper::Request<hyper::Body> {
    fn into(self) -> Request {
        let (parts, body) = self.into_parts();
        let uri = parts.uri;
        let path = uri.path();

        Request {
            url: path.to_string(),
        }
    }
}
