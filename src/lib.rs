extern crate hyper;
extern crate url;
extern crate serde;
extern crate serde_json;

use std::error::Error as StdError;
use std::fmt::{self, Display, Formatter};
use std::io::{Read, Write};
use std::io::Error as IoError;
use std::time::Duration;

use serde::de::Deserialize;
use hyper::client::Request as HyperRequest;
use hyper::client::Response as HyperResponse;
use hyper::method::Method;
use hyper::net::Fresh;
use hyper::status::StatusClass::{Informational, Success, Redirection};
use hyper::header::Cookie;

#[doc(no_inline)]
pub use hyper::header;
#[doc(no_inline)]
pub use url::Url;
#[doc(no_inline)]
pub use hyper::error::Error as HyperError;
#[doc(no_inline)]
pub use hyper::status::StatusCode;

#[derive(Debug)]
pub enum Error {
    UnsuccessfulResponse(Response),
    Json(serde_json::error::Error),
    Hyper(HyperError),
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match *self {
            Error::UnsuccessfulResponse(ref rsp) => write!(f, "server response failed: {:?}", rsp),
            Error::Json(ref error) => write!(f, "json parse error: {}", error),
            Error::Hyper(ref error) => write!(f, "HTTP communication error: {}", error),
        }
    }
}

impl StdError for Error {
    fn description(&self) -> &str {
        "ease error"
    }

    fn cause(&self) -> Option<&StdError> {
        match *self {
            Error::Json(ref error) => Some(error),
            Error::Hyper(ref error) => Some(error),
            _ => None,
        }
    }
}

impl From<HyperError> for Error {
    fn from(h: HyperError) -> Error {
        Error::Hyper(h)
    }
}

impl From<IoError> for Error {
    fn from(i: IoError) -> Error {
        Error::Hyper(HyperError::Io(i))
    }
}

#[derive(Debug)]
pub struct Response {
    pub hyper_response: HyperResponse,
    pub body: String,
}

impl Response {
    fn from_hyper_response(mut hyper_response: HyperResponse) -> Result<Response, IoError> {
        let mut body = String::new();
        hyper_response.read_to_string(&mut body).map(|_| {
            Response {
                hyper_response: hyper_response,
                body: body,
            }
        })
    }

    /// Deserializes the body of the response from JSON into
    /// a `T`.
    pub fn from_json<T: Deserialize>(&self) -> Result<T, Error> {
        serde_json::from_str(&*self.body).map_err(|e| Error::Json(e))
    }

    pub fn get_cookies(&self) -> Vec<String> {
        self.hyper_response
            .headers
            .get_raw("Set-Cookie")
            .unwrap()
            .to_vec()
            .iter()
            .map(|v| {
                v.iter()
                    .map(|&c| c as char)
                    .collect::<String>()
                    .split(";")
                    .next()
                    .unwrap()
                    .to_owned()
            })
            .collect::<Vec<_>>()
    }
}

#[derive(Clone)]
pub struct Request {
    url: Url,
    params: Option<Vec<(String, String)>>,
    body: Option<String>,
    read_timeout: Option<Duration>,
    headers: Option<header::Headers>,
}


impl Request {
    pub fn new(url: Url) -> Request {
        Request {
            url: url,
            params: None,
            body: None,
            read_timeout: None,
            headers: None,
        }
    }

    /// Sets one parameter. On a GET or DELETE request, this parameter will
    /// be stored in the URL. On a POST or PUT request, it is stored in the
    /// body of the request. Hence, if you call this method on a POST or
    /// PUT request, you cannot also call `body`.
    pub fn param<S>(&mut self, key: S, value: S) -> &mut Request
        where S: Into<String>
    {
        if let Some(ref mut p) = self.params {
            p.push((key.into(), value.into()));
        } else {
            let mut v = Vec::new();
            v.push((key.into(), value.into()));
            self.params = Some(v);
        }
        self
    }

    /// Sets many parameters. On a GET or DELETE request, these parameters will
    /// be stored in the URL. On a POST or PUT request, they are stored in the
    /// body of the request. Hence, if you call this method on a POST or
    /// PUT request, you cannot also call `body`.
    pub fn params<S, T>(&mut self, values: T) -> &mut Request
        where S: Into<String>,
              T: IntoIterator<Item = (S, S)>
    {
        if let Some(ref mut p) = self.params {
            for value in values {
                p.push((value.0.into(), value.1.into()));
            }
        } else {
            let mut v = Vec::new();
            for value in values {
                v.push((value.0.into(), value.1.into()));
            }
            self.params = Some(v);
        }
        self
    }

    /// Writes a `String` to the body of the request. Don't call this
    /// method if you also call `param` on a PUT or POST request.
    pub fn body(&mut self, body: String) -> &mut Request {
        self.body = Some(body);
        self
    }

    /// Sets a header for the request.
    pub fn header<H: header::Header + header::HeaderFormat>(&mut self, header: H) -> &mut Request {
        if let Some(ref mut h) = self.headers {
            h.set(header);
        } else {
            let mut v = header::Headers::new();
            v.set(header);
            self.headers = Some(v);
        }
        self
    }

    pub fn cookies(&mut self, cookies: Vec<String>) -> &mut Request {
        self.header(Cookie(cookies))
    }

    /// Sets a read timeout for the response.
    pub fn read_timeout(&mut self, timeout: Duration) -> &mut Request {
        self.read_timeout = Some(timeout);
        self
    }

    fn send_request(&mut self, mut req: HyperRequest<Fresh>) -> Result<Response, Error> {
        if let Some(headers) = self.headers.as_ref() {
            req.headers_mut().extend(headers.iter());
        }

        let _ = req.set_read_timeout(self.read_timeout);

        let mut req = try!(req.start());

        if let Some(body) = self.body.as_ref() {
            try!(req.write_all(body.as_bytes()));
        }

        let resp = try!(req.send());
        let resp = try!(Response::from_hyper_response(resp));

        match resp.hyper_response.status.class() {
            Informational => Ok(resp),
            Success => Ok(resp),
            Redirection => Ok(resp),
            _ => Err(Error::UnsuccessfulResponse(resp)),
        }
    }

    /// Sends a GET request and returns either an error
    /// or a `String` of the response.
    pub fn get(&mut self) -> Result<Response, Error> {
        let mut url = self.url.clone();

        if let Some(ref params) = self.params {
            url.query_pairs_mut().extend_pairs(params);
        }

        let req = try!(HyperRequest::new(Method::Get, url));
        self.send_request(req)
    }

    /// Sends a DELETE request and returns either an error
    /// or a `String` of the response.
    pub fn delete(&mut self) -> Result<Response, Error> {
        let mut url = self.url.clone();

        if let Some(ref params) = self.params {
            url.query_pairs_mut().extend_pairs(params);
        }

        let req = try!(HyperRequest::new(Method::Delete, url));
        self.send_request(req)
    }

    /// Sends a POST request and returns either an error
    /// or a `String` of the response.
    pub fn post(&mut self) -> Result<Response, Error> {
        let url = self.url.clone();

        if let Some(ref params) = self.params {
            let mut serializer = url::form_urlencoded::Serializer::new(String::new());
            serializer.extend_pairs(params);
            self.body = Some(serializer.finish());
        }

        let req = try!(HyperRequest::new(Method::Post, url));
        self.send_request(req)
    }

    /// Sends a PUT request and returns either an error
    /// or a `String` of the response.
    pub fn put(&mut self) -> Result<Response, Error> {
        let url = self.url.clone();

        if let Some(ref params) = self.params {
            let mut serializer = url::form_urlencoded::Serializer::new(String::new());
            serializer.extend_pairs(params);
            self.body = Some(serializer.finish());
        }

        let req = try!(HyperRequest::new(Method::Put, url));
        self.send_request(req)
    }
}
