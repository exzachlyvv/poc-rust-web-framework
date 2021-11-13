use hyper::Body;

#[derive(Debug)]
pub struct Response {
    /// The response's status
    pub status: hyper::StatusCode,

    /// The response's version
    pub version: hyper::Version,

    /// The response's headers
    // pub headers: hyper::HeaderMap<hyper::HeaderValue>,
    body: ResponseBody,
}

impl Into<hyper::Response<hyper::Body>> for Response {
    fn into(self) -> hyper::Response<hyper::Body> {
        hyper::Response::builder()
            .status(self.status)
            .version(self.version)
            // TODO: headers.
            .body(self.body.to_body())
            .unwrap() // TODO: DON'T USER UNWRAP!!!
    }
}

impl Response {
    pub fn text(text: &'static str) -> Self {
        Self {
            status: hyper::StatusCode::OK,
            version: hyper::Version::HTTP_11,
            body: ResponseBody::Text(text),
        }
    }

    pub fn html(html: &'static str) -> Self {
        Self {
            status: hyper::StatusCode::OK,
            version: hyper::Version::HTTP_11,
            body: ResponseBody::Html(html),
        }
    }

    pub fn json(json: &'static str) -> Self {
        Self {
            status: hyper::StatusCode::OK,
            version: hyper::Version::HTTP_11,
            body: ResponseBody::Json(json),
        }
    }
}

#[derive(Debug)]
enum ResponseBody {
    Text(&'static str),
    Html(&'static str),
    Json(&'static str),
}

impl ResponseBody {
    fn to_body(&self) -> hyper::Body {
        match *self {
            ResponseBody::Text(text) => Body::from(text),
            ResponseBody::Html(html) => Body::from(html),
            ResponseBody::Json(json) => Body::from(json),
        }
    }
}
