extern crate ease;
extern crate serde;
extern crate serde_json;

use std::collections::HashMap;
use ease::{Url, Request};

include!(concat!(env!("OUT_DIR"),"/json.rs"));

fn main() {
    let url = Url::parse("http://httpbin.org/post").unwrap();
    println!("{:#?}", Request::new(url).post().and_then(|res| res.json_as::<Response>()));

    let url = Url::parse("http://httpbin.org/get").unwrap();
    println!("{:#?}", Request::new(url).get().and_then(|res| res.json_as::<Response>()));
}
