extern crate hyper;
extern crate url;
extern crate serde;

use std::io::{Read, Write};
use hyper::error::Error;
use hyper::client::Request;
use hyper::method::Method;
use hyper::net::Fresh;
use serde::json::{self, Value, from_value};
use serde::Deserialize;
pub use hyper::header::*;
pub use url::Url;


pub struct RestClient<'a> {
    url: Url,
    params: Option<Vec<(&'a str, &'a str)>>,
    body: Option<String>,
    headers: Option<Headers>,
}

/// Constructs a new RestClient.
impl<'a> RestClient<'a> {
    pub fn new(url: Url) -> RestClient<'a> {
        RestClient { url: url, params: None, body: None, headers: None }
    }

    pub fn param(&'a mut self, param: (&'a str, &'a str)) -> &'a mut RestClient<'a> {
        if let Some(ref mut p) = self.params {
            p.push(param);
        }
        else
        {
            let mut v = Vec::new();
            v.push(param);
            self.params = Some(v);
        }
        self
    }

    pub fn body(&'a mut self, body: String) -> &'a mut RestClient<'a> {
        self.body = Some(body);
        self
    }

    pub fn header<H: Header + HeaderFormat>(&'a mut self, header: H) -> &'a mut RestClient<'a> {
        if let Some(ref mut h) = self.headers {
            h.set(header);
        }
        else
        {
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

    pub fn get(&mut self) -> Result<String, hyper::Error> {
        let mut url = self.url.clone();

        if let Some(ref params) = self.params {
            url.set_query_from_pairs(params.into_iter().map(|&x| x));
        }

        let req = try!(Request::new(Method::Get, url));
        self.send_request(req)
    }

    pub fn get_json_as<T: Deserialize>(&mut self) -> Result<T, String> {
        let body = try!(self.get().map_err(|err| err.to_string()));
        let val: Value = try!(json::from_str(&*body).map_err(|err| err.to_string()));
        from_value(val).map_err(|err| err.to_string())
    }

    pub fn delete(&mut self) -> Result<String, Error> {
        let mut url = self.url.clone();

        if let Some(ref params) = self.params {
            url.set_query_from_pairs(params.into_iter().map(|&x| x));
        }

        let req = try!(Request::new(Method::Delete, url));
        self.send_request(req)
    }

    pub fn delete_json_as<T: Deserialize>(&mut self) -> Result<T, String> {
        let body = try!(self.delete().map_err(|err| err.to_string()));
        let val: Value = try!(json::from_str(&*body).map_err(|err| err.to_string()));
        from_value(val).map_err(|err| err.to_string())
    }

    pub fn post(&mut self) -> Result<String, Error> {
        let url = self.url.clone();

        if let Some(ref params) = self.params {
            self.body = Some(url::form_urlencoded::serialize(params.into_iter()));
        }

        let req = try!(Request::new(Method::Post, url));
        self.send_request(req)
    }

    pub fn post_json_as<T: Deserialize>(&mut self) -> Result<T, String> {
        let body = try!(self.post().map_err(|err| err.to_string()));
        let val: Value = try!(json::from_str(&*body).map_err(|err| err.to_string()));
        from_value(val).map_err(|err| err.to_string())
    }
    
    pub fn put(&mut self) -> Result<String, Error> {
        let url = self.url.clone();

        if let Some(ref params) = self.params {
            self.body = Some(url::form_urlencoded::serialize(params.into_iter()));
        }

        let req = try!(Request::new(Method::Put, url));
        self.send_request(req)
    }

    pub fn put_json_as<T: Deserialize>(&mut self) -> Result<T, String> {
        let body = try!(self.put().map_err(|err| err.to_string()));
        let val: Value = try!(json::from_str(&*body).map_err(|err| err.to_string()));
        from_value(val).map_err(|err| err.to_string())
    }
}
