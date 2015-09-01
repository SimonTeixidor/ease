extern crate hyper;
extern crate url;
extern crate rustc_serialize;

use std::io::{Read, Write};
use hyper::client::Request;
use hyper::method::Method;
use hyper::net::Fresh;
use rustc_serialize::{Decodable, json};

#[doc(no_inline)]
pub use hyper::header::*;
#[doc(no_inline)]
pub use url::Url;
#[doc(no_inline)]
pub use hyper::error::Error;

#[derive(Clone)]
pub struct RestClient<'a> {
    url: Url,
    params: Option<Vec<(&'a str, &'a str)>>,
    body: Option<String>,
    headers: Option<Headers>,
}

impl<'a> RestClient<'a> {
    /// Constructs a new RestClient.
    pub fn new(url: Url) -> RestClient<'a> {
        RestClient { url: url, params: None, body: None, headers: None }
    }

    /// Sets one parameter. On a GET or DELETE request, this parameter will
    /// be stored in the URL. On a POST or PUT request, it is stored in the
    /// body of the request. Hence, if you call this method on a POST or
    /// PUT request, you cannot also call `body`.
    pub fn param(&'a mut self, key: &'a str, value: &'a str) -> &'a mut RestClient<'a> {
        if let Some(ref mut p) = self.params {
            p.push((key, value));
        } else {
            let mut v = Vec::new();
            v.push((key, value));
            self.params = Some(v);
        }
        self
    }

    /// Sets many parameters. On a GET or DELETE request, these parameters will
    /// be stored in the URL. On a POST or PUT request, they are stored in the
    /// body of the request. Hence, if you call this method on a POST or
    /// PUT request, you cannot also call `body`.
    pub fn params<T>(&'a mut self, values: T) -> &'a mut RestClient<'a>
        where T: IntoIterator<Item = (&'a str, &'a str)>
    {
        if let Some(ref mut p) = self.params {
            for value in values {
                p.push(value);
            }
        } else {
            let mut v = Vec::new();
            for value in values {
                v.push(value);
            }
            self.params = Some(v);
        }
        self
    }

    /// Writes a `String` to the body of the request. Don't call this
    /// method if you also call `param` on a PUT or POST request.
    pub fn body(&'a mut self, body: String) -> &'a mut RestClient<'a> {
        self.body = Some(body);
        self
    }

    /// Sets a header for the request.
    pub fn header<H: Header + HeaderFormat>(&'a mut self, header: H) -> &'a mut RestClient<'a> {
        if let Some(ref mut h) = self.headers {
            h.set(header);
        } else {
            let mut v = Headers::new();
            v.set(header);
            self.headers = Some(v);
        }
        self
    }

    fn send_request(&mut self, mut req: Request<Fresh>) -> Result<String, Error> {
        if let Some(headers) = self.headers.as_ref() {
            req.headers_mut().extend(headers.iter());
        }

        let mut req = try!(req.start());

        if let Some(body) = self.body.as_ref() {
            try!(req.write_all(body.as_bytes()));
        }

        let mut resp = try!(req.send());

        let mut response_string = String::new();
        try!(resp.read_to_string(&mut response_string));
        Ok(response_string)
    }

    /// Sends a GET request and returns either an error
    /// or a `String` of the response.
    pub fn get(&mut self) -> Result<String, hyper::Error> {
        let mut url = self.url.clone();

        if let Some(ref params) = self.params {
            url.set_query_from_pairs(params.into_iter().map(|&x| x));
        }

        let req = try!(Request::new(Method::Get, url));
        self.send_request(req)
    }

    /// Sends a GET request and returns either an error
    /// or a `T` representing the response, deserialised from JSON.
    pub fn get_json_as<T: Decodable>(&mut self) -> Result<T, String> {
        let body = try!(self.get().map_err(|err| err.to_string()));
        json::decode(&*body).map_err(|err| format!("{}. Server response: {}", err.to_string(), body))
    }

    /// Sends a DELETE request and returns either an error
    /// or a `String` of the response.
    pub fn delete(&mut self) -> Result<String, Error> {
        let mut url = self.url.clone();

        if let Some(ref params) = self.params {
            url.set_query_from_pairs(params.into_iter().map(|&x| x));
        }

        let req = try!(Request::new(Method::Delete, url));
        self.send_request(req)
    }

    /// Sends a DELETE request and returns either an error
    /// or a `T` representing the response, deserialised from JSON.
    pub fn delete_json_as<T: Decodable>(&mut self) -> Result<T, String> {
        let body = try!(self.delete().map_err(|err| err.to_string()));
        json::decode(&*body).map_err(|err| format!("{}. Server response: {}", err.to_string(), body))
    }

    /// Sends a POST request and returns either an error
    /// or a `String` of the response.
    pub fn post(&mut self) -> Result<String, Error> {
        let url = self.url.clone();

        if let Some(ref params) = self.params {
            self.body = Some(url::form_urlencoded::serialize(params.into_iter()));
        }

        let req = try!(Request::new(Method::Post, url));
        self.send_request(req)
    }

    /// Sends a POST request and returns either an error
    /// or a `T` representing the response, deserialised from JSON.
    pub fn post_json_as<T: Decodable>(&mut self) -> Result<T, String> {
        let body = try!(self.post().map_err(|err| err.to_string()));
        json::decode(&*body).map_err(|err| format!("{}. Server response: {}", err.to_string(), body))
    }

    /// Sends a PUT request and returns either an error
    /// or a `String` of the response.
    pub fn put(&mut self) -> Result<String, Error> {
        let url = self.url.clone();

        if let Some(ref params) = self.params {
            self.body = Some(url::form_urlencoded::serialize(params.into_iter()));
        }

        let req = try!(Request::new(Method::Put, url));
        self.send_request(req)
    }

    /// Sends a PUT request and returns either an error
    /// or a `T` representing the response, deserialised from JSON.
    pub fn put_json_as<T: Decodable>(&mut self) -> Result<T, String> {
        let body = try!(self.put().map_err(|err| err.to_string()));
        json::decode(&*body).map_err(|err| format!("{}. Server response: {}", err.to_string(), body))
    }
}
