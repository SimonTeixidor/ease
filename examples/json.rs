extern crate ease;
extern crate rustc_serialize;

use std::collections::HashMap;
use ease::{Url, Request};

#[derive(RustcDecodable, Debug)]
struct Response {
    args: HashMap<String, String>,
    data: Option<String>,
    files: Option<HashMap<String, String>>,
    form: Option<HashMap<String, String>>,
    headers: HashMap<String, String>,
    json: Option<String>,
    origin: String,
    url: String,
}

fn main() {
    let url = Url::parse("http://httpbin.org/post").unwrap();
    println!("{:#?}", Request::new(url).post().unwrap().json_as::<Response>());

    let url = Url::parse("http://httpbin.org/get").unwrap();
    println!("{:#?}", Request::new(url).get().unwrap().json_as::<Response>());
}
