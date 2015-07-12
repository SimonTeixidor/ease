extern crate hyper;
extern crate url;

use std::io::{Read, Write};
use url::{Url, ParseError};
use hyper::error::Error;
use hyper::client::Request;
use hyper::method::Method;
use hyper::net::Fresh;
pub use hyper::header::*;

pub struct Client<'a, H: Header + HeaderFormat> {
    url: Url,
    params: Option<Vec<(&'a str, &'a str)>>,
    body: Option<String>,
    headers: Option<Vec<H>>,
}

impl<'a, H: Header + HeaderFormat> Client<'a, H> {
    pub fn new(url_str: &'a str) -> Result<Client<'a, H>, ParseError> {
        let url = try!(Url::parse(url_str));
        Ok(Client { url: url, params: None, body: None, headers: None })
    }

    pub fn param(&'a mut self, param: (&'a str, &'a str)) -> &'a mut Client<'a, H> {
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

    pub fn body(&'a mut self, body: String) -> &'a mut Client<'a, H> {
        self.body = Some(body);
        self
    }

    pub fn header(&'a mut self, header: H) -> &'a mut Client<'a, H> {
        if let Some(ref mut h) = self.headers {
            h.push(header);
        }
        else
        {
            let mut v = Vec::new();
            v.push(header);
            self.headers = Some(v);
        }
        self
    }

    fn send_request(&mut self, mut req: Request<Fresh>) -> Result<String, Error> {
        if let Some(headers) = self.headers.as_ref() {
            for header in headers {
                req.headers_mut().set(header.clone());
           }
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

    pub fn get(&mut self) -> Result<String, Error> {
        let mut url = self.url.clone();

        if let Some(ref params) = self.params {
            url.set_query_from_pairs(params.into_iter().map(|&x| x));
        }

        let req = try!(Request::new(Method::Get, url));
        self.send_request(req)
    }

    pub fn post(&mut self) -> Result<String, Error> {
        let url = self.url.clone();

        if let Some(ref params) = self.params {
            self.body = Some(url::form_urlencoded::serialize(params.into_iter()));
        }

        let req = try!(Request::new(Method::Post, url));
        self.send_request(req)
    }
}
