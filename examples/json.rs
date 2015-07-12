#![feature(custom_derive, plugin)]
#![plugin(serde_macros)]

extern crate serde;
extern crate ease;

use std::collections::HashMap;
use ease::{Url, RestClient};

#[derive(Deserialize, Debug)]
struct Response {
    args: HashMap<String, String>,
    data: Option<String>,
    files: HashMap<String, String>,
    form: HashMap<String, String>,
    headers: HashMap<String, String>,
    json: Option<String>,
    origin: String,
    url: String,
}

fn main() {
    let url = Url::parse("http://httpbin.org/post").unwrap();
    println!("{:?}",
             RestClient::new(url)
                        .post_json_as::<Response>()
        );

    let url = Url::parse("http://httpbin.org/get").unwrap();
    println!("{:?}",
             RestClient::new(url)
                        .get_json_as::<Response>()
        );
}
