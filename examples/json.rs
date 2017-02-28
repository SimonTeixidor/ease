#[macro_use]
extern crate serde_derive;

extern crate ease;

use std::collections::HashMap;
use ease::{Url, Request};

#[derive(Deserialize, Debug)]
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
    println!("{:#?}",
             Request::new(url).post().and_then(|res| res.from_json::<Response>()));

    let url = Url::parse("http://httpbin.org/get").unwrap();
    println!("{:#?}",
             Request::new(url).get().and_then(|res| res.from_json::<Response>()));
}
